"""Mastery check for math310-u1-l1 — the group axioms."""
from group import identity, is_group


def add_mod(n):
    return lambda a, b: (a + b) % n


def mul_mod(n):
    return lambda a, b: (a * b) % n


def test_zn_addition_is_a_group():
    for n in [1, 2, 5, 6]:
        elems = list(range(n))
        assert is_group(elems, add_mod(n))
        assert identity(elems, add_mod(n)) == 0


def test_identity_must_be_found_not_first():
    # 0 is the identity but is listed last — identity() must search, not assume
    elems = [1, 2, 3, 4, 0]
    assert identity(elems, add_mod(5)) == 0
    assert is_group(elems, add_mod(5))


def test_mul_mod4_with_zero_is_not_a_group():
    # 0 has no multiplicative inverse -> not a group
    assert is_group([0, 1, 2, 3], mul_mod(4)) is False


def test_units_mod_5_are_a_group():
    elems = [1, 2, 3, 4]
    assert is_group(elems, mul_mod(5))
    assert identity(elems, mul_mod(5)) == 1
