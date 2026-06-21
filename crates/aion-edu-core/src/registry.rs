//! The faculty + curriculum registries — assembled from `inventory` at startup.
//!
//! There is no list to maintain: professors register as `&'static dyn Professor`
//! and courses register a builder fn. Enumeration just iterates the collection.

use crate::{Course, Lesson, Professor};

/// A self-registering course. Curriculum files submit one of these; the builder
/// returns owned [`Course`] data (which cannot be fully `const`).
pub struct CourseRegistration {
    pub build: fn() -> Course,
}

inventory::collect!(CourseRegistration);

/// All registered professors.
pub fn professors() -> Vec<&'static dyn Professor> {
    inventory::iter::<&'static dyn Professor>().copied().collect()
}

/// Look up a professor by id.
pub fn professor(id: &str) -> Option<&'static dyn Professor> {
    inventory::iter::<&'static dyn Professor>()
        .copied()
        .find(|p| p.id() == id)
}

/// All registered courses (rebuilt fresh from their providers).
pub fn courses() -> Vec<Course> {
    inventory::iter::<CourseRegistration>()
        .map(|r| (r.build)())
        .collect()
}

/// Find the course + lesson for a lesson id.
pub fn find_lesson(lesson_id: &str) -> Option<(Course, Lesson)> {
    for course in courses() {
        if let Some(l) = course.all_lessons().find(|l| l.id == lesson_id) {
            return Some((course.clone(), l.clone()));
        }
    }
    None
}
