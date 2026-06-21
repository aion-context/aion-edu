"""Mastery check for cs220-u1-l1 — binary search."""
from bsearch import search


def test_found():
    a = [1, 3, 5, 7, 9]
    assert search(a, 5) == 2
    assert search(a, 1) == 0
    assert search(a, 9) == 4


def test_not_found_within_range():
    assert search([1, 3, 5, 7, 9], 4) == -1


def test_not_found_above_range():
    assert search([1, 3, 5, 7, 9], 10) == -1   # the classic out-of-bounds crash


def test_not_found_below_range():
    assert search([1, 3, 5, 7, 9], 0) == -1


def test_empty():
    assert search([], 3) == -1
