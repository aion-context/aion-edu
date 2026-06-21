#!/usr/bin/env python3
"""Generate .srt captions keyed to the narration clips.

Splits each beat/segment's narration into readable cues and distributes the
clip's measured duration across them by character length. Writes:
  captions/act1.srt           (for recordings/act1.mp4)
  captions/act2.srt           (for recordings/act2.mp4)
  captions/aion-edu-demo.srt  (full film; Act II offset by act1.mp4's length)
Run from the repo root. stdlib + ffprobe only.
"""
import pathlib
import re
import subprocess

# Texts must match narration/ (see gen-narration.py). "…" pause-markers dropped.
ACT1 = [
    ("A seal, pressed in wax, once carried the full weight of a person's word. Unbroken, it meant: this is "
     "true — and I stand behind it. aion-edu is a university built on that same promise. Only now, the seal "
     "is made of mathematics."),
    ("A diploma is a claim about you. But a claim is only as strong as the trust behind it. Today that trust "
     "lives in a database you cannot see, guarded by an institution you have to call. The proof is always "
     "somewhere else — never with you."),
    ("aion-edu changes where the proof lives. Every rubric, every credential, every act of accreditation is "
     "signed, and chained, and sealed, on aion-context: a ledger that cannot be quietly rewritten. Alter a "
     "single byte, and the seal breaks. The proof travels with the document — and answers to no one."),
    ("And who teaches here? The people who built the field. Each professor is a faithful reconstruction of a "
     "master — Lamport on time, Turing on the limits of the machine, Noether on symmetry, Shannon on "
     "information itself. They teach you live. They hold the line. And they grade you against a rubric not "
     "even they can change."),
    ("It is one unbroken chain. You learn — face to face with a master. You earn — a credential, sealed to "
     "the exact standard you met. Institutions federate, recognizing one another across a shared register of "
     "trust. And anyone, anywhere, can verify — your diploma, proven offline, forgery refused."),
    ("And aion-edu is not a rival to the university — it is a new layer the university can adopt. The "
     "institution brings its curriculum and its standards. aion-edu brings faculty who teach them live, at "
     "any scale. And aion-context seals every credential to that exact standard — one shared record of "
     "accuracy."),
    ("For the institution, that means reaching the world without ever diluting its degree — issuing "
     "credentials no one can forge. And for the student, it means proof that is, at last, provably their "
     "own: portable, permanent, and verifiable anywhere."),
    ("This is a university where your achievement is yours — provable, portable, permanent. Sealed in proof, "
     "not in paperwork. Welcome to aion-edu."),
]
ACT1_FILES = [f"narration/beat-{i}.mp3" for i in range(8)]

ACT2 = [
    "We begin as any student does — with a name, and an ambition. The moment you enroll, aion-edu mints you a cryptographic identity, a key that is yours alone. And it lays out your path: the exact lessons, and the masters who'll teach them.",
    "Press begin, and class is in session. This is not a video. The professor reasons with you, in their own voice — and then they ask. You answer in your own words. There are no multiple-choice shortcuts here; mastery is something you demonstrate.",
    "When you've met every outcome, the professor records mastery — and a credential is sealed. Signed by the professor. Bound to the very rubric you were graded against. Untamperable from the moment it is struck.",
    "It lands in your transcript — a wallet of everything you have proven. Each credential carries its own proof, ready to travel.",
    "Hand it to another institution, and watch what happens. Accepted — because they recognize the university that vouched for you. And to one that doesn't? Rejected — instantly, and offline. The same credential. The trust turns entirely on recognition.",
    "Download the diploma, and you hold the whole proof in a single file. Anyone — with nothing but this file and a verifier — can confirm it is genuine. Change one grade, and it is refused. The seal cannot be faked.",
    "Behind it all is the federation, where institutions establish trust as deliberately as they grant it. They recognize one another. They co-accredit joint degrees, requiring signatures from both. They can delegate trust, and they can withdraw it. And they can checkpoint the entire state, signed, for any dispute.",
    "A university that teaches like the greatest minds in history — and credentials like cryptography. aion-edu. Sealed in proof.",
]
ACT2_FILES = ["01-enroll", "02-lesson", "03-credential", "04-wallet", "05-present", "06-verify", "07-federation", "08-close"]

MAXLEN = 84  # max chars per cue


def dur(path):
    return float(subprocess.check_output(["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0", path]))


def chunks(text):
    out = []
    for s in re.split(r"(?<=[.?!])\s+", text.strip()):
        s = s.strip()
        if not s:
            continue
        if len(s) <= MAXLEN:
            out.append(s)
            continue
        cur = ""
        for tok in re.split(r"(\s+—\s+|,\s+)", s):
            if len(cur) + len(tok) <= MAXLEN:
                cur += tok
            else:
                if cur.strip():
                    out.append(cur.strip())
                cur = tok
        if cur.strip():
            out.append(cur.strip())
    return out


def wrap2(s, width=42):
    if len(s) <= width:
        return s
    mid = len(s) // 2
    for d in range(len(s)):
        for i in (mid - d, mid + d):
            if 0 < i < len(s) and s[i] == " ":
                return s[:i].strip() + "\n" + s[i + 1:].strip()
    return s


def ts(t):
    h = int(t // 3600); m = int(t % 3600 // 60); sec = int(t % 60); ms = int(round((t - int(t)) * 1000))
    if ms == 1000:
        sec += 1; ms = 0
    return f"{h:02d}:{m:02d}:{sec:02d},{ms:03d}"


def cues_for(segs, offset):
    cues = []
    t = offset
    for text, d in segs:
        ch = chunks(text)
        total = sum(len(c) for c in ch) or 1
        st, end = t, t + d
        for c in ch:
            cd = max(0.9, d * len(c) / total)
            e = min(st + cd, end)            # never spill past the segment boundary
            if e - st >= 0.25:
                cues.append((st, e, wrap2(c)))
            st = e
        t = end
    return cues


def write_srt(path, cues):
    out = []
    for i, (s, e, txt) in enumerate(cues, 1):
        out += [str(i), f"{ts(s)} --> {ts(e)}", txt, ""]
    pathlib.Path(path).write_text("\n".join(out))
    print(f"  {path}  ({len(cues)} cues, ends {ts(cues[-1][1])})")


def main():
    a1 = [(ACT1[i], dur(ACT1_FILES[i])) for i in range(8)]
    a2 = [(ACT2[i], dur(f"narration/act2/{ACT2_FILES[i]}.mp3")) for i in range(8)]
    pathlib.Path("captions").mkdir(exist_ok=True)
    write_srt("captions/act1.srt", cues_for(a1, 0))
    write_srt("captions/act2.srt", cues_for(a2, 0))
    act1_len = dur("recordings/act1.mp4") if pathlib.Path("recordings/act1.mp4").exists() else sum(d for _, d in a1)
    write_srt("captions/aion-edu-demo.srt", cues_for(a1, 0) + cues_for(a2, act1_len))
    print(f"  (Act II offset by act1 = {act1_len:.2f}s)")


if __name__ == "__main__":
    main()
