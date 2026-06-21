"""Parity invariant — STARTER.

State: a list of 0/1 (coins; 1 = heads). Operation: flip any two coins. That
changes the number of heads by -2, 0, or +2 — so the PARITY of the head count is
an invariant. A target (same length) is reachable only if it has the same
head-count parity. Fix same_parity and reachable.
"""


def heads(state):
    return sum(state)


def same_parity(a, b):
    # BUG: compares the counts, not their parity.
    return heads(a) == heads(b)


def reachable(start, target):
    # Necessary condition: same length AND same head-count parity.
    # BUG: ignores parity — allows any target of equal length.
    return len(start) == len(target)
