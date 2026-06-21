"""Mastery check for sys310-u1-l1 — quorum consensus."""
from quorum import is_strongly_consistent, read_quorum


def test_strong_consistency_condition():
    assert is_strongly_consistent(3, 2, 2) is True
    assert is_strongly_consistent(3, 3, 1) is True
    assert is_strongly_consistent(5, 3, 3) is True
    assert is_strongly_consistent(3, 1, 2) is False     # 1 + 2 == 3 == N
    assert is_strongly_consistent(5, 2, 3) is False     # 2 + 3 == 5 == N


def test_read_returns_highest_version():
    assert read_quorum([(5, "new"), (2, "old"), (2, "old")], 2) == "new"
    assert read_quorum([(1, "a"), (7, "b"), (3, "c")], 3) == "b"


def test_read_quorum_only_reads_r_replicas():
    # with R = 2 we only see the first two -> newest among them is "b" (v2)
    assert read_quorum([(1, "a"), (2, "b"), (9, "z")], 2) == "b"
