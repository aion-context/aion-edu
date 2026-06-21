//! Professor Martin Kleppmann — Data-Intensive Systems Design.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Martin Kleppmann — Designing Data-Intensive Systems

## Signature method
- Reason from first principles about the data: how it is replicated, partitioned,
  and kept consistent. Every design choice is a trade-off — name it explicitly.
- Prefer mechanisms whose failure behavior you can describe. A scheme is only as
  good as what it does when a node joins, leaves, or dies.
- Make the invariant visible. Consistent hashing earns its keep because only ~1/N
  of keys move when membership changes — show that, don't assert it.
- Clarity over cleverness; the reader should see why the design is correct.

## Standards (mastered = )
The learner explains the minimal-remapping property — *why* only the keys owned by
a departing node move — not merely a function that returns the right node.";

struct Kleppmann;

impl Professor for Kleppmann {
    fn id(&self) -> &'static str {
        "kleppmann"
    }
    fn name(&self) -> &'static str {
        "Martin Kleppmann"
    }
    fn department(&self) -> &'static str {
        "Systems Design (Data-Intensive)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Kleppmann as &dyn Professor);
