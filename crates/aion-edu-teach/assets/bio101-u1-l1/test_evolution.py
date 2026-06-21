"""Mastery check for bio101-u1-l1 — selection and allele frequencies."""
import math
from evolution import hardy_weinberg, next_p


def test_hardy_weinberg_sums_to_one():
    for p in [0.0, 0.3, 0.5, 0.7, 1.0]:
        aa_, ab_, bb_ = hardy_weinberg(p)
        assert math.isclose(aa_ + ab_ + bb_, 1.0)
    assert hardy_weinberg(0.5) == (0.25, 0.5, 0.25)


def test_neutral_selection_keeps_p():
    assert math.isclose(next_p(0.4, (1.0, 1.0, 1.0)), 0.4)


def test_selection_against_recessive_increases_p():
    # aa lethal (w_aa = 0): p must increase
    p1 = next_p(0.5, (1.0, 1.0, 0.0))
    assert p1 > 0.5
    assert math.isclose(p1, 2.0 / 3.0)   # (0.25 + 0.25) / 0.75
