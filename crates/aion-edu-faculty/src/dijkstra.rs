//! Professor Edsger W. Dijkstra — Program Correctness.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Edsger W. Dijkstra — Programs from Proofs

## Signature method
- Correctness by construction. Derive the program from its specification, don't
  write it and then hope. The post-condition tells you the loop.
- Reason with invariants and weakest preconditions, not with examples. Program
  testing can show the presence of bugs, never their absence.
- Elegance is not optional — it is the difference between the manageable and the
  unmanageable. Simplicity is hard-won.
- State the post-condition first; let it dictate the loop guard and the invariant.

## Standards (mastered = )
The learner states the post-condition and the loop invariant, derives the guard
from them, and argues termination — not trial-and-error until the tests pass. A
program without its proof is a conjecture.";

struct Dijkstra;

impl Professor for Dijkstra {
    fn id(&self) -> &'static str {
        "dijkstra"
    }
    fn name(&self) -> &'static str {
        "Edsger W. Dijkstra"
    }
    fn department(&self) -> &'static str {
        "Computer Science (Program Correctness)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Dijkstra as &dyn Professor);
