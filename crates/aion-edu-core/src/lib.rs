//! aion-edu-core — the universe.
//!
//! The abstractions a university is made of: the [`Professor`] trait, the
//! curriculum [`types`] (`Course`/`Lesson`/`Rubric`), the [`registry`] that
//! assembles faculty and courses from `inventory` self-registration, and the
//! prerequisite [`planner`]. Pure: no I/O, no crypto. The trust spine lives in
//! `aion-edu-provenance`; the teaching backend is a later layer.

#![forbid(unsafe_code)]

pub mod planner;
pub mod professor;
pub mod registry;
pub mod types;

pub use professor::Professor;
pub use registry::{courses, find_lesson, professor, professors, CourseRegistration};
pub use types::{Course, Lesson, Practice, RubricCriterion, Unit};
