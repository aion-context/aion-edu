"""Shannon entropy — measuring information — STARTER.

The entropy of a distribution p (a list of probabilities summing to 1) is
    H(p) = -sum(pi * log2(pi))   over pi > 0   (terms with pi = 0 contribute 0).
It is measured in bits: the average number of yes/no questions to identify an
outcome. A fair coin has H = 1 bit; a certain outcome has H = 0. Fix `entropy`.
"""
import math


def entropy(p):
    # H = -sum(pi * log2(pi)), skipping zero-probability terms.
    # BUG: uses natural log and forgets the leading minus sign.
    return sum(pi * math.log(pi) for pi in p if pi > 0)
