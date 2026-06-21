#!/usr/bin/env python3
"""Render candidate pronunciations of "aion-edu" (target: AY-on ED-you) so a human
can audition and pick. Writes narration/test/*.mp3. Reads key from eleven.env."""
import json
import os
import pathlib
import sys
import urllib.request

VOICE = os.environ.get("ELEVEN_VOICE_ID", "nPczCjzI2devNBz1zQrb")  # Brian
PHRASE = "Welcome to aion-edu. A university sealed in proof."
SETTINGS = {"stability": 0.45, "similarity_boost": 0.85, "style": 0.15, "use_speaker_boost": True}


def key():
    for v in ("ELEVENLABS_API_KEY", "ELEVEN_LABS_API_KEY"):
        if os.environ.get(v):
            return os.environ[v].strip()
    for line in pathlib.Path("eleven.env").read_text().splitlines():
        if "API_KEY" in line and "=" in line:
            return line.split("=", 1)[1].strip().strip('"').strip("'")
    sys.exit("no key")


K = key()


def api(method, path, body=None):
    req = urllib.request.Request("https://api.elevenlabs.io" + path, method=method,
                                 data=json.dumps(body).encode() if body else None,
                                 headers={"xi-api-key": K, "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())


def make_dict(name, rules):
    d = api("POST", "/v1/pronunciation-dictionaries/add-from-rules", {"name": name, "rules": rules})
    return {"pronunciation_dictionary_id": d["id"], "version_id": d.get("version_id") or d.get("latest_version_id")}


def tts(out, model, text=PHRASE, locators=None):
    body = {"text": text, "model_id": model, "voice_settings": SETTINGS}
    if locators:
        body["pronunciation_dictionary_locators"] = locators
    req = urllib.request.Request(
        f"https://api.elevenlabs.io/v1/text-to-speech/{VOICE}?output_format=mp3_44100_128",
        data=json.dumps(body).encode(),
        headers={"xi-api-key": K, "Content-Type": "application/json", "Accept": "audio/mpeg"})
    try:
        with urllib.request.urlopen(req, timeout=90) as r:
            pathlib.Path(out).write_bytes(r.read())
        print(f"  wrote {out}")
    except urllib.error.HTTPError as e:
        print(f"  SKIP {out}: HTTP {e.code} {e.read().decode(errors='replace')[:120]}")


def main():
    out = pathlib.Path("narration/test")
    out.mkdir(parents=True, exist_ok=True)
    alias_edyou = make_dict("aion-alias-edyou", [{"type": "alias", "string_to_replace": "aion-edu", "alias": "Aion ED-you"}])
    alias_spaced = make_dict("aion-alias-spaced", [{"type": "alias", "string_to_replace": "aion-edu", "alias": "Aion ed you"}])
    phon = make_dict("aion-phoneme", [{"type": "phoneme", "string_to_replace": "aion-edu", "phoneme": "ˈeɪɒn ˈɛdʒuː", "alphabet": "ipa"}])

    tts("narration/test/A-baseline-multi.mp3", "eleven_multilingual_v2")
    tts("narration/test/B-multi-alias-edyou.mp3", "eleven_multilingual_v2", locators=[alias_edyou])
    tts("narration/test/C-multi-alias-spaced.mp3", "eleven_multilingual_v2", locators=[alias_spaced])
    tts("narration/test/D-turbo2-phoneme.mp3", "eleven_turbo_v2", locators=[phon])
    tts("narration/test/E-v3-alias-edyou.mp3", "eleven_v3", locators=[alias_edyou])
    print("\nAudition narration/test/*.mp3 and tell me the letter that sounds right.")


if __name__ == "__main__":
    main()
