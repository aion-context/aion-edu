//! Course math230 — Number Theory (Prof. Gauss).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the Euclidean recurrence gcd(a,b) = gcd(b, a mod b) and the Bézout identity a·x + b·y = gcd(a,b)",
        "Implement the extended Euclidean algorithm returning (g, x, y) with a·x + b·y = g",
        "Compute a modular inverse from the extended algorithm, and recognize when none exists (gcd != 1)",
    ];
    lesson(
        "math230-u1-l1", "The Euclidean Algorithm and Modular Inverses", &o,
        "Congruences turn divisibility into arithmetic. gcd by repeated remainder; the EXTENDED \
         algorithm also yields x,y with a·x+b·y=gcd (Bézout), and then x mod m is the modular inverse \
         of a when gcd(a,m)=1. The classic bug forgets to update the Bézout coefficients in the \
         recursive step. Require the recurrence and the no-inverse condition.",
        &[],
        practice("Implement ext_gcd(a,b) -> (g,x,y) with a·x+b·y=g, and mod_inverse(a,m).",
                 &["gcd.py", "test_gcd.py"], "pytest -q test_gcd.py"),
        vec![
            crit(o[0], "States gcd(a,b)=gcd(b,a mod b) and the Bézout identity a·x+b·y=gcd."),
            crit(o[1], "ext_gcd returns coefficients satisfying a·x+b·y=g; tests pass."),
            crit(o[2], "mod_inverse correct, and returns None / explains why when gcd(a,m) != 1."),
        ],
    )
}

fn math230() -> Course {
    Course {
        id: s("math230"),
        title: s("Number Theory"),
        professor: s("gauss"),
        prerequisites: vec![],
        units: vec![unit("math230-u1", "Divisibility & Congruence", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: math230 });
