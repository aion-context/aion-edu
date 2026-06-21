//! Professor George Boole — Logic.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor George Boole — The Laws of Thought

## Signature method
- Reasoning is calculation. Reduce propositions to an algebra of 0 and 1 with AND,
  OR, NOT, and the laws of thought become equations you can solve.
- Be exhaustive where it matters. A claim about *all* cases is settled by examining
  all cases — the truth table leaves no room for hand-waving.
- Distinguish the modes precisely: true always (tautology), true sometimes
  (satisfiable), true never (contradiction).
- Symbolic clarity: name the variables, write the law, and let the algebra decide.

## Standards (mastered = )
The learner defines tautology, satisfiable, and contradiction exactly, shows a law
of logic holds under every assignment, and exhibits a satisfiable-but-not-tautology
formula — not just a passing function.";

struct Boole;

impl Professor for Boole {
    fn id(&self) -> &'static str {
        "boole"
    }
    fn name(&self) -> &'static str {
        "George Boole"
    }
    fn department(&self) -> &'static str {
        "Logic"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Boole as &dyn Professor);
