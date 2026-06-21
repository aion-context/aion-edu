//! Course cs270 — What Is Computable (Prof. Turing).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Describe a Turing machine as tape + head + state + transition table, and what one step does (read, write, move, change state)",
        "Implement the step function (write the symbol, move the head, change state) and run-until-HALT",
        "Show the binary-increment machine increments correctly, including carry propagation that extends the tape",
    ];
    lesson(
        "cs270-u1-l1", "A Turing Machine that Increments", &o,
        "Reduce computation to the barest mechanism. A config is (tape dict, head int, state). \
         One step reads tape[head], looks up (state,symbol) in the table, writes, moves +/-1, and \
         changes state. The given INCREMENT_TABLE then turns '1011' into '1100'. The classic stub \
         bug writes nothing and never moves. Require a precise description of the model.",
        &[],
        practice("Fix step(tape,head,state,table) to write the symbol and move the head; run to HALT.",
                 &["tm.py", "test_tm.py"], "pytest -q test_tm.py"),
        vec![
            crit(o[0], "Describes read/write/move/change-state precisely over tape+head+state."),
            crit(o[1], "step writes and moves correctly; run halts; tests pass."),
            crit(o[2], "Shows carry propagation (e.g. 111 -> 1000) extending the tape leftward."),
        ],
    )
}

fn cs270() -> Course {
    Course {
        id: s("cs270"),
        title: s("What Is Computable"),
        professor: s("turing"),
        prerequisites: vec![],
        units: vec![unit("cs270-u1", "Models of Computation", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: cs270 });
