# aion-edu — Master Narration Script

> The voiceover for the whole tool, in two acts. **Act I — The Film** is the
> cinematic landing intro (`/`), eight beats, ~165s. **Act II — The Walkthrough**
> narrates a live screen-capture of the real product, ~3–4 min.
>
> Each beat is timed loosely; the player chains one audio clip per beat and
> advances on `ended`, so exact durations don't have to be hand-tuned.

## Voice direction

- **Persona:** a prestige-documentary narrator — warm, measured, authoritative;
  the gravitas of a master's college, not an ad. Think candlelit library.
- **Pace:** unhurried, ~130–140 wpm. Let the seal lines breathe; tighten on the
  cryptographic lines so they land crisp and certain.
- **Register:** reverent on trust and provenance; precise and clean on mechanism.
- **ElevenLabs settings (suggested):** `eleven_multilingual_v2`,
  stability `0.45`, similarity `0.85`, style `0.15`, speaker-boost on.
  `[beat]` marks a held pause (the generator splits or lets the model breathe).

---

## ACT I — THE FILM  (landing `/`, eight beats)

### BEAT 0 · hero — the strike of the seal  (~12s)
*Direction: slow, weighty. The seal is drawing on screen as this is spoken.*

> A seal, pressed in wax, once carried the full weight of a person's word.
> Unbroken, it meant: *this is true — and I stand behind it.* [beat]
> aion-edu is a university built on that same promise. Only now, the seal is
> made of mathematics.

### BEAT 1 · the problem  (~16s)
*Direction: plain, grounded — name the quiet failure.*

> A diploma is a claim about you. But a claim is only as strong as the trust
> behind it. Today that trust lives in a database you cannot see, guarded by an
> institution you have to call. The proof is always somewhere else — never with you.

### BEAT 2 · aion-context — the trust spine  (~20s)
*Direction: this is the thesis. Confident, exact.*

> aion-edu changes where the proof lives. Every rubric, every credential, every
> act of accreditation is signed — and chained — and sealed, on **aion-context**:
> a ledger that cannot be quietly rewritten. Alter a single byte, and the seal
> breaks. The proof travels *with* the document — and answers to no one.

### BEAT 3 · the faculty  (~18s)
*Direction: warm pride. The professor names ride the on-screen marquee.*

> And who teaches here? The people who built the field. Each professor is a
> faithful reconstruction of a master — Lamport on time, Turing on the limits of
> the machine, Noether on symmetry, Shannon on information itself. They teach you
> live. They hold the line. And they grade you against a rubric not even they can change.

### BEAT 4 · the lifecycle  (~18s)
*Direction: build momentum across the four verbs.*

> It is one unbroken chain. You **learn** — face to face with a master. You
> **earn** — a credential, sealed to the exact standard you met. Institutions
> **federate** — recognizing one another across a shared register of trust. And
> anyone, anywhere, can **verify** — your diploma, proven offline, forgery refused.

### BEAT 5 · the institutional layer  (~16s)
*Direction: the pivot. Warm, expansive — the thesis for the institutions watching.
The film holds on the reframe and the three adoption steps.*

> And aion-edu is not a rival to the university — it is a new layer the university
> can adopt. The institution brings its curriculum and its standards. aion-edu brings
> faculty who teach them live, at any scale. And aion-context seals every credential
> to that exact standard — one shared record of accuracy.

### BEAT 6 · what it unlocks  (~16s)
*Direction: two sides, one proof. The film holds on the dual-empowerment panels.*

> For the institution, that means reaching the world without ever diluting its degree —
> issuing credentials no one can forge. And for the student, it means proof that is, at
> last, provably their own: portable, permanent, and verifiable anywhere.

### BEAT 7 · the call  (~12s)
*Direction: land it. A held pause before the name.*

> This is a university where your achievement is yours — provable, portable,
> permanent. Sealed in proof, not in paperwork. [beat] Welcome to aion-edu.

---

## ACT II — THE WALKTHROUGH  (over a live screen capture, ~3–4 min)

### W1 · enroll  ·  *on-screen: type name + target → Enroll → identity card + path*
> We begin as any student does — with a name, and an ambition. The moment you
> enroll, aion-edu mints you a cryptographic identity, a key that is yours alone.
> And it lays out your path: the exact lessons, and the masters who'll teach them.

### W2 · the lesson  ·  *on-screen: Begin → professor speaks → asks → you type*
> Press begin, and class is in session. This is not a video. The professor reasons
> with you, in their own voice — and then they ask. You answer in your own words.
> There are no multiple-choice shortcuts here; mastery is something you *demonstrate.*

### W3 · the credential  ·  *on-screen: pytest passes → "Credential issued"*
> When you've met every outcome, the professor records mastery — and a credential
> is sealed. Signed by the professor. Bound to the very rubric you were graded
> against. Untamperable from the moment it is struck.

### W4 · the wallet  ·  *on-screen: transcript wallet, rubric-bound badge*
> It lands in your transcript — a wallet of everything you have proven. Each
> credential carries its own proof, ready to travel.

### W5 · present to a partner  ·  *on-screen: Present → ACCEPTED, then stranger → REJECTED*
> Hand it to another institution, and watch what happens. **Accepted** — because
> they recognize the university that vouched for you. And to one that doesn't?
> **Rejected** — instantly, and offline. The same credential. The trust turns
> entirely on recognition.

### W6 · download & verify  ·  *on-screen: Download diploma → CLI verify-diploma → forged → refused*
> Download the diploma, and you hold the whole proof in a single file. Anyone —
> with nothing but this file and a verifier — can confirm it is genuine. Change one
> grade… and it is refused. The seal cannot be faked.

### W7 · the federation  ·  *on-screen: /federate console — recognize, co-accredit, delegate, revoke, snapshot*
> Behind it all is the federation, where institutions establish trust as
> deliberately as they grant it. They recognize one another. They co-accredit joint
> degrees, requiring signatures from both. They can delegate trust, and they can
> withdraw it. And they can checkpoint the entire state, signed, for any dispute.

### CLOSE  ·  *on-screen: the seal*
> A university that teaches like the greatest minds in history — and credentials
> like cryptography. **aion-edu.** Sealed in proof.

---

## Production notes
- Generate one MP3 per **Act I** beat → `narration/beat-0.mp3 … beat-7.mp3`
  plus `narration/manifest.json` (order: hero, problem, spine, faculty, lifecycle, institutions, empower, call).
- The landing **▶ Play intro** auto-detects the manifest: if present it plays the
  voiced film (advancing each beat on audio `ended`); otherwise it falls back to
  the silent timed tour.
- Act II is narrated over a recorded screen capture in an editor; the same
  per-segment clips can be generated with the same tool (`--act 2`).
- Generator: `tools/gen-narration.py` (reads the ElevenLabs key from `ELEVENLABS_API_KEY`
  or `eleven.key`; voice from `ELEVEN_VOICE_ID`).
