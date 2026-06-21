"""Mastery check for math110-u1-l2 — projection onto a line."""
import math
from proj import project, error, is_orthogonal


def test_project_onto_axis():
    assert project([1, 0], [2, 3]) == [2.0, 0.0]


def test_projection_formula():
    a, b = [1, 1], [3, 1]              # (a·b)/(a·a) = 4/2 = 2 -> p = (2,2)
    p = project(a, b)
    assert math.isclose(p[0], 2.0) and math.isclose(p[1], 2.0)


def test_error_is_orthogonal_to_a():
    a, b = [1, 1], [3, 1]
    assert is_orthogonal(a, error(a, b))


def test_projection_is_idempotent():
    a, b = [2, 1], [5, 0]
    p = project(a, b)
    p2 = project(a, p)                 # projecting again changes nothing
    assert all(math.isclose(p[i], p2[i]) for i in range(2))
