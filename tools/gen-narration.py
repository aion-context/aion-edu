#!/usr/bin/env python3
"""Generate the Act I narration (the landing film) with ElevenLabs.

Reads the API key from $ELEVENLABS_API_KEY or ./eleven.key or ~/.config/elevenlabs/key,
and the voice from $ELEVEN_VOICE_ID (or --voice <id>). Writes one MP3 per beat plus a
manifest into ./narration/ — exactly what the landing "▶ Play intro" player expects.

Usage:
    ELEVEN_VOICE_ID=<voice_id> python3 tools/gen-narration.py
    python3 tools/gen-narration.py --voice <voice_id> --model eleven_multilingual_v2
No third-party packages — stdlib urllib only.
"""
import json
import os
import pathlib
import sys
import urllib.request

# Act I — the six beats (order = hero, problem, spine, faculty, lifecycle, call).
# "…" induces a held pause; mirrors docs/NARRATION.md.
BEATS = [
    ("hero",
     "A seal, pressed in wax, once carried the full weight of a person's word. "
     "Unbroken, it meant: this is true — and I stand behind it. … "
     "aion-edu is a university built on that same promise. Only now, the seal is made of mathematics."),
    ("problem",
     "A diploma is a claim about you. But a claim is only as strong as the trust behind it. "
     "Today that trust lives in a database you cannot see, guarded by an institution you have to call. "
     "The proof is always somewhere else — never with you."),
    ("spine",
     "aion-edu changes where the proof lives. Every rubric, every credential, every act of accreditation "
     "is signed, and chained, and sealed, on aion-context: a ledger that cannot be quietly rewritten. "
     "Alter a single byte, and the seal breaks. The proof travels with the document — and answers to no one."),
    ("faculty",
     "And who teaches here? The people who built the field. Each professor is a faithful reconstruction of "
     "a master — Lamport on time, Turing on the limits of the machine, Noether on symmetry, Shannon on "
     "information itself. They teach you live. They hold the line. And they grade you against a rubric not "
     "even they can change."),
    ("lifecycle",
     "It is one unbroken chain. You learn — face to face with a master. You earn — a credential, sealed to "
     "the exact standard you met. Institutions federate, recognizing one another across a shared register of "
     "trust. And anyone, anywhere, can verify — your diploma, proven offline, forgery refused."),
    ("institutions",
     "And aion-edu is not a rival to the university — it is a new layer the university can adopt. "
     "The institution brings its curriculum and its standards. aion-edu brings faculty who teach them "
     "live, at any scale. And aion-context seals every credential to that exact standard — one shared "
     "record of accuracy."),
    ("empower",
     "For the institution, that means reaching the world without ever diluting its degree — issuing "
     "credentials no one can forge. And for the student, it means proof that is, at last, provably their "
     "own: portable, permanent, and verifiable anywhere."),
    ("call",
     "This is a university where your achievement is yours — provable, portable, permanent. "
     "Sealed in proof, not in paperwork. … Welcome to aion-edu."),
]

# Act II — the live-walkthrough VO (narrated over a screen capture). Each segment
# carries an on-screen cue for the editor. Mirrors docs/NARRATION.md.
ACT2 = [
    ("enroll", "type name + target -> Enroll -> identity card + path",
     "We begin as any student does — with a name, and an ambition. The moment you enroll, aion-edu mints "
     "you a cryptographic identity, a key that is yours alone. And it lays out your path: the exact lessons, "
     "and the masters who'll teach them."),
    ("lesson", "Begin -> professor speaks -> asks -> you type your answer",
     "Press begin, and class is in session. This is not a video. The professor reasons with you, in their "
     "own voice — and then they ask. You answer in your own words. There are no multiple-choice shortcuts "
     "here; mastery is something you demonstrate."),
    ("credential", "pytest passes -> 'Credential issued'",
     "When you've met every outcome, the professor records mastery — and a credential is sealed. Signed by "
     "the professor. Bound to the very rubric you were graded against. Untamperable from the moment it is struck."),
    ("wallet", "transcript wallet, rubric-bound badge",
     "It lands in your transcript — a wallet of everything you have proven. Each credential carries its own "
     "proof, ready to travel."),
    ("present", "Present -> ACCEPTED ; then a stranger institution -> REJECTED",
     "Hand it to another institution, and watch what happens. Accepted — because they recognize the "
     "university that vouched for you. And to one that doesn't? Rejected — instantly, and offline. The same "
     "credential. The trust turns entirely on recognition."),
    ("verify", "Download diploma -> CLI verify-diploma -> forge a grade -> refused",
     "Download the diploma, and you hold the whole proof in a single file. Anyone — with nothing but this "
     "file and a verifier — can confirm it is genuine. Change one grade… and it is refused. The seal cannot be faked."),
    ("federation", "/federate console — recognize, co-accredit, delegate, revoke, snapshot",
     "Behind it all is the federation, where institutions establish trust as deliberately as they grant it. "
     "They recognize one another. They co-accredit joint degrees, requiring signatures from both. They can "
     "delegate trust, and they can withdraw it. And they can checkpoint the entire state, signed, for any dispute."),
    ("close", "the seal",
     "A university that teaches like the greatest minds in history — and credentials like cryptography. "
     "aion-edu. Sealed in proof."),
]

