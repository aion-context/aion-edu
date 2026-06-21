//! Course cs450 — The Art of Multiprocessor Programming (Prof. Herlihy).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define linearizability: a concurrent history is correct iff some sequential order respects real-time order and is legal for the object's sequential spec",
        "Implement precedes (real-time order: a ends before b starts) and legal_register (a read returns the most recent write)",
        "Show a history with a read returning a stale value after the write completed is NOT linearizable, while overlapping operations can be",
    ];
    lesson(
        "cs450-u1-l1", "Linearizability", &o,
        "The gold standard for concurrent-object correctness. A history of register operations (each \
         with a real-time [start,end] interval) is linearizable iff some single sequential order both \
         respects real-time precedence (A before B if A returns before B is called) AND is legal for a \
         register (a read returns the most recent write). The stub compares start times and never \
         checks the read value. Require the definition, not just a passing checker.",
        &[],
        practice("Fix precedes(a,b) (a ends before b starts) and legal_register(order) (replay writes, check reads).",
                 &["linearize.py", "test_linearize.py"], "pytest -q test_linearize.py"),
        vec![
            crit(o[0], "Defines linearizability as a legal sequential witness consistent with real-time order."),
            crit(o[1], "precedes uses a.end < b.start; legal_register replays writes and checks reads; tests pass."),
            crit(o[2], "Explains why a stale read after a completed write has no linearization."),
        ],
    )
}

fn cs450() -> Course {
    Course {
        id: s("cs450"),
        title: s("The Art of Multiprocessor Programming"),
        professor: s("herlihy"),
        prerequisites: vec![],
        units: vec![unit("cs450-u1", "Correctness of Concurrent Objects", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs450 });
