"""Mastery check for phys101-u1-l1 — the pendulum from units alone."""
import math
from pendulum import period, predict_ratio, simulate_period


def test_period_formula():
    assert math.isclose(period(1.0, 9.8), 2 * math.pi * math.sqrt(1.0 / 9.8), rel_tol=1e-9)


def test_predict_ratio_is_scaling_law():
    # T1/T2 = sqrt((L1/g1)/(L2/g2)); quadrupling L doubles T
    assert math.isclose(predict_ratio(4.0, 9.8, 1.0, 9.8), 2.0, rel_tol=1e-9)


def test_simulation_matches_formula_small_angle():
    assert math.isclose(simulate_period(1.0, 9.8, 0.1), period(1.0, 9.8), rel_tol=0.03)


def test_amplitude_independence_small_angle():
    ta = simulate_period(1.0, 9.8, 0.05)
    tb = simulate_period(1.0, 9.8, 0.10)
    assert math.isclose(ta, tb, rel_tol=0.02)


def test_length_scaling_in_simulation():
    # quadruple the length -> double the period (sqrt(L) scaling)
    assert math.isclose(simulate_period(4.0, 9.8, 0.1),
                        2 * simulate_period(1.0, 9.8, 0.1), rel_tol=0.03)
