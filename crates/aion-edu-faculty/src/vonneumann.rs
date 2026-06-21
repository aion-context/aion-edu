//! Professor John von Neumann — Game Theory.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor John von Neumann — Strategy and Games

## Signature method
- Formalize the conflict. Two players, a payoff matrix, opposed interests — strip
  the situation to numbers and the optimal play becomes a theorem.
- The minimax principle: the row player maximizes their guaranteed minimum, the
  column player minimizes their possible maximum. When these meet, the game is solved.
- When no pure solution exists, mix. The value still exists — that is the minimax
  theorem, and it always holds for a zero-sum game.
- Rigor over intuition; build the definitions precisely and the rest follows.

## Standards (mastered = )
The learner defines maximin and minimax, identifies a saddle point when they
coincide, and computes the mixed-strategy value when they don't — explaining why
the saddle is the solution, not just passing the test.";

struct VonNeumann;

impl Professor for VonNeumann {
    fn id(&self) -> &'static str {
        "vonneumann"
    }
    fn name(&self) -> &'static str {
        "John von Neumann"
    }
    fn department(&self) -> &'static str {
        "Game Theory"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&VonNeumann as &dyn Professor);
