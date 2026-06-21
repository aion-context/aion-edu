"""Mastery check for math150-u1-l1 — the mutilated chessboard invariant."""
from tiling import imbalance, tileable_parity


def full(n):
    return [[r, c] for r in range(n) for c in range(n)]


def test_full_board_is_balanced():
    assert imbalance(full(8)) == 0
    assert tileable_parity(full(8)) is True


def test_mutilated_chessboard_is_impossible():
    # remove two OPPOSITE corners (0,0) and (7,7) — both the same color
    board = [c for c in full(8) if c not in ([0, 0], [7, 7])]
    assert imbalance(board) == 2          # the invariant sees it
    assert tileable_parity(board) is False


def test_remove_two_opposite_color_cells_keeps_parity():
    # remove (0,0) [even] and (0,1) [odd] — balance preserved
    board = [c for c in full(8) if c not in ([0, 0], [0, 1])]
    assert imbalance(board) == 0
    assert tileable_parity(board) is True


def test_odd_cell_count_fails():
    assert tileable_parity(full(3)) is False   # 9 cells — can't be tiled by dominoes
