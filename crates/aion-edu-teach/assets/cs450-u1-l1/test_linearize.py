"""Mastery check for cs450-u1-l1 — linearizability of a register history."""
from linearize import precedes, legal_register, is_linearizable


def W(v, s, e):
    return ("W", v, s, e)


def R(v, s, e):
    return ("R", v, s, e)


def test_precedes_uses_end_before_start():
    assert precedes(W(1, 0, 5), R(0, 6, 10)) is True       # 5 < 6
    assert precedes(W(1, 0, 10), R(1, 5, 15)) is False     # overlap -> not before
    a, b = W(1, 0, 10), R(1, 5, 15)
    assert precedes(b, a) is False                         # neither precedes


def test_legal_register_replays_writes():
    assert legal_register([W(1, 0, 5), R(1, 5, 6)]) is True
    assert legal_register([W(1, 0, 5), R(0, 5, 6)]) is False    # stale read
    assert legal_register([R(0, 0, 1)]) is True                # initial value
    assert legal_register([W(5, 0, 1), W(7, 1, 2), R(7, 2, 3)]) is True


def test_linearizable_concurrent_history():
    # W(1) overlaps R->1: order it W then R
    assert is_linearizable([W(1, 0, 10), R(1, 5, 15)]) is True
    # one write of 1, two reads of 1 (overlapping/after) -> consistent
    assert is_linearizable([W(1, 0, 5), R(1, 3, 8), R(1, 6, 9)]) is True


def test_non_linearizable_history():
    # read returns 0 although the write of 1 already completed before it began
    assert is_linearizable([W(1, 0, 5), R(0, 6, 10)]) is False
    # read returns 1 after both writes (last is 2) completed -> illegal
    assert is_linearizable([W(1, 0, 2), W(2, 3, 5), R(1, 6, 8)]) is False
