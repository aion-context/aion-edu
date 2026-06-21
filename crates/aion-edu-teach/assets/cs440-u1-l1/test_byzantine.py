"""Mastery check for cs440-u1-l1 — Byzantine agreement and the 3f+1 bound."""
from byzantine import agreement_possible, om_rounds, majority


def test_three_f_plus_one_bound():
    assert agreement_possible(4, 1) is True      # 4 >= 4
    assert agreement_possible(3, 1) is False     # 3 generals, 1 traitor: impossible
    assert agreement_possible(7, 2) is True      # 7 >= 7
    assert agreement_possible(6, 2) is False     # 6 < 7
    assert agreement_possible(1, 0) is True


def test_om_round_count():
    assert om_rounds(0) == 1
    assert om_rounds(1) == 2
    assert om_rounds(3) == 4


def test_loyal_majority_decides():
    # n=4, f=1: three loyal say "attack", one traitor says "retreat"
    assert majority(["attack", "attack", "attack", "retreat"], "RETREAT") == "attack"
    # a tie is not a strict majority -> default
    assert majority(["a", "a", "b", "b"], "RETREAT") == "RETREAT"
    # a plurality that is not a majority -> default
    assert majority(["a", "a", "b", "c", "d"], "RETREAT") == "RETREAT"
