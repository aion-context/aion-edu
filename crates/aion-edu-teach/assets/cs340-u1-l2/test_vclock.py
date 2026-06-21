"""Mastery check for cs340-u1-l2 — vector clocks decide causality exactly."""
from process import Process, happens_before, concurrent


def test_send_increments_own_component():
    p = Process(0, 3)
    assert p.send() == [1, 0, 0]
    assert p.send() == [2, 0, 0]


def test_recv_merges_componentwise_then_bumps():
    p = Process(1, 3)
    p.tick()                              # [0,1,0]
    assert p.recv([2, 0, 1]) == [2, 2, 1]  # max(local,msg) then own+1


def test_causal_order_detected():
    p0, p1 = Process(0, 3), Process(1, 3)
    e0 = p0.send()                        # [1,0,0]
    e1 = p1.recv(e0)                      # [1,1,0]
    assert happens_before(e0, e1) is True
    assert happens_before(e1, e0) is False


def test_concurrency_detected_exactly():
    a = Process(0, 3).tick()             # [1,0,0]
    b = Process(2, 3).tick()             # [0,0,1]
    assert concurrent(a, b) is True
    assert happens_before(a, b) is False


def test_detects_what_lamport_missed():
    # the l1 gap: integer clocks would order these, vector clocks see concurrency
    a = Process(0, 3).tick()             # [1,0,0]
    p1 = Process(1, 3); p1.tick(); b = p1.tick()   # [0,2,0]
    assert concurrent(a, b) is True
    assert happens_before(a, b) is False
    assert happens_before(b, a) is False
