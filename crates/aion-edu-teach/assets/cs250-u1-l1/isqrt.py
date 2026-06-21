"""Integer square root by invariant — STARTER (overshoots by one).

isqrt(n) returns the largest a with a*a <= n. Post-condition:
    a*a <= n < (a+1)*(a+1)
Derive the loop guard from it: a = 0; while (a+1)*(a+1) <= n: a += 1; return a.
Invariant a*a <= n holds throughout; a strictly increases and is bounded, so the
loop terminates. Fix `isqrt`.
"""


def isqrt(n):
    # BUG: guard `a*a <= n` overshoots — returns floor(sqrt(n)) + 1.
    a = 0
    while a * a <= n:
        a += 1
    return a
