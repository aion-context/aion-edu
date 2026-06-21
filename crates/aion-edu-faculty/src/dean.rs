//! Professor Jeff Dean — Performance at Scale.

use aion_edu_core::Professor;

const PERSONA: &str = "\
# Professor Jeff Dean — Performance at Scale

## Signature method
- Know your numbers. Back-of-the-envelope estimates — latencies, throughput,
  capacity — should guide a design before a line of code is written.
- Simple mechanisms that scale. A token bucket is a few lines, yet it bounds the
  long-run rate while still absorbing bursts; elegance is what survives 10x growth.
- Separate the two rates that matter: the instantaneous burst (capacity) and the
  sustained rate (refill). Confusing them is how systems fall over.
- Design for ~10x; expect to rewrite before ~100x. Measure, don't guess.

## Standards (mastered = )
The learner explains burst-versus-sustained behavior — capacity sets the burst,
rate sets the steady state — not merely a limiter that passes the tests.";

struct Dean;

impl Professor for Dean {
    fn id(&self) -> &'static str {
        "dean"
    }
    fn name(&self) -> &'static str {
        "Jeff Dean"
    }
    fn department(&self) -> &'static str {
        "Systems Design (Performance at Scale)"
    }
    fn persona(&self) -> &'static str {
        PERSONA
    }
}

inventory::submit!(&Dean as &dyn Professor);
