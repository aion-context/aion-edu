//! Course math150 — How to Solve It: Invariants (Prof. Pólya).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State what an invariant is and why an unchanging quantity proves an arrangement is impossible",
        "Implement the coloring-parity invariant (imbalance) and a parity-based tileability check",
        "Apply the invariant to the mutilated chessboard, and note that parity is necessary but not sufficient",
    ];
    lesson(
        "math150-u1-l1", "The Mutilated Chessboard", &o,
        "Lead the learner to the coloring invariant: a single domino covers one black + one white \
         square, so (#black - #white) is invariant. Guard the trap: parity is necessary, not sufficient.",
        &[],
        practice("Implement imbalance(cells) and tileable_parity(cells) for domino tilings.",
                 &["tiling.py", "test_tiling.py"], "pytest -q test_tiling.py"),
        vec![
            crit(o[0], "Explains an invariant as unchanged by every legal move, so a differing target is unreachable."),
            crit(o[1], "imbalance and tileable_parity implemented correctly; tests pass."),
            crit(o[2], "Decides the mutilated chessboard via imbalance=2 and states parity is necessary not sufficient."),
        ],
    )
}

fn l2() -> Lesson {
    let o = [
        "Identify the parity invariant preserved by an operation (flipping two coins changes the head count by an even number)",
        "Implement the invariant and use it to decide reachability",
        "Explain why equal parity is necessary, and discuss when it is (and isn't) sufficient",
    ];
    lesson(
        "math150-u1-l2", "Parity Invariants", &o,
        "Generalizes l1's coloring invariant to a moving system. Flipping two coins changes the head \
         count by an even amount, so its parity is invariant. Guard the same trap: necessary, not sufficient.",
        &["math150-u1-l1"],
        practice("Implement same_parity(a,b) and reachable(start,target) (same length AND same parity).",
                 &["parity.py", "test_parity.py"], "pytest -q test_parity.py"),
        vec![
            crit(o[0], "States flipping two coins changes the head count by -2/0/+2, so parity is invariant."),
            crit(o[1], "same_parity and reachable implemented; tests pass."),
            crit(o[2], "Explains parity as a necessary condition and notes sufficiency is separate."),
        ],
    )
}

fn l3() -> Lesson {
    let o = [
        "Compute the nim-sum (XOR of pile sizes) and state that nim-sum = 0 is the invariant of a losing position",
        "Decide whether a Nim position is winning for the player to move (nim-sum != 0)",
        "Find a winning move that returns the nim-sum to 0",
    ];
    lesson(
        "math150-u1-l3", "Nim and the XOR Invariant", &o,
        "The invariant idea at its most surprising: in Nim, XOR of the piles is the conserved quantity \
         that decides who wins. Small cases -> nim-sum -> the constructive winning move. Look back to l1/l2.",
        &["math150-u1-l2"],
        practice("Implement nim_sum(piles), is_winning(piles), and winning_move(piles) -> (index, new_size).",
                 &["nim.py", "test_nim.py"], "pytest -q test_nim.py"),
        vec![
            crit(o[0], "nim_sum is the XOR of piles; states nim-sum = 0 marks a P-position."),
            crit(o[1], "is_winning returns nim-sum != 0; tests pass."),
            crit(o[2], "winning_move returns a legal move whose result has nim-sum 0, and None from a loss."),
        ],
    )
}

fn math150() -> Course {
    Course {
        id: s("math150"),
        title: s("How to Solve It: Invariants"),
        professor: s("polya"),
        prerequisites: vec![],
        units: vec![unit("math150-u1", "Invariants & Impossibility", vec![l1(), l2(), l3()])],
    }
}

inventory::submit!(CourseRegistration { build: math150 });
