"""Energy of a mass on a spring — STARTER.

Total mechanical energy E = ½ m v² + ½ k x². It is conserved as the system
evolves — IF you integrate with a symplectic (semi-implicit) Euler step: update
the velocity first, then the position with the NEW velocity. Fix the two bugs.
"""


def energy(m, k, x, v):
    # E = ½ m v² + ½ k x².   BUG: forgets the potential-energy term.
    return 0.5 * m * v * v


def step(m, k, x, v, dt):
    # BUG: explicit Euler (advances x with the OLD v) — this leaks energy.
    x_new = x + v * dt
    v_new = v - (k / m) * x * dt
    return (x_new, v_new)


def simulate(m, k, x0, v0, steps, dt=1e-3):
    x, v = x0, v0
    traj = [(x, v)]
    for _ in range(steps):
        x, v = step(m, k, x, v, dt)
        traj.append((x, v))
    return traj


def max_speed(omega, A):
    # v_max = omega * A (all the energy is kinetic at x=0).  BUG: wrong.
    return A
