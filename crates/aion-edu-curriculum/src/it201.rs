//! Course it201 — Information Theory (Prof. Shannon).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define Shannon entropy H(p) = -sum(pi log2 pi) in bits, and what it measures (average surprise to identify an outcome)",
        "Implement entropy correctly, in bits, treating zero-probability terms as zero",
        "Show H is 0 for a certain outcome and log2(n) (maximal) for the uniform distribution over n outcomes",
    ];
    lesson(
        "it201-u1-l1", "Entropy: Measuring Information", &o,
        "Information is surprise, measured in bits. H(p) = -sum(pi*log2(pi)) over pi>0. A fair coin \
         is 1 bit; a certain outcome is 0; the uniform over n is log2(n) — the maximum. The classic \
         bug uses natural log or drops the minus sign. Require the endpoints, not just a float.",
        &[],
        practice("Implement entropy(p) = -sum(pi*log2(pi)) over pi>0 (bits).",
                 &["entropy.py", "test_entropy.py"], "pytest -q test_entropy.py"),
        vec![
            crit(o[0], "Defines H in bits as average surprise / questions to identify an outcome."),
            crit(o[1], "entropy uses log base 2, the minus sign, and skips zero-probability terms; tests pass."),
            crit(o[2], "Shows certainty -> 0 bits and uniform over n -> log2(n) (the maximum)."),
        ],
    )
}

fn it201() -> Course {
    Course {
        id: s("it201"),
        title: s("Information Theory"),
        professor: s("shannon"),
        prerequisites: vec![],
        units: vec![unit("it201-u1", "Entropy", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: it201 });
