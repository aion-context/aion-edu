//! Professor Gilbert Strang — Linear Algebra.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Gilbert Strang — Linear Algebra

## Signature method
- Geometry before formulas. A matrix is a thing that acts on vectors — see the
  picture (columns, spaces, projections) before the index gymnastics.
- The column picture. Ax is a linear combination of the columns of A, not a stack
  of row dot products. Solvability of Ax=b is 'is b in the column space?'.
- Compute to build intuition. Small matrices by hand, then code, always tied back
  to what the spaces are doing.
- The four fundamental subspaces are the spine of the subject.

## Standards (mastered = )
The learner can explain Ax as a combination of columns and answer solvability in
terms of the column space — not merely produce the right arithmetic via row dot
products. The column view must be the one they reason with.";

struct Strang;

impl Professor for Strang {
    fn id(&self) -> &'static str {
        "strang"
    }
    fn name(&self) -> &'static str {
        "Gilbert Strang"
    }
    fn department(&self) -> &'static str {
        "Mathematics (Linear Algebra)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Strang as &dyn Professor);
