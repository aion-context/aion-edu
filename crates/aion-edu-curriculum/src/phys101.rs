//! Course phys101 — Mechanics: Reasoning Before Calculating (Prof. Feynman).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Use dimensional analysis to deduce that a pendulum's period scales as sqrt(L/g) and cannot depend on mass",
        "Implement period(L,g)=2*pi*sqrt(L/g) and a small-angle pendulum simulation, and show they agree",
        "Show numerically that the period is independent of mass and (for small angles) amplitude",
    ];
    lesson(
        "phys101-u1-l1", "The Pendulum, From Units Alone", &o,
        "Learner reaches for the formula. Make them derive the FORM by dimensions first: only \
         sqrt(L/g) has units of time; mass cannot appear. Then build the simulator and confront the prediction.",
        &[],
        practice("Implement period(L,g), predict_ratio(...), and simulate_period(L,g,theta0).",
                 &["pendulum.py", "test_pendulum.py"], "pytest -q test_pendulum.py"),
        vec![
            crit(o[0], "Argues from units that T ~ sqrt(L/g) and that mass cannot appear, BEFORE computing."),
            crit(o[1], "period and simulate_period agree within tolerance; tests pass."),
            crit(o[2], "Simulated period unchanged across small amplitudes and carries no mass dependence; explains why."),
        ],
    )
}

fn l2() -> Lesson {
    let o = [
        "Write the total mechanical energy E = 1/2 m v^2 + 1/2 k x^2 for a mass on a spring",
        "Show numerically that E is conserved as the system evolves (symplectic integration)",
        "Derive the amplitude-max-speed relation v_max = omega A from energy conservation",
    ];
    lesson(
        "phys101-u1-l2", "Energy: the Quantity That Doesn't Change", &o,
        "The Feynman point: there is a number — the energy — that doesn't change, and that fact \
         predicts the motion. Make the learner SEE conservation; connect why explicit Euler leaks energy.",
        &["phys101-u1-l1"],
        practice("Implement energy(m,k,x,v), a symplectic step/simulate, and max_speed(omega,A)=omega*A.",
                 &["energy.py", "test_energy.py"], "pytest -q test_energy.py"),
        vec![
            crit(o[0], "energy() includes both kinetic and potential terms; formula tests pass."),
            crit(o[1], "Symplectic step keeps energy constant within tolerance; explains why explicit Euler does not."),
            crit(o[2], "Derives v_max = omega A by setting all energy kinetic at x=0; sim peak speed matches."),
        ],
    )
}

fn l3() -> Lesson {
    let o = [
        "Write the SHM solution x(t) = A cos(omega t + phi) with omega = sqrt(k/m)",
        "Show the small-angle pendulum is SHM with omega = sqrt(g/L), recovering the lesson-1 period",
        "Verify the closed-form solution matches a numerical integration",
    ];
    lesson(
        "phys101-u1-l3", "The Harmonic Oscillator Everywhere", &o,
        "Ties l1 (period) and l2 (energy) together: anything near a stable equilibrium is a harmonic \
         oscillator. Recover 2*pi*sqrt(L/g) as a special case; confirm the cosine against integration.",
        &["phys101-u1-l2"],
        practice("Implement omega_spring, omega_pendulum, shm_x(t,A,omega,phi)=A cos(...), period(omega).",
                 &["shm.py", "test_shm.py"], "pytest -q test_shm.py"),
        vec![
            crit(o[0], "shm_x uses cosine with a phase and the correct omega; initial-condition tests pass."),
            crit(o[1], "omega_pendulum gives period 2*pi*sqrt(L/g), matching lesson 1; explains linearization."),
            crit(o[2], "shm_x agrees with the integrated trajectory within tolerance."),
        ],
    )
}

fn phys101() -> Course {
    Course {
        id: s("phys101"),
        title: s("Mechanics: Reasoning Before Calculating"),
        professor: s("feynman"),
        prerequisites: vec![],
        units: vec![unit("phys101-u1", "From Units to Oscillation", vec![l1(), l2(), l3()])],
    }
}

inventory::submit!(CourseRegistration { build: phys101 });
