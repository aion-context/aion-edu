#!/usr/bin/env bash
# aion-edu — one-shot demo / recording setup.  See docs/SHOTLIST.md.
#
# Resets the data dir, seals every rubric, establishes the federation so that
# 'Present' resolves to ACCEPTED (partner-u) and REJECTED (stranger-u), checks the
# narration assets, prints the recording checklist, and starts the server.
#
#   tools/demo-setup.sh                 # set up + start server on :8080
#   PORT=9000 tools/demo-setup.sh       # different port
#   tools/demo-setup.sh --no-serve      # set up only (start the server yourself)
set -euo pipefail

PORT="${PORT:-8080}"
SERVE=1
[ "${1:-}" = "--no-serve" ] && SERVE=0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$HERE"
BIN="./target/debug/aion-edu"

say()  { printf '\033[1;33m▶\033[0m %s\n' "$*"; }
ok()   { printf '\033[1;32m✓\033[0m %s\n' "$*"; }
warn() { printf '\033[1;31m!\033[0m %s\n' "$*"; }

# 1 · binary
if [ ! -x "$BIN" ]; then say "building aion-edu (debug)…"; cargo build -q --bin aion-edu; fi
ok "binary  $BIN"

# 2 · Anthropic key — needed only for the LIVE lesson (segment 02)
if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
  warn "ANTHROPIC_API_KEY not set — the live lesson (seg 02) won't run."
  warn "  export it before recording the lesson; everything else works without it."
else
  ok "ANTHROPIC_API_KEY set — live lesson ready"
fi

# 3 · fresh data + seal
say "resetting aion-edu-data and sealing rubrics…"
rm -rf aion-edu-data
ok "$("$BIN" seal | tail -1)"

# 4 · federation (so seg 05 'Present' has an ACCEPTED and a REJECTED)
say "establishing the federation…"
"$BIN" federate recognize partner-u aion-edu >/dev/null
"$BIN" federate delegate  lynch --by aion-edu >/dev/null
ok "partner-u recognizes aion-edu · Lynch's key delegated  → Present: partner-u ACCEPTED, stranger-u REJECTED"

# 5 · narration assets
A1=$(find narration       -maxdepth 1 -name 'beat-*.mp3' 2>/dev/null | wc -l | tr -d ' ')
A2=$(find narration/act2  -maxdepth 1 -name '*.mp3'      2>/dev/null | wc -l | tr -d ' ')
if [ "$A1" -ge 6 ]; then ok "Act I film narration — $A1 clips (landing ▶ Play intro is voiced)"
else warn "Act I narration missing ($A1/6) — landing plays silent. Run: python3 tools/gen-narration.py --act 1"; fi
if [ "$A2" -ge 8 ]; then ok "Act II walkthrough narration — $A2 clips (reference track ready)"
else warn "Act II narration missing ($A2/8) — run: python3 tools/gen-narration.py --act 2"; fi

# 6 · checklist
cat <<EOF

  ────────────────────────────────────────────────────────────────────
   READY TO RECORD   ·   board: docs/SHOTLIST.md
  ────────────────────────────────────────────────────────────────────
   Landing  (Act I film) ....  http://127.0.0.1:${PORT}/          ▶ Play intro
   Classroom (Act II) .......  http://127.0.0.1:${PORT}/learn
   Federation (seg 07) ......  http://127.0.0.1:${PORT}/federate
   Protagonist ..............  alice   ·   lesson  cs440-u1-l1  (Prof. lynch)
   Present  (seg 05) ........  partner-u → ACCEPTED   ·   stranger-u → REJECTED
   Verify   (seg 06) ........  in ANOTHER terminal, after Download diploma:
        $BIN verify-diploma ~/Downloads/diploma-alice-cs440-u1-l1.json
   Reset & rerun ............  tools/demo-setup.sh
  ────────────────────────────────────────────────────────────────────

EOF

if [ "$SERVE" = 0 ]; then
  ok "setup complete (--no-serve). Start it yourself:  $BIN serve --port ${PORT}"
  exit 0
fi
say "starting server on :${PORT}  (Ctrl-C to stop)…"
echo
exec "$BIN" serve --port "$PORT"
