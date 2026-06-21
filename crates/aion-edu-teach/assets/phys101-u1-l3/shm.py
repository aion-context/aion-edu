"""Simple harmonic motion — STARTER.

Everything that oscillates about a stable equilibrium is, for small departures, a
harmonic oscillator: x(t) = A cos(ω t + φ). A spring has ω = √(k/m); a small-angle
pendulum has ω = √(g/L) (so its period is 2π√(L/g), matching lesson 1). Fix shm_x.
"""
import math


def omega_spring(k, m):
    return math.sqrt(k / m)


def omega_pendulum(g, L):
    return math.sqrt(g / L)


def shm_x(t, A, omega, phi):
    # x(t) = A cos(ω t + φ).   BUG: uses sin and drops the phase φ.
    return A * math.sin(omega * t)


def period(omega):
    return 2 * math.pi / omega
