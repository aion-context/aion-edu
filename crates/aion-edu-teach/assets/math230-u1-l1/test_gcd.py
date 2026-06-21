"""Mastery check for math230-u1-l1 — Euclidean algorithm & modular inverse."""
from gcd import gcd, ext_gcd, mod_inverse


def test_gcd():
    assert gcd(12, 8) == 4 and gcd(48, 18) == 6 and gcd(17, 5) == 1 and gcd(0, 7) == 7


def test_bezout_identity():
    for a, b in [(12, 8), (48, 18), (17, 5), (240, 46)]:
        g, x, y = ext_gcd(a, b)
        assert g == gcd(a, b)
        assert a * x + b * y == g


def test_modular_inverse():
    assert mod_inverse(3, 7) == 5        # 3*5 = 15 ≡ 1 (mod 7)
    assert mod_inverse(10, 17) == 12     # 10*12 = 120 ≡ 1 (mod 17)
    assert (mod_inverse(3, 7) * 3) % 7 == 1


def test_no_inverse_when_not_coprime():
    assert mod_inverse(4, 8) is None
