"""Eigenvalues of a 2x2 matrix — STARTER.

Av = λv: an eigenvector v is a direction A only *scales*; λ is the scale factor.
For a 2x2 matrix the eigenvalues solve the characteristic equation
    λ² - (trace)·λ + det = 0      ->  λ = (t ± sqrt(t² - 4d)) / 2
with t = trace, d = det. Fix eigenvalues_2x2.
"""
import math


def trace(A):
    return A[0][0] + A[1][1]


def det(A):
    return A[0][0] * A[1][1] - A[0][1] * A[1][0]


def matvec(A, v):
    return [A[0][0] * v[0] + A[0][1] * v[1], A[1][0] * v[0] + A[1][1] * v[1]]


def eigenvalues_2x2(A):
    # λ = (t ± sqrt(t² - 4d)) / 2.   BUG: returns the trace twice.
    t = trace(A)
    return (t, t)
