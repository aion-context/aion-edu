"""Kepler's third law from a simulation — STARTER.

In units where G = 1, a small body in a circular orbit of radius r about a mass M
has speed v = sqrt(M/r) and period T = 2*pi*r / v. Kepler's third law: T²/r³ is
the SAME constant (4*pi²/M) for every orbit around M. Fix circular_period and
period_ratio.
"""
import math


def circular_speed(M, r):
    return math.sqrt(M / r)


def circular_period(M, r):
    # T = 2*pi*r / v.   BUG: drops the 2*pi factor.
    return r / circular_speed(M, r)


def kepler_constant(M, r):
    T = circular_period(M, r)
    return T * T / (r ** 3)


def period_ratio(M, r1, r2):
    # T1/T2 = (r1/r2)^(3/2) by Kepler's law.   BUG: returns r1/r2.
    return r1 / r2
