//! Course sys310 — Consistency and Consensus (Prof. Brewer).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the quorum condition R + W > N and explain why it forces every read set to overlap a write set (strong consistency)",
        "Implement is_strongly_consistent (strictly R + W > N) and a read quorum returning the highest-versioned value among R replicas",
        "Show that R + W == N does NOT guarantee consistency (the read and write sets can miss each other)",
    ];
    lesson(
        "sys310-u1-l1", "Quorum Consensus", &o,
        "The dial behind Dynamo-style stores. With N replicas, writing to W and reading from R, the \
         condition R + W > N forces the read and write sets to overlap, so a read always sees the \
         latest write. The trap is R + W == N (off-by-one) which does NOT guarantee overlap. The stub \
         uses >= and ignores versions. Require the overlap argument.",
        &[],
        practice("Implement is_strongly_consistent(n,r,w) and read_quorum(replicas,r) (highest version among first R).",
                 &["quorum.py", "test_quorum.py"], "pytest -q test_quorum.py"),
        vec![
            crit(o[0], "States R + W > N and the overlap argument for strong consistency."),
            crit(o[1], "is_strongly_consistent is strict (>); read_quorum returns the newest version among R; tests pass."),
            crit(o[2], "Explains why R + W == N can leave read and write sets disjoint."),
        ],
    )
}

fn sys310() -> Course {
    Course {
        id: s("sys310"),
        title: s("Consistency and Consensus"),
        professor: s("brewer"),
        prerequisites: vec![],
        units: vec![unit("sys310-u1", "Quorums", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: sys310 });
