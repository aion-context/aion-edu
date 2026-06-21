//! Course gt101 — Strategy and Games (Prof. von Neumann).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Define maximin (row player) and minimax (column player) for a zero-sum game, and when they coincide as a saddle point",
        "Implement maximin, minimax, and saddle-point detection correctly",
        "Compute the value of a 2x2 game with no saddle via the mixed-strategy formula",
    ];
    lesson(
        "gt101-u1-l1", "Zero-Sum Games and the Saddle Point", &o,
        "Formalize the conflict as a payoff matrix A (row player gains, column loses). Row player \
         maximizes the row-minimum (maximin); column minimizes the column-maximum (minimax). They \
         coincide => a saddle point (pure value). For a 2x2 with no saddle, value = \
         (a00·a11 − a01·a10)/(a00+a11−a01−a10). The stub uses row-max and col-min by mistake.",
        &[],
        practice("Implement maximin(A), minimax(A), has_saddle_point(A), and game_value(A).",
                 &["games.py", "test_games.py"], "pytest -q test_games.py"),
        vec![
            crit(o[0], "Defines maximin as max of row-mins, minimax as min of col-maxes, saddle when equal."),
            crit(o[1], "maximin/minimax/has_saddle_point correct; tests pass."),
            crit(o[2], "Computes the 2x2 mixed-strategy value when no saddle exists, and explains the saddle."),
        ],
    )
}

fn gt101() -> Course {
    Course {
        id: s("gt101"),
        title: s("Strategy and Games"),
        professor: s("vonneumann"),
        prerequisites: vec![],
        units: vec![unit("gt101-u1", "Minimax", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: gt101 });
