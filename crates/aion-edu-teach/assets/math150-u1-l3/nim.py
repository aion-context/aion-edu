"""Nim and the XOR invariant — STARTER.

The nim-sum is the XOR of the pile sizes. The invariant of a losing position (a
P-position, a loss for the player about to move) is nim-sum == 0. Otherwise the
mover wins and can always move to make the nim-sum 0. Fix nim_sum and winning_move.
"""
from functools import reduce


def nim_sum(piles):
    # XOR of all pile sizes.   BUG: returns the ordinary sum instead.
    return sum(piles)


def is_winning(piles):
    # Winning for the player to move iff the nim-sum is non-zero.
    return nim_sum(piles) != 0


def winning_move(piles):
    # Return (pile_index, new_size) that makes the nim-sum 0, or None if losing.
    # BUG: not implemented.
    return None
