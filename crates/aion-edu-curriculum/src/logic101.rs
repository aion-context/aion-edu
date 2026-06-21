//! Course logic101 — The Laws of Thought (Prof. Boole).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define tautology (true under every assignment), satisfiable (true under some), and contradiction (true under none)",
        "Implement is_tautology and is_satisfiable by checking all assignments",
        "Verify a law of logic (e.g. De Morgan, excluded middle) is a tautology, and exhibit a satisfiable-but-not-tautology formula",
    ];
    lesson(
        "logic101-u1-l1", "Boolean Algebra and Tautology", &o,
        "Reasoning is calculation over 0 and 1. A formula is f(env)->bool; all_envs enumerates every \
         assignment. is_tautology = true under EVERY assignment (all); is_satisfiable = true under SOME \
         (any). The classic bug returns f of only the first assignment (a `return` inside the loop). \
         Require the precise distinction between the three modes.",
        &[],
        practice("Implement is_tautology(f,vars) = all(...) and is_satisfiable(f,vars) = any(...) over all_envs.",
                 &["boole.py", "test_boole.py"], "pytest -q test_boole.py"),
        vec![
            crit(o[0], "Defines tautology / satisfiable / contradiction precisely."),
            crit(o[1], "is_tautology uses all over every assignment; is_satisfiable uses any; tests pass."),
            crit(o[2], "Shows De Morgan / excluded middle is a tautology and a satisfiable-but-not-tautology case."),
        ],
    )
}

fn logic101() -> Course {
    Course {
        id: s("logic101"),
        title: s("The Laws of Thought"),
        professor: s("boole"),
        prerequisites: vec![],
        units: vec![unit("logic101-u1", "Propositional Logic", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: logic101 });
