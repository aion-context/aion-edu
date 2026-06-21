"""Linear algebra — the column picture. STARTER (incomplete on purpose).

Fix three things so `pytest -q test_linalg.py` passes:
  - columns(A): return the COLUMNS of A (the starter wrongly returns the rows).
  - matvec(A, x): the linear combination  x[0]*col0 + x[1]*col1 + ...
  - solvable_2x2(A, b): True iff b lies in the column space of A.
A is a list of rows (list of lists); vectors are lists.
"""


def columns(A):
    # BUG: returns the rows, not the columns.
    return [row[:] for row in A]


def matvec(A, x):
    # Numerically a correct product, but built the row way. Rebuild it as a
    # combination of columns() so the column picture is the unit of thought.
    return [sum(A[i][j] * x[j] for j in range(len(x))) for i in range(len(A))]


def solvable_2x2(A, b):
    # BUG: claims every system is solvable. Dependent columns make that false.
    return True
