"""Mastery check for sys330-u1-l1 — the circuit breaker."""
from breaker import CircuitBreaker, CLOSED, OPEN, HALF_OPEN


def test_trips_open_after_threshold_failures():
    b = CircuitBreaker(threshold=3, cooldown=10)
    for t in range(3):
        assert b.allow(t) is True
        b.on_result(False, t)
    assert b.state == OPEN
    assert b.allow(5) is False              # within cooldown -> rejected fast


def test_half_open_then_close_on_success():
    b = CircuitBreaker(threshold=2, cooldown=10)
    for t in range(2):
        b.allow(t)
        b.on_result(False, t)
    assert b.state == OPEN
    assert b.allow(5) is False              # still cooling down
    assert b.allow(15) is True              # cooldown elapsed -> trial allowed
    assert b.state == HALF_OPEN
    b.on_result(True, 15)                   # trial succeeds -> close
    assert b.state == CLOSED
    assert b.allow(16) is True


def test_half_open_failure_reopens():
    b = CircuitBreaker(threshold=2, cooldown=10)
    for t in range(2):
        b.allow(t)
        b.on_result(False, t)
    assert b.allow(15) is True              # half-open trial
    b.on_result(False, 15)                  # trial fails -> reopen
    assert b.state == OPEN
    assert b.allow(16) is False


def test_success_resets_failure_count():
    b = CircuitBreaker(threshold=3, cooldown=10)
    b.allow(0); b.on_result(False, 0)
    b.allow(1); b.on_result(False, 1)
    b.allow(2); b.on_result(True, 2)        # success resets the count
    assert b.failures == 0 and b.state == CLOSED
    b.allow(3); b.on_result(False, 3)
    assert b.state == CLOSED                # only one failure since the reset
