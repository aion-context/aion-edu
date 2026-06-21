//! Course sys330 — Designing for Failure (Prof. Vogels).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Describe the circuit breaker's three states (CLOSED, OPEN, HALF_OPEN) and the transitions between them",
        "Implement allow(now) (reject while OPEN until cooldown, then allow one trial) and on_result(success, now)",
        "Show the breaker trips after consecutive failures, rejects fast while open, recovers via a half-open trial, and that a success resets the failure count",
    ];
    lesson(
        "sys330-u1-l1", "The Circuit Breaker", &o,
        "The resilience pattern that keeps a sick dependency from taking down its callers. CLOSED lets \
         calls through; after `threshold` consecutive failures it trips OPEN and rejects fast; after \
         `cooldown` it goes HALF_OPEN and allows one probe — success closes it, failure re-opens it. \
         Time is explicit. The stub allows everything and records nothing. Require why fail-fast helps.",
        &[],
        practice("Implement CircuitBreaker.allow(now) and on_result(success, now) over CLOSED/OPEN/HALF_OPEN.",
                 &["breaker.py", "test_breaker.py"], "pytest -q test_breaker.py"),
        vec![
            crit(o[0], "Describes the three states and all transitions (trip, cooldown, probe, close/reopen)."),
            crit(o[1], "allow rejects in OPEN until cooldown then probes; on_result updates state; tests pass."),
            crit(o[2], "Shows trip, fast-reject, half-open recovery, and failure-count reset on success."),
        ],
    )
}

fn sys330() -> Course {
    Course {
        id: s("sys330"),
        title: s("Designing for Failure"),
        professor: s("vogels"),
        prerequisites: vec![],
        units: vec![unit("sys330-u1", "Resilience Patterns", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: sys330 });
