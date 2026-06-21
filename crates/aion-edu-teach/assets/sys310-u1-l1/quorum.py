"""Quorum consensus (R + W > N) — STARTER.

N replicas hold versioned values. A write goes to W replicas (highest version
wins); a read queries R replicas and takes the value with the highest version.
If R + W > N, the read set and any write set must OVERLAP, so a read always sees
the latest write — strong consistency. Fix `is_strongly_consistent` and
`read_quorum`.
"""


def is_strongly_consistent(n, r, w):
    # strong consistency is guaranteed iff R + W > N.
    # BUG: uses >= instead of > (R + W == N does NOT guarantee overlap).
    return r + w >= n


def read_quorum(replicas, r):
    # replicas: list of (version, value). Read the first R replicas and return
    # the value with the HIGHEST version among them.
    # BUG: returns the first replica's value, ignoring versions.
    chosen = replicas[:r]
    return chosen[0][1]
