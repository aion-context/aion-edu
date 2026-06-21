"""Mastery check for math150-u1-l2 — parity invariants."""
from parity import heads, same_parity, reachable


def test_heads():
    assert heads([1, 0, 1, 1, 0]) == 3


def test_same_parity():
    assert same_parity([1, 1, 0, 0], [0, 0, 1, 1]) is True   # 2 and 2 — both even
    assert same_parity([1, 0, 0, 0], [1, 1, 1, 0]) is True   # 1 and 3 — both odd
    assert same_parity([1, 0, 0, 0], [1, 1, 0, 0]) is False  # 1 vs 2


def test_reachable_requires_same_parity():
    assert reachable([1, 0, 1, 0], [0, 1, 0, 1]) is True     # 2 and 2 — reachable
    assert reachable([1, 0, 0, 0], [1, 1, 0, 0]) is False    # 1 vs 2 — not reachable


def test_reachable_requires_same_length():
    assert reachable([1, 0], [1, 0, 0]) is False
