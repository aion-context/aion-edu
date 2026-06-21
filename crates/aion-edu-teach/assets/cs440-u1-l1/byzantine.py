"""Byzantine agreement and the 3f+1 bound (Lynch) — STARTER.

To reach agreement among n processes when up to f are BYZANTINE (arbitrary,
two-faced) faults, you need  n >= 3f + 1  — with 3 generals and 1 traitor it is
impossible; with 4 it is solvable. The Oral-Messages algorithm OM(f) needs f+1
rounds. When n >= 3f+1, the n-f loyal processes are a strict majority, so a
majority vote over the reported values is decided by the loyal ones. Fix
`agreement_possible`, `om_rounds`, and `majority`.
"""
from collections import Counter


def agreement_possible(n, f):
    # Byzantine agreement is solvable iff n >= 3f + 1.
    # BUG: uses 2f + 1 (the crash-fault bound), not 3f + 1.
    return n >= 2 * f + 1


def om_rounds(f):
    # number of rounds the Oral-Messages algorithm OM(f) uses.
    # BUG: off-by-one, returns f instead of f + 1.
    return f


def majority(values, default):
    # return the value held by a STRICT majority (> len/2), else `default`.
    # BUG: returns the most common value even without a strict majority.
    return Counter(values).most_common(1)[0][0]
