"""Consistent hashing — a hash ring — STARTER.

Place each node at h(node) on a ring of size RING. A key is owned by the first
node clockwise from h(key) — the node with the smallest position >= h(key),
wrapping around to the first node. This way adding or removing a node remaps only
the keys near it (~1/N), not everything. Fix `node_for`.
"""
RING = 1024


def h(s):
    # deterministic FNV-1a + avalanche, reduced into [0, RING). The mixing step
    # matters: it spreads even short node names across the ring so ownership is
    # balanced (the real fix for clustering is virtual nodes — a later lesson).
    x = 2166136261
    for ch in str(s):
        x = ((x ^ ord(ch)) * 16777619) & 0xFFFFFFFF
    x ^= x >> 13
    x = (x * 2654435761) & 0xFFFFFFFF
    return x % RING


def build_ring(nodes):
    # list of (position, node) sorted by position
    return sorted((h(n), n) for n in nodes)


def node_for(key, ring):
    # ring: sorted list of (pos, node). Return the node owning `key`: the first
    # node clockwise (smallest pos >= h(key)), wrapping to ring[0].
    # BUG: returns the first node on the ring regardless of the key.
    return ring[0][1]
