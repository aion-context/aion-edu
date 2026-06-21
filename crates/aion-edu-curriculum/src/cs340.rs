//! Course cs340 — Distributed Systems: Reasoning About Concurrency (Prof. Lamport).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define the happens-before relation and identify concurrent vs. causally-ordered events",
        "Implement a Lamport logical clock that respects happens-before",
        "Show by counterexample that Lamport clocks cannot detect concurrency (the converse fails)",
    ];
    lesson(
        "cs340-u1-l1", "Happens-Before and Lamport Clocks", &o,
        "Learner conflates time with order. Anchor on causality; physical clocks come later. The \
         third outcome is the one they miss — make them produce the counterexample themselves.",
        &[],
        practice("Implement a Lamport clock for a 3-process sim; exhibit two events with C(a)<C(b) that are concurrent.",
                 &["process.py", "test_clock.py"], "pytest -q test_clock.py"),
        vec![
            crit(o[0], "Correctly labels every pair in a 3-process execution as ordered or concurrent."),
            crit(o[1], "Clock monotonic per process and updated on receive; tests pass."),
            crit(o[2], "Exhibits a concrete pair where C(a)<C(b) yet a||b, and explains why."),
        ],
    )
}

fn l2() -> Lesson {
    let o = [
        "Implement vector clocks and use them to decide causality and concurrency exactly",
        "State the space cost of vector clocks and why it is fundamental",
    ];
    lesson(
        "cs340-u1-l2", "Vector Clocks and the Limits of Ordering", &o,
        "Direct sequel to l1's counterexample: vector clocks fix exactly the gap they just found. \
         Tie the O(n) cost to tracking each process independently.",
        &["cs340-u1-l1"],
        practice("Extend the sim to vector clocks; implement happens_before and concurrent.",
                 &["process.py", "test_vclock.py"], "pytest -q test_vclock.py"),
        vec![
            crit(o[0], "happens_before/concurrent correct on adversarial executions; tests pass."),
            crit(o[1], "Explains O(n) entries as irreducible given independent process histories."),
        ],
    )
}

fn cs340() -> Course {
    Course {
        id: s("cs340"),
        title: s("Distributed Systems: Reasoning About Concurrency"),
        professor: s("lamport"),
        prerequisites: vec![s("cs201"), s("cs210")],
        units: vec![unit("cs340-u1", "Logical Time", vec![l1(), l2()])],
    }
}

inventory::submit!(CourseRegistration { build: cs340 });
