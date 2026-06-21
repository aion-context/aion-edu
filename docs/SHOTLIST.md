# aion-edu — Act II Shot List (click-by-click)

Recording board for the live walkthrough, timed to the Brian narration in
`narration/act2/`. Total VO ≈ **111.5 s**. One protagonist (**alice**), one lesson
(**Lynch · cs440-u1-l1**, Byzantine Agreement). The whole thing is one continuous
alice journey; only the live lesson (seg 02–03) is captured long and trimmed.

---

## Pre-production (run ONCE before the take)

```bash
cd /home/ops/Project/aion-edu
export ANTHROPIC_API_KEY=…              # needed for the live lesson (seg 02)
rm -rf aion-edu-data
./target/debug/aion-edu seal
# federation so 'Present' shows ACCEPTED later (partner recognizes us; Lynch delegated)
./target/debug/aion-edu federate recognize partner-u aion-edu
./target/debug/aion-edu federate delegate  lynch --by aion-edu
# server (leave running, serves narration/ from this dir too)
./target/debug/aion-edu serve --port 8080
```

Do **NOT** pre-mint alice's credential — she earns it on camera (seg 02–03).
Keep a **terminal** open in this directory (for seg 06) and the browser at
`http://127.0.0.1:8080/learn`.

**Recorder setup:** 1920×1080, browser zoom 100–110%, hide bookmarks/extensions.
Import the eight `narration/act2/*.mp3` as a **reference audio track** so you can
hit the marks; mute it on final export (the clean clips are the real track).

