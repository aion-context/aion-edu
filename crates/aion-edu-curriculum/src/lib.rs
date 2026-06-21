//! aion-edu-curriculum — the concrete courses.
//!
//! Each course is one module ending in `inventory::submit!(CourseRegistration …)`.
//! To add a course: create `src/<id>.rs`, build a [`aion_edu_core::Course`] with
//! the [`builder`] helpers, submit it, and add a `mod` line here. The kernel
//! discovers it automatically.

#![forbid(unsafe_code)]

mod astro101;
mod bio101;
mod builder;
mod cs220;
mod cs250;
mod cs270;
mod cs330;
mod cs340;
mod cs440;
mod cs450;
mod ee210;
mod gt101;
mod it201;
mod logic101;
mod math110;
mod math150;
mod math210;
mod math230;
mod math310;
mod phys101;
mod sys301;
mod sys310;
mod sys320;
mod sys330;

#[cfg(test)]
mod tests {
    use aion_edu_core::{planner, registry};
    use std::collections::BTreeSet;

    #[test]
    fn all_four_departments_register() {
        let ids: Vec<String> = registry::courses().into_iter().map(|c| c.id).collect();
        for c in ["cs340", "math110", "phys101", "math150"] {
            assert!(ids.contains(&c.to_string()), "{c} must self-register");
        }
    }

    #[test]
    fn math110_u1_is_a_three_lesson_chain() {
        let p = planner::plan("math110-u1-l3", &BTreeSet::new());
        assert_eq!(p.path, vec!["math110-u1-l1", "math110-u1-l2", "math110-u1-l3"]);
    }

    #[test]
    fn math110_has_a_second_unit() {
        // the u2 eigenvalues lesson lives in a different unit, prereq u1-l1
        let course = registry::courses().into_iter().find(|c| c.id == "math110").unwrap();
        assert_eq!(course.units.len(), 2, "math110 has two units");
        assert_eq!(course.units[1].id, "math110-u2");
        let p = planner::plan("math110-u2-l1", &BTreeSet::new());
        assert_eq!(p.path, vec!["math110-u1-l1", "math110-u2-l1"]);
    }

    #[test]
    fn concept_routes_through_prereq_chain() {
        // "XOR" uniquely names math150-u1-l3 ("Nim and the XOR Invariant");
        // its prereq chain comes first. (Note: "nim" would also match "miNIMax".)
        let p = planner::plan("XOR", &BTreeSet::new());
        assert_eq!(p.resolved, vec!["math150-u1-l3"]);
        assert_eq!(p.path, vec!["math150-u1-l1", "math150-u1-l2", "math150-u1-l3"]);
    }

    #[test]
    fn plan_skips_mastered() {
        let mastered: BTreeSet<String> = ["math110-u1-l1".to_string()].into_iter().collect();
        let p = planner::plan("math110-u1-l3", &mastered);
        assert_eq!(p.skipped, vec!["math110-u1-l1"]);
        assert_eq!(p.path, vec!["math110-u1-l2", "math110-u1-l3"]);
    }
}
