"""Rational numbers as an abstract data type — STARTER (Liskov).

A RatNum is represented by a pair (num, den). The REPRESENTATION INVARIANT we
promise to maintain after every operation:
    den > 0   and   gcd(|num|, den) == 1     (always lowest terms; sign in num)
The ABSTRACTION FUNCTION maps the rep to the value it denotes: AF(num,den)=num/den.
`make` must establish the invariant; `add` must re-establish it; `check_rep`
verifies it. Fix `make`, `add`, and `check_rep`.
"""
from math import gcd


def numer(r):
    return r[0]


def denom(r):
    return r[1]


def value(r):           # the abstraction function, as a float (for testing)
    return r[0] / r[1]


def make(num, den):
    # construct a RatNum in lowest terms with den > 0.
    # BUG: stores (num, den) as given — not reduced, sign not normalized.
    return (num, den)


def add(a, b):
    # a/b + c/d = (ad + cb) / (bd), then REDUCE via make.
    n = numer(a) * denom(b) + numer(b) * denom(a)
    d = denom(a) * denom(b)
    # BUG: returns the unreduced pair (invariant broken).
    return (n, d)


def mul(a, b):
    return make(numer(a) * numer(b), denom(a) * denom(b))


def check_rep(r):
    # True iff the representation invariant holds for r.
    # BUG: always returns True (checks nothing).
    return True
