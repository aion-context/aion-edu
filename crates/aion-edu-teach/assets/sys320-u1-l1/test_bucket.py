"""Mastery check for sys320-u1-l1 — token-bucket rate limiter."""
from bucket import TokenBucket


def test_burst_up_to_capacity():
    b = TokenBucket(capacity=3, rate=1)
    assert [b.allow(0) for _ in range(4)] == [True, True, True, False]


def test_refill_over_time():
    b = TokenBucket(capacity=3, rate=1)   # 1 token / second
    for _ in range(3):
        b.allow(0)                        # drain
    assert b.allow(0) is False            # empty
    assert b.allow(2) is True             # 2 seconds -> 2 tokens refilled
    assert b.allow(2) is True
    assert b.allow(2) is False            # only 2 were refilled


def test_refill_capped_at_capacity():
    b = TokenBucket(capacity=3, rate=1)
    for _ in range(3):
        b.allow(0)
    # a long wait refills to capacity (3), not 100
    assert [b.allow(100) for _ in range(4)] == [True, True, True, False]
