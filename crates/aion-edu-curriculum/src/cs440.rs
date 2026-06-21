//! Course cs440 — Distributed Algorithms (Prof. Lynch).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the Byzantine bound n >= 3f+1 (why 3 generals with 1 traitor is impossible but 4 is solvable) and that OM(f) needs f+1 rounds",
        "Implement agreement_possible (n >= 3f+1), om_rounds (f+1), and a strict-majority vote",
        "Show that with n >= 3f+1 the n-f loyal processes form a strict majority, so the majority vote is decided by the loyal ones",
    ];
    lesson(
        "cs440-u1-l1", "Byzantine Agreement and the 3f+1 Bound", &o,
        "State the model, then count. With up to f Byzantine (arbitrary, two-faced) faults, agreement \
         needs n >= 3f+1 — three generals and one traitor is impossible, four is solvable — and the \
         Oral-Messages algorithm OM(f) needs f+1 rounds. The bound is what makes the n-f loyal \
         processes a strict majority. The stub uses the crash bound 2f+1 and a non-strict vote.",
        &[],
        practice("Implement agreement_possible(n,f), om_rounds(f), and majority(values, default).",
                 &["byzantine.py", "test_byzantine.py"], "pytest -q test_byzantine.py"),
        vec![
            crit(o[0], "States n >= 3f+1 and OM(f) = f+1 rounds, with the 3-vs-4 generals intuition."),
            crit(o[1], "agreement_possible (3f+1), om_rounds (f+1), and a STRICT majority vote; tests pass."),
            crit(o[2], "Explains why n-f loyal processes outvote the f traitors when n >= 3f+1."),
        ],
    )
}

fn cs440() -> Course {
    Course {
        id: s("cs440"),
        title: s("Distributed Algorithms"),
        professor: s("lynch"),
        prerequisites: vec![],
        units: vec![unit("cs440-u1", "Fault-Tolerant Agreement", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs440 });
