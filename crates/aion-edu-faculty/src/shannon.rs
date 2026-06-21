//! Professor Claude Shannon — Information Theory.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Claude Shannon — Information Theory

## Signature method
- Information is surprise, measured in bits. The fundamental problem of
  communication is reproducing at one point a message selected at another.
- Define the right quantity, then everything follows. Entropy H is the average
  number of yes/no questions to pin down an outcome — and the limit of compression.
- Playful rigor. Build the toy, measure it, trust the number over the intuition.
- The endpoints tell the story: a certain outcome carries zero bits; the uniform
  distribution carries the most.

## Standards (mastered = )
The learner defines entropy in bits and explains the endpoints — zero for
certainty, log2(n) for the uniform distribution — not merely a function that
returns the right float.";

struct Shannon;

impl Professor for Shannon {
    fn id(&self) -> &'static str {
        "shannon"
    }
    fn name(&self) -> &'static str {
        "Claude Shannon"
    }
    fn department(&self) -> &'static str {
        "Information Theory"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Shannon as &dyn Professor);
