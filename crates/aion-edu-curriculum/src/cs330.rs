//! Course cs330 — Abstraction and Specification (Prof. Liskov).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define a representation invariant and an abstraction function for an abstract data type, and why every operation must preserve the rep invariant",
        "Implement make/add so a RatNum is always kept in lowest terms with a positive denominator (the rep invariant)",
        "Implement check_rep to verify the invariant, and show it catches unreduced or negative-denominator representations",
    ];
    lesson(
        "cs330-u1-l1", "Abstract Data Types: The Rep Invariant", &o,
        "The rational-number ADT — Liskov's classic. The rep is a pair (num, den); the rep invariant \
         is den>0 and gcd(|num|,den)==1 (lowest terms, sign in the numerator); the abstraction \
         function is num/den. Every operation must re-establish the invariant — the stub stores \
         fractions unreduced and check_rep returns True always. Require the invariant + AF stated.",
        &[],
        practice("Fix make(num,den), add(a,b), and check_rep(r) so the rep invariant always holds.",
                 &["ratnum.py", "test_ratnum.py"], "pytest -q test_ratnum.py"),
        vec![
            crit(o[0], "States the rep invariant and abstraction function and the preserve-on-every-op duty."),
            crit(o[1], "make/add keep RatNum reduced with den>0; tests pass."),
            crit(o[2], "check_rep catches unreduced and negative-denominator reps."),
        ],
    )
}

fn cs330() -> Course {
    Course {
        id: s("cs330"),
        title: s("Abstraction and Specification"),
        professor: s("liskov"),
        prerequisites: vec![],
        units: vec![unit("cs330-u1", "Abstract Data Types", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs330 });
