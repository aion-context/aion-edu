"""Mastery check for math110-u2-l1 — eigenvalues of a 2x2 matrix."""
import math
from eig import trace, det, eigenvalues_2x2, matvec


def test_trace_and_det():
    A = [[2, 0], [0, 3]]
    assert trace(A) == 5 and det(A) == 6


def test_diagonal_eigenvalues():
    ev = sorted(eigenvalues_2x2([[2, 0], [0, 3]]))
    assert math.isclose(ev[0], 2.0) and math.isclose(ev[1], 3.0)


def test_characteristic_equation():
    # [[2,1],[1,2]] has eigenvalues 1 and 3
    ev = sorted(eigenvalues_2x2([[2, 1], [1, 2]]))
    assert math.isclose(ev[0], 1.0) and math.isclose(ev[1], 3.0)


def test_eigenpair_satisfies_av_equals_lambda_v():
    A, lam, v = [[2, 1], [1, 2]], 3.0, [1, 1]   # A v = (3,3) = 3 v
    av = matvec(A, v)
    assert math.isclose(av[0], lam * v[0]) and math.isclose(av[1], lam * v[1])
