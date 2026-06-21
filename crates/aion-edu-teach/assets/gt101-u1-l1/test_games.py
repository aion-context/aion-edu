"""Mastery check for gt101-u1-l1 — zero-sum games."""
import math
from games import maximin, minimax, has_saddle_point, game_value


def test_saddle_point_game():
    A = [[4, 3], [2, 1]]   # row-mins 3,1 -> maximin 3; col-maxes 4,3 -> minimax 3
    assert maximin(A) == 3 and minimax(A) == 3
    assert has_saddle_point(A) is True
    assert game_value(A) == 3


def test_no_saddle_mixed_value():
    A = [[2, -1], [-1, 1]]   # maximin -1, minimax 1 -> no saddle
    assert has_saddle_point(A) is False
    # v = (2*1 - (-1)(-1)) / (2 + 1 - (-1) - (-1)) = 1 / 5
    assert math.isclose(game_value(A), 0.2)
