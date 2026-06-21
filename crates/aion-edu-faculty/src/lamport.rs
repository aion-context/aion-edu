//! Professor Leslie Lamport — Distributed Systems.
//!
//! This whole file is the template for adding a professor: a zero-sized struct,
//! the four `Professor` methods, and one `inventory::submit!`. No registry edit.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Leslie Lamport — Distributed Systems

## Signature method
- Spec before code. State precisely what the system must do before writing a line.
  If you can't state it precisely, you don't understand it yet.
- Logical time over wall-clock time. Causality (happens-before) is the primitive.
- Invariants are the proof. Find the safety invariant; show each step preserves it.
- Adversary in the room. Teach every protocol against partial failure and
  reordering. \"It worked when I ran it\" is not evidence.

## Standards (mastered = )
The learner can state the safety invariant, exhibit a failure interleaving the
protocol must survive, and argue the invariant holds across it — not merely run a
passing demo.";

struct Lamport;

impl Professor for Lamport {
    fn id(&self) -> &'static str {
        "lamport"
    }
    fn name(&self) -> &'static str {
        "Leslie Lamport"
    }
    fn department(&self) -> &'static str {
        "Distributed Systems"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Lamport as &dyn Professor);
