//! Course math310 — Groups and Symmetry (Prof. Noether).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the four group axioms (closure, associativity, identity, inverses)",
        "Implement identity (find the true identity, not assume the first element) and the inverse check, so is_group is correct",
        "Decide whether a (set, operation) is a group — e.g. Z_n under addition yes, {0..n-1} under multiplication no",
    ];
    lesson(
        "math310-u1-l1", "The Group Axioms", &o,
        "Structure over computation: a group is a set with an operation obeying closure, \
         associativity, an identity, and inverses. The identity must be FOUND (the stub wrongly \
         assumes the first element); inverses must hold on both sides. Z_n under + is a group; \
         {0,1,2,3} under * mod 4 is not (0 has no inverse). Require which axiom fails for a non-example.",
        &[],
        practice("Implement identity(elems,op) (search for the true identity) and has_inverses(elems,op,e).",
                 &["group.py", "test_group.py"], "pytest -q test_group.py"),
        vec![
            crit(o[0], "States closure, associativity, identity, and inverses."),
            crit(o[1], "identity searches for the real identity; has_inverses checks both sides; is_group correct."),
            crit(o[2], "Decides group vs non-group and names the failing axiom (e.g. 0 has no multiplicative inverse)."),
        ],
    )
}

fn math310() -> Course {
    Course {
        id: s("math310"),
        title: s("Groups and Symmetry"),
        professor: s("noether"),
        prerequisites: vec![],
        units: vec![unit("math310-u1", "Structure of Groups", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: math310 });
