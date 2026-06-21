//! The `Professor` trait and its inventory collection.
//!
//! A professor is the "teach" step of the pedagogy loop, in one master's voice.
//! It is stateless — method, voice, and standards expressed as `&'static str`.
//! Concrete professors live in `aion-edu-faculty`, one file each, and register
//! themselves with a single `inventory::submit!(&X as &dyn Professor)`.

/// A master-teacher persona. Implementors are zero-sized and `'static`.
pub trait Professor: Sync {
    /// Stable id, matches [`crate::Course::professor`] (e.g. `"lamport"`).
    fn id(&self) -> &'static str;
    /// The human this persona distills (e.g. `"Leslie Lamport"`).
    fn name(&self) -> &'static str;
    /// Department / field (e.g. `"Distributed Systems"`).
    fn department(&self) -> &'static str;
    /// The persona contract: signature method, voice, standards. Fed to the
    /// teaching backend as the system prompt, ahead of the universal protocol.
    fn persona(&self) -> &'static str;
}

inventory::collect!(&'static dyn Professor);
