"""Circuit breaker — designing for failure — STARTER.

Three states: CLOSED (calls pass through), OPEN (calls are rejected immediately to
shed load), HALF_OPEN (one trial call is allowed to test recovery). After
`threshold` consecutive failures it trips to OPEN; after `cooldown` seconds it
moves to HALF_OPEN; a success there closes it, a failure re-opens it. Time is
passed in explicitly. Fix `allow` and `on_result`.
"""
CLOSED, OPEN, HALF_OPEN = "CLOSED", "OPEN", "HALF_OPEN"


class CircuitBreaker:
    def __init__(self, threshold, cooldown):
        self.threshold = threshold
        self.cooldown = cooldown
        self.state = CLOSED
        self.failures = 0
        self.opened_at = 0.0

    def allow(self, now):
        # whether a call is permitted right now.
        # BUG: always allows (ignores OPEN / cooldown / half-open).
        return True

    def on_result(self, success, now):
        # record the outcome of a permitted call and update the state.
        # BUG: does nothing.
        return
