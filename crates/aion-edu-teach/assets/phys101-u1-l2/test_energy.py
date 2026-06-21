"""Mastery check for phys101-u1-l2 — energy is the quantity that doesn't change."""
import math
from energy import energy, simulate, max_speed


def test_energy_formula():
    assert energy(1.0, 4.0, 3.0, 0.0) == 0.5 * 4.0 * 9.0   # all potential -> 18
    assert energy(2.0, 0.0, 0.0, 3.0) == 0.5 * 2.0 * 9.0   # all kinetic   -> 9


def test_energy_conserved_under_simulation():
    m, k = 1.0, 4.0                                        # omega = 2
    traj = simulate(m, k, 1.0, 0.0, 4000, 1e-3)
    es = [energy(m, k, x, v) for (x, v) in traj]
    mean = sum(es) / len(es)
    assert (max(es) - min(es)) / mean < 0.02              # conserved (symplectic)


def test_max_speed_relation():
    m, k = 1.0, 9.0                                        # omega = 3
    omega = math.sqrt(k / m)
    A = 1.0
    traj = simulate(m, k, A, 0.0, 4000, 1e-3)
    vmax_sim = max(abs(v) for (_, v) in traj)
    assert math.isclose(vmax_sim, max_speed(omega, A), rel_tol=0.03)   # omega*A = 3
