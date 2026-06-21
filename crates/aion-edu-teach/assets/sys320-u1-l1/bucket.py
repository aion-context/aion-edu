"""Token-bucket rate limiter — STARTER.

A bucket holds up to `capacity` tokens and refills at `rate` tokens per second.
Each request takes one token if available, else it is rejected. On each call we
first refill based on elapsed time: tokens = min(capacity, tokens + rate*elapsed).
Time is passed in explicitly (seconds) so the logic is deterministic. Fix `allow`.
"""


class TokenBucket:
    def __init__(self, capacity, rate):
        self.capacity = capacity
        self.rate = rate
        self.tokens = capacity
        self.last = 0.0

    def allow(self, now):
        # refill by the elapsed time, then consume one token if available.
        elapsed = now - self.last
        self.last = now
        # BUG: forgets to refill, and never checks or decrements tokens.
        return True
