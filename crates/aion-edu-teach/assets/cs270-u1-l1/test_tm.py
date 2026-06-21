"""Mastery check for cs270-u1-l1 — a Turing machine that increments."""
from tm import to_tape, run, tape_str, INCREMENT_TABLE


def increment(s):
    tape, _head, _state = run(to_tape(s), 0, "GOTO_END", INCREMENT_TABLE)
    return tape_str(tape)


def test_simple_increment():
    assert increment("0") == "1"
    assert increment("1") == "10"
    assert increment("1011") == "1100"   # 11 -> 12


def test_carry_propagation():
    assert increment("111") == "1000"           # 7 -> 8 (tape grows left)
    assert increment("101111") == "110000"       # 47 -> 48


def test_machine_halts():
    _t, _h, state = run(to_tape("1011"), 0, "GOTO_END", INCREMENT_TABLE)
    assert state == "HALT"
