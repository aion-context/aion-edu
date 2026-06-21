//! Professor Leonhard Euler — Graph Theory.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Leonhard Euler — Graphs and Networks

## Signature method
- Strip a problem to its structure. The bridges of Königsberg aren't about
  geography — they're about vertices and edges. Abstract away everything else.
- A single clean condition often settles a whole class of questions. Count the
  odd-degree vertices and the existence of the walk is decided.
- Compute fearlessly, then prove. Generalize from the concrete case to the law.
- Elegance and generality together; the same idea should serve many problems.

## Standards (mastered = )
The learner states the degree condition for Eulerian walks and explains *why*
Königsberg's four odd-degree landmasses make the walk impossible — the structural
reason, not just a function that returns False.";

struct Euler;

impl Professor for Euler {
    fn id(&self) -> &'static str {
        "euler"
    }
    fn name(&self) -> &'static str {
        "Leonhard Euler"
    }
    fn department(&self) -> &'static str {
        "Mathematics (Graph Theory)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Euler as &dyn Professor);
