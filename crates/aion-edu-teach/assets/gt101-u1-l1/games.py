"""Zero-sum games and the saddle point — STARTER.

A is the row player's payoff matrix (the column player receives -A). The row
player picks the row to MAXIMIZE the row-minimum (maximin); the column player picks
the column to MINIMIZE the column-maximum (minimax). If maximin == minimax, that
value is a SADDLE POINT (a pure-strategy solution). For a 2x2 with no saddle, the
value is  v = (a00·a11 - a01·a10) / (a00 + a11 - a01 - a10). Fix maximin/minimax.
"""


def maximin(A):
    # max over rows of the row's MINIMUM.  BUG: uses the row's maximum.
    return max(max(row) for row in A)


def minimax(A):
    cols = list(zip(*A))
    # min over columns of the column's MAXIMUM.  BUG: uses the column's minimum.
    return min(min(c) for c in cols)


def has_saddle_point(A):
    return maximin(A) == minimax(A)


def game_value(A):
    if has_saddle_point(A):
        return maximin(A)
    a00, a01 = A[0]
    a10, a11 = A[1]
    return (a00 * a11 - a01 * a10) / (a00 + a11 - a01 - a10)
