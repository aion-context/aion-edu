"""Mastery check for cs250-u1-l1 — integer square root."""
from isqrt import isqrt


def test_perfect_squares():
    assert isqrt(0) == 0 and isqrt(1) == 1 and isqrt(4) == 2
    assert isqrt(9) == 3 and isqrt(144) == 12


def test_between_squares():
    assert isqrt(2) == 1 and isqrt(8) == 2 and isqrt(10) == 3 and isqrt(143) == 11


def test_post_condition_holds():
    for n in [0, 1, 2, 3, 15, 16, 17, 99, 100, 101]:
        a = isqrt(n)
        assert a * a <= n < (a + 1) * (a + 1)
