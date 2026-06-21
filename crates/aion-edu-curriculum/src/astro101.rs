//! Course astro101 — Cosmos: Order in the Sky (Prof. Sagan).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State Kepler's third law: T² is proportional to r³, with the same constant for every orbit around a given mass",
        "Implement the circular orbital period T = 2·pi·r / v with v = sqrt(M/r), and show T²/r³ is constant across radii",
        "Predict from the law alone that quadrupling the radius multiplies the period by eight (4^1.5)",
    ];
    lesson(
        "astro101-u1-l1", "Kepler's Third Law", &o,
        "From one orbit to the law that rules every orbit. With G=1, a circular orbit has \
         v=sqrt(M/r) and T=2·pi·r/v, so T²/r³ = 4·pi²/M — the same for every radius. Make them \
         SEE the constant hold across radii (evidence), then predict a new case from the law.",
        &[],
        practice("Implement circular_period(M,r) and period_ratio(M,r1,r2); confirm T²/r³ is constant.",
                 &["kepler.py", "test_kepler.py"], "pytest -q test_kepler.py"),
        vec![
            crit(o[0], "States T² ∝ r³ with one constant per central mass."),
            crit(o[1], "circular_period = 2·pi·r/v; kepler_constant is equal across several radii."),
            crit(o[2], "Predicts period_ratio for 4× radius = 8 from the 3/2-power law."),
        ],
    )
}

fn astro101() -> Course {
    Course {
        id: s("astro101"),
        title: s("Cosmos: Order in the Sky"),
        professor: s("sagan"),
        prerequisites: vec![],
        units: vec![unit("astro101-u1", "Laws of Motion in the Heavens", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: astro101 });
