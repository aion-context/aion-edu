"""Mastery check for math110-u1-l3 — least squares line fit."""
import math
from lstsq import fit_line, residuals


def test_exact_line_recovered():
    pts = [(0, 1), (1, 3), (2, 5)]        # y = 1 + 2x exactly
    c, d = fit_line(pts)
    assert math.isclose(c, 1.0, abs_tol=1e-9) and math.isclose(d, 2.0, abs_tol=1e-9)


def test_residual_orthogonal_to_columns():
    pts = [(0, 0), (1, 1), (2, 2), (3, 2)]   # not collinear
    c, d = fit_line(pts)
    r = residuals(pts, c, d)
    # residual orthogonal to the columns 1 and x: sum(r)=0 and sum(x_i r_i)=0
    assert math.isclose(sum(r), 0.0, abs_tol=1e-9)
    assert math.isclose(sum(x * ri for (x, _), ri in zip(pts, r)), 0.0, abs_tol=1e-9)


def test_horizontal_line_is_the_mean():
    pts = [(0, 2), (1, 2), (2, 2)]
    c, d = fit_line(pts)
    assert math.isclose(c, 2.0, abs_tol=1e-9) and math.isclose(d, 0.0, abs_tol=1e-9)
