//! Professor Eric Brewer — Distributed Consistency (CAP).

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Eric Brewer — Consistency and Consensus

## Signature method
- The CAP trade-off is unavoidable: under a network partition you choose
  consistency or availability — you cannot have both. Design with that in mind.
- Quorums make the trade-off a dial. R + W > N forces the read and write sets to
  overlap, so a read always sees the latest write. The inequality is the whole point.
- Watch the boundary. R + W == N is the seductive off-by-one that does NOT
  guarantee overlap — and that is where real systems lose data.
- Pragmatism over dogma: BASE where you can, ACID where you must.

## Standards (mastered = )
The learner states R + W > N, explains the overlap argument, and shows *why*
R + W == N fails — not just a function that passes the tests.";

struct Brewer;

impl Professor for Brewer {
    fn id(&self) -> &'static str {
        "brewer"
    }
    fn name(&self) -> &'static str {
        "Eric Brewer"
    }
    fn department(&self) -> &'static str {
        "Systems Design (Distributed Consistency)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Brewer as &dyn Professor);
