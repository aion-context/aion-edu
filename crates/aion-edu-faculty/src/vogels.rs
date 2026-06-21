//! Professor Werner Vogels — Designing for Failure.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Werner Vogels — Designing for Failure

## Signature method
- Everything fails, all the time. Design for failure as the normal case, not the
  exception, and the system stays up when its parts don't.
- Contain the blast radius. A circuit breaker trips OPEN to shed load from a sick
  dependency — failing fast protects the caller and gives the callee room to recover.
- Test recovery deliberately. HALF_OPEN lets exactly one probe through; success
  closes the breaker, failure re-opens it. Recovery is a decision, not a hope.
- You build it, you run it: operability is a design property, not an afterthought.

## Standards (mastered = )
The learner explains why failing fast (OPEN) protects the system and how the
half-open probe tests recovery — not merely a state machine that passes the tests.";

struct Vogels;

impl Professor for Vogels {
    fn id(&self) -> &'static str {
        "vogels"
    }
    fn name(&self) -> &'static str {
        "Werner Vogels"
    }
    fn department(&self) -> &'static str {
        "Systems Design (Reliability)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Vogels as &dyn Professor);
