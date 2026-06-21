"""Binary search — STARTER (boundary handling is wrong on purpose).

Invariant (half-open): if x is in the array, it lies in a[lo:hi). Start lo=0,
hi=len(a). While lo < hi: mid = (lo + hi) // 2; if a[mid] < x: lo = mid + 1 else
hi = mid. Return lo if lo < len(a) and a[lo] == x else -1. Fix `search`.
"""


def search(a, x):
    # BUG: closed interval with hi = len(a) and `lo <= hi` — indexes a[mid] out of
    # bounds for targets above the range, and mishandles the empty array.
    lo, hi = 0, len(a)
    while lo <= hi:
        mid = (lo + hi) // 2
        if a[mid] == x:
            return mid
        elif a[mid] < x:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
