"""Lamport logical clock — STARTER (recv is wrong on purpose).

Fix `recv` so a process that receives a message with timestamp `ts` advances
its clock to max(local, ts) + 1. Then `pytest -q test_clock.py` should pass.
"""


class Process:
    def __init__(self, pid: int) -> None:
        self.pid = pid
        self.clock = 0

    def tick(self) -> int:
        self.clock += 1
        return self.clock

    def send(self) -> int:
        return self.tick()

    def recv(self, ts: int) -> int:
        # BUG: ignores the incoming timestamp — does not respect happens-before.
        self.clock += 1
        return self.clock
