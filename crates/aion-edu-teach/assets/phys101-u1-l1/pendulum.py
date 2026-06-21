"""Simple pendulum — STARTER (incomplete on purpose).

Fix three things so `pytest -q test_pendulum.py` passes:
  - period(L, g):     2*pi*sqrt(L/g)   (the starter has the wrong dimensional form)
  - predict_ratio(...): ratio of periods from the scaling law alone
  - simulate_period(L, g, theta0): integrate theta'' = -(g/L) sin(theta) and
    measure the period (the starter does nothing).
Note there is no `mass` argument anywhere — dimensional analysis says it can't matter.
"""
import math


def period(L, g):
    # BUG: wrong dimensional form (this isn't even a time).
    return 2 * math.pi * math.sqrt(L * g)


def predict_ratio(L1, g1, L2, g2):
    # BUG: not the scaling law.
    return (L1 * g1) / (L2 * g2)


def simulate_period(L, g, theta0):
    # TODO: integrate the pendulum and return its measured period.
    return 0.0
