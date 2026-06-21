"""Mastery check for sys301-u1-l1 — consistent hashing."""
from ring import h, build_ring, node_for


def _owner(key, ring):
    k = h(key)
    for pos, n in ring:
        if pos >= k:
            return n
    return ring[0][1]


def test_key_goes_to_clockwise_node():
    ring = build_ring(["A", "B", "C"])
    for key in ["k1", "k2", "apple", "banana", "x", "y", "z"]:
        assert node_for(key, ring) == _owner(key, ring)


def test_distribution_uses_multiple_nodes():
    ring = build_ring(["A", "B", "C", "D"])
    seen = {node_for("key%d" % i, ring) for i in range(200)}
    assert len(seen) >= 2


def test_removing_a_node_remaps_only_its_keys():
    nodes = ["A", "B", "C", "D", "E"]
    keys = ["key%d" % i for i in range(300)]
    full = build_ring(nodes)
    before = {k: node_for(k, full) for k in keys}
    minus = build_ring([n for n in nodes if n != "C"])
    after = {k: node_for(k, minus) for k in keys}
    for k in keys:
        if before[k] != "C":
            assert after[k] == before[k]            # untouched
    # every key that moved was previously owned by C
    assert all(before[k] == "C" for k in keys if before[k] != after[k])
