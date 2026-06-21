//! Course ee210 — Error-Correcting Codes (Prof. Hamming).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Explain how 3 parity bits over overlapping position-sets let a single-bit error be both detected and located",
        "Implement the syndrome so it returns the 1-based position of a flipped bit (0 if none)",
        "Show decode corrects any single-bit error and recovers the original 4 data bits",
    ];
    lesson(
        "ee210-u1-l1", "Hamming(7,4): Single-Error Correction", &o,
        "The leap from detection to correction. Parity bits at positions 1,2,4 cover the \
         positions whose index has that bit set; the recomputed parities, read as a binary \
         number, are the ADDRESS of the flipped bit. Require why the syndrome locates the error.",
        &[],
        practice("Implement syndrome(code) (1-based error position, 0 if none) and decode(code).",
                 &["hamming.py", "test_hamming.py"], "pytest -q test_hamming.py"),
        vec![
            crit(o[0], "Explains overlapping parity sets encode the binary position of the flipped bit."),
            crit(o[1], "syndrome returns the correct 1-based position for every single-bit flip, 0 if clean."),
            crit(o[2], "decode corrects any single-bit error and recovers the 4 data bits; tests pass."),
        ],
    )
}

fn ee210() -> Course {
    Course {
        id: s("ee210"),
        title: s("Error-Correcting Codes"),
        professor: s("hamming"),
        prerequisites: vec![],
        units: vec![unit("ee210-u1", "Detection & Correction", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: ee210 });
