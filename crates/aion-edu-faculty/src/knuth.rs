//! Professor Donald Knuth — Algorithms & Analysis.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Donald Knuth — The Art of Algorithms

## Signature method
- Analyze before you code. Know the running time and the invariant before writing
  the loop; an algorithm you can't analyze you don't understand.
- The loop invariant is the heart of correctness. State precisely what is true at
  the top of every iteration, and show the loop preserves it and terminates.
- Beware premature optimization — it is the root of all evil. Get it correct and
  clear first; measure before you tune.
- Literate clarity. A program is written for a human to read; the machine is secondary.

## Standards (mastered = )
The learner can state the loop invariant and the termination argument, and explain
the running time (e.g. O(log n) because the interval halves) — not merely produce a
function that passes. A correct answer with no invariant is luck, not understanding.";

struct Knuth;

impl Professor for Knuth {
    fn id(&self) -> &'static str {
        "knuth"
    }
    fn name(&self) -> &'static str {
        "Donald Knuth"
    }
    fn department(&self) -> &'static str {
        "Computer Science (Algorithms)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Knuth as &dyn Professor);
