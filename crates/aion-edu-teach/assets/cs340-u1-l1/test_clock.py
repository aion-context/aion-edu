"""Mastery check for cs340-u1-l1 — Lamport clocks."""
from process import Process


def test_send_increments_monotonically():
    p = Process(1)
    assert p.send() == 1
    assert p.send() == 2


def test_recv_takes_max_plus_one():
    p = Process(2)
    p.tick()                 # local clock = 1
    assert p.recv(5) == 6    # max(1, 5) + 1   (fails on the buggy starter)


def test_three_process_message_passing():
    p1, p2, p3 = Process(1), Process(2), Process(3)
    m1 = p1.send()           # 1
    p2.recv(m1)              # 2
    m2 = p2.send()           # 3
    p3.recv(m2)              # 4
    assert (p1.clock, p2.clock, p3.clock) == (1, 3, 4)
