//! Curriculum data model — `Course → Lesson → Rubric`.
//!
//! Mirrors the Sigma schema: a lesson targets ≤3 mastery outcomes, carries a
//! minimal tutor brief (how to teach, not a script), a runnable `practice`
//! (artifact + verify command), and a `rubric` of one criterion per outcome.

use serde::{Deserialize, Serialize};

/// The artifact a learner produces in the classroom, plus the check the harness
/// runs to grade it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practice {
    pub prompt: String,
    #[serde(default)]
    pub starter_files: Vec<String>,
    /// Command the harness runs to verify the work (e.g. `pytest -q test.py`).
    pub verify: String,
}

/// One observable grading criterion, bound to a single mastery outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricCriterion {
    pub outcome: String,
    pub criterion: String,
}

/// A single teachable unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    /// What the learner can DO afterward (≤3, concept-named).
    pub mastery_outcomes: Vec<String>,
    /// Minimal context for the professor on HOW to teach this lesson.
    pub tutor_brief: String,
    /// Lesson ids that must be mastered first.
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub practice: Practice,
    pub rubric: Vec<RubricCriterion>,
}

/// A unit groups related lessons within a course (e.g. "Logical Time").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub title: String,
    pub lessons: Vec<Lesson>,
}

/// A course taught by one professor, organized into units.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    /// The id of the [`crate::Professor`] who teaches it.
    pub professor: String,
    /// External course ids assumed mastered before this course.
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub units: Vec<Unit>,
}

impl Course {
    /// Every lesson in the course, in unit then lesson order.
    pub fn all_lessons(&self) -> impl Iterator<Item = &Lesson> {
        self.units.iter().flat_map(|u| u.lessons.iter())
    }
}

impl Lesson {
    /// True if `needle` (lowercased) matches the title or any mastery outcome —
    /// used by the planner to resolve free-text concept targets.
    pub fn matches_concept(&self, needle: &str) -> bool {
        let n = needle.to_lowercase();
        self.title.to_lowercase().contains(&n)
            || self.mastery_outcomes.iter().any(|o| o.to_lowercase().contains(&n))
    }
}
