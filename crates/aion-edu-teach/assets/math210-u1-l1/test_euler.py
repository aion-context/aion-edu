"""Mastery check for math210-u1-l1 — Eulerian paths."""
from euler import odd_degree_count, has_eulerian_circuit, has_eulerian_path

# Königsberg: 4 landmasses, 7 bridges, every vertex odd degree.
KONIGSBERG = (4, [(0, 1), (0, 1), (0, 2), (0, 2), (0, 3), (1, 3), (2, 3)])


def test_odd_count():
    assert odd_degree_count(*KONIGSBERG) == 4
    assert odd_degree_count(3, [(0, 1), (1, 2), (2, 0)]) == 0   # triangle, all even


def test_konigsberg_has_no_euler_walk():
    assert has_eulerian_path(*KONIGSBERG) is False   # four odd -> impossible


def test_circuit_when_all_even():
    assert has_eulerian_circuit(3, [(0, 1), (1, 2), (2, 0)]) is True


def test_path_with_exactly_two_odd():
    # path 0-1-2: endpoints 0 and 2 are odd -> Eulerian path but not circuit
    assert has_eulerian_path(3, [(0, 1), (1, 2)]) is True
    assert has_eulerian_circuit(3, [(0, 1), (1, 2)]) is False
