//! Course cs220 — The Art of Algorithms (Prof. Knuth).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the loop invariant of binary search (if x is present it lies in the current half-open interval) and why the loop terminates",
        "Implement a correct binary search that handles the empty array and out-of-range targets without indexing errors",
        "Explain why the running time is O(log n) — the interval halves each step",
    ];
    lesson(
        "cs220-u1-l1", "Binary Search and the Invariant", &o,
        "The algorithm everyone gets wrong. Force the half-open interval [lo,hi) and the \
         invariant 'if present, x is in a[lo:hi]'. The classic bug is a closed interval with \
         hi=len(a) and <= that indexes out of bounds. Require the O(log n) argument.",
        &[],
        practice("Implement search(a, x) on a sorted array; handle empty and out-of-range without errors.",
                 &["bsearch.py", "test_bsearch.py"], "pytest -q test_bsearch.py"),
        vec![
            crit(o[0], "States the half-open invariant and that the interval strictly shrinks (termination)."),
            crit(o[1], "search passes all cases including empty array and targets above/below the range."),
            crit(o[2], "Explains O(log n) because the interval halves each iteration."),
        ],
    )
}

fn cs220() -> Course {
    Course {
        id: s("cs220"),
        title: s("The Art of Algorithms"),
        professor: s("knuth"),
        prerequisites: vec![],
        units: vec![unit("cs220-u1", "Searching & Invariants", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs220 });
