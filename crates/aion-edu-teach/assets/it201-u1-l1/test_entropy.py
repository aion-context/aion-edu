"""Mastery check for it201-u1-l1 — Shannon entropy."""
import math
from entropy import entropy


def test_fair_coin_is_one_bit():
    assert math.isclose(entropy([0.5, 0.5]), 1.0)


def test_certain_outcome_is_zero():
    assert math.isclose(entropy([1.0, 0.0]), 0.0)
    assert math.isclose(entropy([0.0, 1.0, 0.0]), 0.0)


def test_uniform_is_log2_n():
    assert math.isclose(entropy([0.25, 0.25, 0.25, 0.25]), 2.0)   # log2(4)
    assert math.isclose(entropy([1 / 8] * 8), 3.0)                 # log2(8)


def test_skips_zero_terms():
    assert math.isclose(entropy([0.5, 0.5, 0.0]), 1.0)
