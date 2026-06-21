"""Vector clocks — STARTER (the merge on receive and happens_before are missing).

Each process keeps a vector V of length n (one counter per process). A local or
send event bumps V[self]. On receive of a message vector M, you must MERGE
componentwise V[i] = max(V[i], M[i]) for all i, then bump V[self]. Causality:
a -> b iff a <= b componentwise and a != b.
"""


class Process:
    def __init__(self, pid: int, n: int) -> None:
        self.pid = pid
        self.n = n
        self.v = [0] * n

    def tick(self) -> list:
        self.v[self.pid] += 1
        return self.v[:]

    def send(self) -> list:
        return self.tick()

    def recv(self, msg: list) -> list:
        # BUG: bumps own component but forgets to MERGE the incoming vector.
        self.v[self.pid] += 1
        return self.v[:]


def happens_before(a: list, b: list) -> bool:
    # a -> b iff a <= b componentwise AND a != b.  STUB: not implemented.
    return False


def concurrent(a: list, b: list) -> bool:
    return not happens_before(a, b) and not happens_before(b, a)
