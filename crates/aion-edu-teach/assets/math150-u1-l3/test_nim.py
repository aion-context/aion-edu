"""Mastery check for math150-u1-l3 — Nim and the XOR invariant."""
from nim import nim_sum, is_winning, winning_move


def test_nim_sum_is_xor():
    assert nim_sum([1, 2, 3]) == 0      # 1 ^ 2 ^ 3 = 0
    assert nim_sum([3, 4, 5]) == 2      # 3 ^ 4 ^ 5 = 2


def test_losing_position_has_zero_nim_sum():
    assert is_winning([1, 2, 3]) is False   # P-position — loss for the mover
    assert is_winning([1, 1]) is False


def test_winning_move_returns_to_zero():
    piles = [3, 4, 5]                        # nim-sum 2 -> winning
    assert is_winning(piles) is True
    mv = winning_move(piles)
    assert mv is not None
    i, new = mv
    assert 0 <= new < piles[i]               # must remove at least one
    after = piles[:]; after[i] = new
    assert nim_sum(after) == 0               # moved to a P-position


def test_no_winning_move_from_losing_position():
    assert winning_move([1, 2, 3]) is None
