"""Eulerian paths — the Seven Bridges of Königsberg — STARTER.

A connected (multi)graph has an Eulerian CIRCUIT iff every vertex has even degree,
and an Eulerian PATH (start != end allowed) iff exactly zero or two vertices have
odd degree. Königsberg had four landmasses, all of odd degree -> no walk crossing
every bridge exactly once. Fix `odd_degree_count` and `has_eulerian_path`.
"""


def degrees(n, edges):
    d = [0] * n
    for u, v in edges:
        d[u] += 1
        d[v] += 1
    return d


def odd_degree_count(n, edges):
    # number of vertices with ODD degree.  BUG: counts even-degree vertices.
    return sum(1 for x in degrees(n, edges) if x % 2 == 0)


def has_eulerian_circuit(n, edges):
    return odd_degree_count(n, edges) == 0


def has_eulerian_path(n, edges):
    # Euler's condition: 0 or 2 odd-degree vertices.  BUG: only allows circuits.
    return odd_degree_count(n, edges) == 0
