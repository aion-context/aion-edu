"""Domino tilings & the coloring invariant — STARTER (incomplete on purpose).

A board is a list of available cells, each a pair [r, c]. Color a cell by
(r + c) % 2 (a checkerboard). A single domino always covers exactly one even-color
and one odd-color cell — so the difference (#even - #odd) is an INVARIANT of any
tiling. Fix:
  - imbalance(cells): |#even-color - #odd-color|
  - tileable_parity(cells): True iff len(cells) is even AND imbalance(cells) == 0
"""


def imbalance(cells):
    # BUG: ignores color entirely, so it can never see the mutilated board.
    return 0


def tileable_parity(cells):
    # BUG: checks only that the count is even, ignoring the coloring invariant.
    return len(cells) % 2 == 0
