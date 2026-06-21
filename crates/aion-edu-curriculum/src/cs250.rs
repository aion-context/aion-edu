//! Course cs250 — Programs from Proofs (Prof. Dijkstra).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the post-condition (a*a <= n < (a+1)*(a+1)) and the loop invariant (a*a <= n) for integer square root",
        "Implement isqrt deriving the loop guard from the post-condition, correct on perfect squares and between them",
        "Argue termination: a strictly increases and is bounded by sqrt(n)",
    ];
    lesson(
        "cs250-u1-l1", "Integer Square Root by Invariant", &o,
        "Derive the program from its specification. Post-condition a*a <= n < (a+1)². The guard \
         follows: while (a+1)*(a+1) <= n: a += 1. The classic bug `while a*a <= n` overshoots by \
         one. Require the post-condition, invariant, and termination — not trial and error.",
        &[],
        practice("Implement isqrt(n) = largest a with a*a <= n, derived from the post-condition.",
                 &["isqrt.py", "test_isqrt.py"], "pytest -q test_isqrt.py"),
        vec![
            crit(o[0], "States the post-condition a*a <= n < (a+1)² and the invariant a*a <= n."),
            crit(o[1], "isqrt correct on perfect squares and between them; tests pass."),
            crit(o[2], "Argues a strictly increases and is bounded, so the loop terminates."),
        ],
    )
}

fn cs250() -> Course {
    Course {
        id: s("cs250"),
        title: s("Programs from Proofs"),
        professor: s("dijkstra"),
        prerequisites: vec![],
        units: vec![unit("cs250-u1", "Correctness by Construction", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs250 });
