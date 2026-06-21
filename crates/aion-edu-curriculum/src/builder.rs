//! Small builders so each course file stays declarative.

use aion_edu_core::{Lesson, Practice, RubricCriterion, Unit};

pub(crate) fn s(x: &str) -> String {
    x.to_string()
}

pub(crate) fn unit(id: &str, title: &str, lessons: Vec<Lesson>) -> Unit {
    Unit { id: s(id), title: s(title), lessons }
}

pub(crate) fn crit(outcome: &str, criterion: &str) -> RubricCriterion {
    RubricCriterion { outcome: s(outcome), criterion: s(criterion) }
}

pub(crate) fn practice(prompt: &str, starter: &[&str], verify: &str) -> Practice {
    Practice {
        prompt: s(prompt),
        starter_files: starter.iter().map(|f| s(f)).collect(),
        verify: s(verify),
    }
}

/// Assemble a lesson. `rubric` pairs each outcome with its criterion.
pub(crate) fn lesson(
    id: &str,
    title: &str,
    outcomes: &[&str],
    brief: &str,
    prereqs: &[&str],
    practice: Practice,
    rubric: Vec<RubricCriterion>,
) -> Lesson {
    Lesson {
        id: s(id),
        title: s(title),
        mastery_outcomes: outcomes.iter().map(|o| s(o)).collect(),
        tutor_brief: s(brief),
        prerequisites: prereqs.iter().map(|p| s(p)).collect(),
        practice,
        rubric,
    }
}
