"""Mastery check for astro101-u1-l1 — Kepler's third law."""
import math
from kepler import circular_period, kepler_constant, period_ratio


def test_period_formula():
    M, r = 1.0, 4.0
    v = math.sqrt(M / r)
    assert math.isclose(circular_period(M, r), 2 * math.pi * r / v)


def test_constant_independent_of_radius():
    M = 1.0
    c1, c2, c3 = (kepler_constant(M, x) for x in (1.0, 4.0, 9.0))
    assert math.isclose(c1, c2) and math.isclose(c2, c3)   # T²/r³ is constant


def test_constant_value():
    # with G = 1, T²/r³ = 4*pi²/M
    assert math.isclose(kepler_constant(1.0, 2.5), 4 * math.pi * math.pi)


def test_three_halves_power_law():
    # quadrupling the radius multiplies the period by 4^1.5 = 8
    assert math.isclose(period_ratio(1.0, 4.0, 1.0), 8.0)
