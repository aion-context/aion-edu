"""Least squares: the best-fit line y = c + d·x via the normal equations.

For points (x_i, y_i) and A with rows [1, x_i], the best fit solves the normal
equations AᵀA [c, d]ᵀ = Aᵀy. Closed form for a line:
    det = n·Sxx - Sx²
    c = (Sy·Sxx - Sx·Sxy) / det
    d = (n·Sxy - Sx·Sy) / det
Fix fit_line() so the tests pass.
"""


def fit_line(points):
    # STUB: returns a flat line at 0. Replace with the normal-equations solution.
    return (0.0, 0.0)


def residuals(points, c, d):
    return [y - (c + d * x) for (x, y) in points]