SETTINGS = {"stability": 0.45, "similarity_boost": 0.85, "style": 0.15, "use_speaker_boost": True}


def load_key():
    for var in ("ELEVENLABS_API_KEY", "ELEVEN_LABS_API_KEY"):
        if os.environ.get(var):
            return os.environ[var].strip()
    for p in ("eleven.env", "eleven.key", os.path.expanduser("~/.creds/eleven.env"),
              os.path.expanduser("~/.config/elevenlabs/key")):
        if os.path.exists(p):
            txt = pathlib.Path(p).read_text()
            for line in txt.splitlines():
                s = line.strip()
                if s.startswith(("ELEVEN_LABS_API_KEY", "ELEVENLABS_API_KEY")) and "=" in s:
                    return s.split("=", 1)[1].strip().strip('"').strip("'")
            if txt.strip() and "=" not in txt:  # a raw key file
                return txt.strip()
    sys.exit("No ElevenLabs key. Set $ELEVEN_LABS_API_KEY, or write it to ./eleven.env / ./eleven.key")


def arg(flag, default=None):
    return sys.argv[sys.argv.index(flag) + 1] if flag in sys.argv else default


def synth(key, voice, model, text):
    payload = {"text": text, "model_id": model, "voice_settings": SETTINGS}
    pid, pver = os.environ.get("ELEVEN_PRON_ID"), os.environ.get("ELEVEN_PRON_VER")
    if pid and pver:
        payload["pronunciation_dictionary_locators"] = [{"pronunciation_dictionary_id": pid, "version_id": pver}]
    req = urllib.request.Request(
        f"https://api.elevenlabs.io/v1/text-to-speech/{voice}?output_format=mp3_44100_128",
        data=json.dumps(payload).encode(),
        headers={"xi-api-key": key, "Content-Type": "application/json", "Accept": "audio/mpeg"},
    )
    with urllib.request.urlopen(req, timeout=120) as r:
        return r.read()


def gen_act1(key, voice, model):
    out = pathlib.Path("narration")
    out.mkdir(exist_ok=True)
    manifest = {"voice": voice, "model": model, "beats": []}
    total = 0
    for i, (bid, text) in enumerate(BEATS):
        total += len(text)
        print(f"[A1 {i + 1}/{len(BEATS)}] {bid} ({len(text)} chars) …", flush=True)
        try:
            data = synth(key, voice, model, text)
        except urllib.error.HTTPError as e:
            sys.exit(f"  TTS failed for {bid}: HTTP {e.code} {e.read().decode(errors='replace')[:200]}")
        except Exception as e:  # noqa: BLE001
            sys.exit(f"  TTS failed for {bid}: {e}")
        fn = f"beat-{i}.mp3"
        (out / fn).write_bytes(data)
        manifest["beats"].append({"id": bid, "file": fn, "chars": len(text)})
        print(f"  → narration/{fn}  ({len(data):,} bytes)")
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2))
    print(f"Act I → narration/manifest.json  ({total} chars, {len(BEATS)} beats)\n")


def gen_act2(key, voice, model):
    out = pathlib.Path("narration/act2")
    out.mkdir(parents=True, exist_ok=True)
    manifest = {"voice": voice, "model": model, "act": 2, "segments": []}
    total = 0
    for i, (sid, cue, text) in enumerate(ACT2, 1):
        total += len(text)
        print(f"[A2 {i}/{len(ACT2)}] {sid} ({len(text)} chars) …", flush=True)
        try:
            data = synth(key, voice, model, text)
        except urllib.error.HTTPError as e:
            sys.exit(f"  TTS failed for {sid}: HTTP {e.code} {e.read().decode(errors='replace')[:200]}")
        except Exception as e:  # noqa: BLE001
            sys.exit(f"  TTS failed for {sid}: {e}")
        fn = f"{i:02d}-{sid}.mp3"
        (out / fn).write_bytes(data)
        manifest["segments"].append({"id": sid, "file": fn, "cue": cue, "text": text, "chars": len(text)})
        print(f"  → narration/act2/{fn}  ({len(data):,} bytes)")
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2))
    print(f"Act II → narration/act2/manifest.json  ({total} chars, {len(ACT2)} segments)\n")


def main():
    key = load_key()
    voice = os.environ.get("ELEVEN_VOICE_ID") or arg("--voice")
    if not voice:
        sys.exit("No voice. Set $ELEVEN_VOICE_ID or pass --voice <voice_id>")
    model = os.environ.get("ELEVEN_MODEL") or arg("--model", "eleven_multilingual_v2")
    act = arg("--act", "1")
    if act in ("1", "all"):
        gen_act1(key, voice, model)
    if act in ("2", "all"):
        gen_act2(key, voice, model)
    if act not in ("1", "2", "all"):
        sys.exit("--act must be 1, 2, or all")
    print("Done.")


if __name__ == "__main__":
    main()
