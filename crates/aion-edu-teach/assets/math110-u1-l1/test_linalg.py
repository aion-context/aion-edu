"""Mastery check for math110-u1-l1 — the column picture of Ax."""
from linalg import columns, matvec, solvable_2x2


def test_columns_returns_columns():
    assert columns([[1, 2, 3], [4, 5, 6]]) == [[1, 4], [2, 5], [3, 6]]


def test_matvec_numeric():
    assert matvec([[1, 2], [3, 4]], [5, 6]) == [17, 39]


def test_matvec_is_a_combination_of_columns():
    A, x = [[1, 2], [3, 4]], [5, 6]
    cols = columns(A)                     # [[1,3],[2,4]]
    combo = [x[0] * cols[0][i] + x[1] * cols[1][i] for i in range(2)]
    assert matvec(A, x) == combo          # fails if columns() returns rows


def test_solvable_independent_columns():
    assert solvable_2x2([[1, 0], [0, 1]], [3, 7]) is True


def test_solvable_dependent_b_on_the_line():
    # columns (1,2) and (2,4) are dependent; b=(3,6) lies on their line
    assert solvable_2x2([[1, 2], [2, 4]], [3, 6]) is True


def test_unsolvable_dependent_b_off_the_line():
    # b=(1,3) is NOT on the line spanned by (1,2)
    assert solvable_2x2([[1, 2], [2, 4]], [1, 3]) is False
