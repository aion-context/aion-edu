"""Mastery check for phys101-u1-l3 — the harmonic oscillator everywhere."""
import math
from shm import omega_spring, omega_pendulum, shm_x, period


def test_omega_spring():
    assert math.isclose(omega_spring(9.0, 1.0), 3.0)


def test_pendulum_period_matches_lesson1():
    g, L = 9.8, 1.0
    assert math.isclose(period(omega_pendulum(g, L)),
                        2 * math.pi * math.sqrt(L / g), rel_tol=1e-9)


def test_shm_initial_conditions():
    A, omega, phi = 2.0, 3.0, 0.0
    assert math.isclose(shm_x(0, A, omega, phi), 2.0)                       # cos(0)=1
    assert math.isclose(shm_x(math.pi / (2 * omega), A, omega, phi), 0.0, abs_tol=1e-9)


def test_shm_matches_numerical_integration():
    # x'' = -omega^2 x, x(0)=A, x'(0)=0  ->  x(t)=A cos(omega t)
    A, omega, dt = 1.0, 2.0, 1e-4
    x, v, t = A, 0.0, 0.0
    while t < 1.0:
        v += -omega * omega * x * dt
        x += v * dt
        t += dt
    assert math.isclose(x, shm_x(1.0, A, omega, 0.0), rel_tol=0.02)
