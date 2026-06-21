"""The group axioms — STARTER.

A group is a set G with a binary operation • satisfying: (1) closure — a•b in G;
(2) associativity — (a•b)•c = a•(b•c); (3) an identity e with e•a = a•e = a;
(4) every a has an inverse a' with a•a' = a'•a = e. We model • as op(a,b) over a
list `elems`. Fix `identity` (find the true identity) and `has_inverses` (both sides).
"""


def is_closed(elems, op):
    s = set(elems)
    return all(op(a, b) in s for a in elems for b in elems)


def is_associative(elems, op):
    return all(op(op(a, b), c) == op(a, op(b, c))
               for a in elems for b in elems for c in elems)


def identity(elems, op):
    # the e with op(e,a) == a == op(a,e) for ALL a; None if there is none.
    # BUG: returns the first element without checking it is an identity.
    return elems[0]


def has_inverses(elems, op, e):
    # every a has some b with op(a,b) == e == op(b,a).
    # BUG: returns True without checking anything.
    return True


def is_group(elems, op):
    if not is_closed(elems, op) or not is_associative(elems, op):
        return False
    e = identity(elems, op)
    if e is None:
        return False
    return has_inverses(elems, op, e)
