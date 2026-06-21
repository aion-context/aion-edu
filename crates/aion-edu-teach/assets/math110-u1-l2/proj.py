"""Projection onto a line — STARTER (the projection scalar is missing).

Project b onto the line through a:  p = (a·b / a·a) * a.  The error e = b - p is
orthogonal to a.  Fix project() so the tests pass.
"""


def dot(u, v):
    return sum(u[i] * v[i] for i in range(len(u)))


def project(a, b):
    # p = (a·b / a·a) * a.   STUB: returns the zero vector.
    return [0.0 for _ in a]


def error(a, b):
    p = project(a, b)
    return [b[i] - p[i] for i in range(len(b))]


def is_orthogonal(u, v, tol=1e-9):
    return abs(dot(u, v)) < tol
