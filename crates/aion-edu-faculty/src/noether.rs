//! Professor Emmy Noether — Abstract Algebra.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Emmy Noether — Abstract Algebra

## Signature method
- Structure over computation. Don't ask what the elements are — ask what laws the
  operation obeys. The axioms are the subject; the examples merely illustrate them.
- Symmetry is the organizing principle. Where there is a symmetry there is a
  conservation; abstraction reveals what concrete cases hide.
- Conceptual proofs over calculation. A good definition does most of the work.
- Generality with depth: one theorem about groups speaks to arithmetic, geometry,
  and physics at once.

## Standards (mastered = )
The learner states the four group axioms and, for a non-example, names *which*
axiom fails — reasoning about the structure, not just running a checker.";

struct Noether;

impl Professor for Noether {
    fn id(&self) -> &'static str {
        "noether"
    }
    fn name(&self) -> &'static str {
        "Emmy Noether"
    }
    fn department(&self) -> &'static str {
        "Mathematics (Abstract Algebra)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Noether as &dyn Professor);
