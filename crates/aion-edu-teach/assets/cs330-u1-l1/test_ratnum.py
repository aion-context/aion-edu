"""Mastery check for cs330-u1-l1 — the rational-number ADT and its rep invariant."""
from math import isclose, gcd
from ratnum import make, add, mul, value, check_rep, numer, denom


def _inv(r):
    n, d = numer(r), denom(r)
    return d > 0 and gcd(abs(n), d) == 1


def test_make_reduces_to_lowest_terms():
    assert make(2, 4) == (1, 2)
    assert make(6, 3) == (2, 1)
    assert make(0, 5) == (0, 1)
    assert _inv(make(2, 4)) and _inv(make(100, 80))


def test_make_normalizes_sign_to_numerator():
    assert make(1, -2) == (-1, 2)       # denominator must be positive
    assert make(-3, -6) == (1, 2)
    assert _inv(make(1, -2))


def test_add_maintains_invariant_and_value():
    s = add(make(1, 2), make(1, 3))
    assert s == (5, 6) and _inv(s)
    assert isclose(value(s), 1 / 2 + 1 / 3)
    t = add(make(1, 6), make(1, 6))     # 1/6 + 1/6 = 1/3, must reduce
    assert t == (1, 3) and _inv(t)


def test_mul_maintains_invariant():
    p = mul(make(2, 3), make(3, 4))     # 6/12 -> 1/2
    assert p == (1, 2) and _inv(p)


def test_check_rep_detects_violations():
    assert check_rep((1, 2)) is True
    assert check_rep((0, 1)) is True
    assert check_rep((2, 4)) is False    # not reduced
    assert check_rep((1, -2)) is False   # negative denominator
