"""Linearizability of a concurrent register history (Herlihy & Wing) — STARTER.

A history is a list of operations on a shared register, each a tuple:
    (kind, value, start, end)   kind in {"W","R"}; value is written (W) or returned (R)
[start, end] is the operation's real-time interval (intervals may overlap). The
history is LINEARIZABLE if there is a single sequential order of the operations
that (1) respects real-time order — if A finishes before B starts, A precedes B —
and (2) is legal for a register: every read returns the value of the most recent
write (or the initial value). Fix `precedes` and `legal_register`.
"""
from itertools import permutations

KIND, VALUE, START, END = 0, 1, 2, 3


def precedes(a, b):
    # True iff op a finishes strictly before op b starts (real-time order).
    # BUG: compares start times instead of a.end < b.start.
    return a[START] < b[START]


def legal_register(order, initial=0):
    # order: ops in a proposed total order. Replay it: a write sets the current
    # value, a read must return the current value. True iff every read matches.
    # BUG: ignores writes and never checks the read value.
    return True


def respects_realtime(order, ops):
    pos = {id(op): i for i, op in enumerate(order)}
    for a in ops:
        for b in ops:
            if a is not b and precedes(a, b) and pos[id(a)] > pos[id(b)]:
                return False
    return True


def is_linearizable(ops, initial=0):
    for order in permutations(ops):
        if respects_realtime(order, ops) and legal_register(order, initial):
            return True
    return False