Timecodes below are *within each segment* (0.0 = that clip's first word).

---

## 01 · enroll — 15.6 s · page `/learn`
> *"We begin as any student does — with a name, and an ambition. The moment you
> enroll, aion-edu mints you a cryptographic identity, a key that is yours alone.
> And it lays out your path: the exact lessons, and the masters who'll teach them."*

**Open on:** the classroom, empty enroll form in the sidebar.
- **0.0–3.5s** — on *"a name, and an ambition"*: type `alice` in **Your name**,
  then `cs440-u1-l1` in **Target**. (Type at a calm pace; finish by ~3.5s.)
- **~5.5s** — on *"The moment you enroll"*: click **Enroll**.
- **6–11s** — on *"a cryptographic identity, a key that is yours alone"*: the gold
  **identity card** has appeared — slow-zoom / hold on `identity key 28…`.
- **11–15.6s** — on *"lays out your path… the masters"*: pan to the **Your path**
  panel; the lesson row shows **▶ Byzantine Agreement · Prof. lynch**.

## 02 · lesson — 15.7 s · page `/learn` (CAPTURE LONG, TRIM)
> *"Press begin, and class is in session. This is not a video. The professor
> reasons with you, in their own voice — and then they ask. You answer in your own
> words. There are no multiple-choice shortcuts here; mastery is something you demonstrate."*

**This is the one captured-and-trimmed shot.** Click **Begin lesson ▸**, then let the
real lesson run (~2 min). Capture everything; in the editor pull a **~16 s slice**:
- **0.0–4s** — a **Professor** message bubble visible (the teaching voice).
- **~5–7s** — a **Professor → you** *question* bubble appears + the reply bar opens.
- **7–15.7s** — alice **types an answer** into the reply bar and hits **Send**;
  the **YOU** bubble appears. (Choose a take where the answer is in plain words.)

> Tip: a `pytest` tool line can flash here too — but save the *pass* for seg 03.

## 03 · credential — 13.0 s · page `/learn` (same capture, trim)
> *"When you've met every outcome, the professor records mastery — and a credential
> is sealed. Signed by the professor. Bound to the very rubric you were graded
> against. Untamperable from the moment it is struck."*

From the same lesson capture, take the **ending** slice:
- **0.0–5s** — on *"records mastery"*: a green **tool · pass** line (`pytest … rc=0`)
  and the professor's closing message.
- **5–13s** — on *"a credential is sealed"*: the green **◆ Credential issued.** event
  lands in the feed — hold on `file_id …` and `binding verified`.

## 04 · wallet — 8.3 s · page `/learn`
> *"It lands in your transcript — a wallet of everything you have proven. Each
> credential carries its own proof, ready to travel."*

The lesson is done; the **Transcript · credential wallet** panel now has a row.
- **0.0–4s** — on *"lands in your transcript"*: pan to the wallet; the row reads
  **◆ cs440-u1-l1 · Prof. lynch · #… · rubric-bound**.
- **4–8.3s** — on *"carries its own proof, ready to travel"*: hover the row so the
  **⤓ Download diploma** / **✓ Verify offline** controls are visible (don't click yet).

## 05 · present — 16.7 s · page `/learn` (wallet row)
> *"Hand it to another institution, and watch what happens. Accepted — because they
> recognize the university that vouched for you. And to one that doesn't? Rejected —
> instantly, and offline. The same credential. The trust turns entirely on recognition."*

In the wallet row's **present to** control (defaults to `partner-u`):
- **~2s** — on *"watch what happens"*: click **Present ▸**.
- **3–6s** — on *"Accepted"*: the verdict shows **✓ ACCEPTED** (`recognized ✓`) — hold.
- **~8s** — change the partner field to `stranger-u`.
- **9–12s** — on *"Rejected"*: click **Present ▸** → **✗ REJECTED** (`recognized ✗`).
- **12–16.7s** — on *"turns entirely on recognition"*: hold on the two opposite
  verdicts (cut between them if your editor allows).

## 06 · verify — 14.4 s · wallet → TERMINAL
> *"Download the diploma, and you hold the whole proof in a single file. Anyone —
> with nothing but this file and a verifier — can confirm it is genuine. Change one
> grade… and it is refused. The seal cannot be faked."*

- **0.0–3s** — on *"Download the diploma"*: click **⤓ Download diploma** (saves
  `diploma-alice-cs440-u1-l1.json`). Optional: also click **✓ Verify offline** → AUTHENTIC.
- **~3s** — cut to the **terminal**. Run (genuine):
  ```bash
  ./target/debug/aion-edu verify-diploma ~/Downloads/diploma-alice-cs440-u1-l1.json
  ```
- **4–8s** — on *"confirm it is genuine"*: the output shows **=> AUTHENTIC**.
- **~9s** — on *"Change one grade…"*: forge + re-verify:
  ```bash
  python3 -c "import json;d=json.load(open('$HOME/Downloads/diploma-alice-cs440-u1-l1.json'));d['lesson_id']='cs999-u1-l1';json.dump(d,open('/tmp/forged.json','w'))"
  ./target/debug/aion-edu verify-diploma /tmp/forged.json
  ```
- **10–14.4s** — on *"it is refused… cannot be faked"*: output shows **=> NOT AUTHENTIC**
  (`claims match: false`). Hold on the red verdict.

> Pre-stage the two terminal commands in history (↑) so you only press Enter on cue.

## 07 · federation — 19.6 s · page `/federate`
> *"Behind it all is the federation, where institutions establish trust as
> deliberately as they grant it. They recognize one another. They co-accredit joint
> degrees, requiring signatures from both. They can delegate trust, and they can
> withdraw it. And they can checkpoint the entire state, signed, for any dispute."*

Navigate to **/federate** (the state already shows partner-u recognition + Lynch
delegation from pre-production). Perform live, on cue:
- **0.0–4s** — on *"the federation"*: land on the console; **Federation state** shows
  the recognition row (✓).
- **~5s** — on *"recognize one another"*: brief hold on the Recognitions table.
- **6–10s** — on *"co-accredit joint degrees… signatures from both"*: in **Layer 2**,
  click **Declare** (`jd-systems`, 2-of-2), then **Endorse** as `aion-edu`, change
  *by* to `partner-u`, **Endorse** again → state shows **jd-systems 2/2 ✓**.
- **11–14s** — on *"delegate… and withdraw it"*: gesture to **Layer 4** (Lynch already
  delegated); optionally click **Revoke** then re-**Delegate** to show the control.
- **14–19.6s** — on *"checkpoint the entire state, signed"*: click **Snapshot** →
  the Snapshots table shows **#1 · match · aion-edu**. Hold.

## 08 · close — 8.2 s · the seal
> *"A university that teaches like the greatest minds in history — and credentials
> like cryptography. aion-edu. Sealed in proof."*

- **0.0–5s** — cut to the **landing** (`/`): the Aion Seal and `aion·edu` wordmark
  (or trigger the hero so the seal is lit). Slow push-in.
- **5–8.2s** — on *"Sealed in proof."*: settle on the seal; fade.

---

## Assembly notes
- **Order on the timeline:** ACT I film (the landing, ~95 s, screen-capture with its
  own auto-synced audio) → hard cut → ACT II segments 01–08 in order.
- **Lesson trim (02–03):** the only non-realtime part — capture the full ~2-min
  lesson once, then cut the question/answer slice (02) and the credential slice (03).
- **Audio:** drop each `narration/act2/NN-*.mp3` on the VO track back-to-back; nudge
  the video cuts to the durations in this doc. Mute the in-browser sounds.
- **Polish:** a soft low music bed (-22 dB), 6–8 frame cross-dissolves between
  segments, and captions keyed to each segment if you want accessibility.
- **Safety:** re-time is forgiving — VO is the spine; if an action runs long, the
  next clip simply starts a touch later. Keep the *verdict reveals* (present, verify)
  exactly on the "Accepted/Rejected/refused" words; those are the money frames.
