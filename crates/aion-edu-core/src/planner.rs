//! Prerequisite routing — resolve "teach me X" into an ordered lesson path.
//!
//! Resolve a target (lesson id | course id | free-text concept) → lessons,
//! expand the prerequisite closure, topologically order it (a lesson never
//! precedes its prereqs), drop already-mastered lessons, and surface external
//! (course-level) prerequisites not present in the catalog.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{registry, Course, Lesson};

/// The computed learning path.
#[derive(Debug, Clone)]
pub struct Plan {
    pub target: String,
    pub resolved: Vec<String>,
    pub path: Vec<String>,
    pub skipped: Vec<String>,
    pub external_prereqs: Vec<String>,
}

type Index = BTreeMap<String, (Course, Lesson)>;

fn index() -> Index {
    let mut idx = Index::new();
    for course in registry::courses() {
        for lesson in course.all_lessons() {
            idx.insert(lesson.id.clone(), (course.clone(), lesson.clone()));
        }
    }
    idx
}

/// Resolve a target string to target lesson ids.
pub fn resolve_target(target: &str, idx: &Index) -> Vec<String> {
    if idx.contains_key(target) {
        return vec![target.to_string()];
    }
    let by_course: Vec<String> = idx
        .values()
        .filter(|(c, _)| c.id == target)
        .map(|(_, l)| l.id.clone())
        .collect();
    if !by_course.is_empty() {
        return by_course;
    }
    idx.iter()
        .filter(|(_, (_, l))| l.matches_concept(target))
        .map(|(id, _)| id.clone())
        .collect()
}

/// Depth-first prerequisite closure; records external (non-catalog) prereqs.
fn closure(targets: &[String], idx: &Index) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut needed = BTreeSet::new();
    let mut external = BTreeSet::new();
    let mut stack: Vec<String> = targets.to_vec();
    while let Some(id) = stack.pop() {
        if !needed.insert(id.clone()) {
            continue;
        }
        let (course, lesson) = &idx[&id];
        for pre in &lesson.prerequisites {
            if idx.contains_key(pre) {
                stack.push(pre.clone());
            } else {
                external.insert(pre.clone());
            }
        }
        for pre in &course.prerequisites {
            if !idx.contains_key(pre) {
                external.insert(pre.clone());
            }
        }
    }
    (needed, external)
}

/// Kahn topological sort over the induced subgraph (prereq → lesson).
fn topo_order(needed: &BTreeSet<String>, idx: &Index) -> Vec<String> {
    let mut deps: BTreeMap<String, BTreeSet<String>> = needed
        .iter()
        .map(|id| {
            let d = idx[id]
                .1
                .prerequisites
                .iter()
                .filter(|p| needed.contains(*p))
                .cloned()
                .collect();
            (id.clone(), d)
        })
        .collect();
    let mut ready: VecDeque<String> =
        deps.iter().filter(|(_, d)| d.is_empty()).map(|(k, _)| k.clone()).collect();
    let mut order = Vec::new();
    while let Some(n) = ready.pop_front() {
        order.push(n.clone());
        for (m, d) in deps.iter_mut() {
            if d.remove(&n) && d.is_empty() && !order.contains(m) && !ready.contains(m) {
                ready.push_back(m.clone());
            }
        }
    }
    for id in needed {
        if !order.contains(id) {
            order.push(id.clone()); // cycle fallback — deterministic
        }
    }
    order
}

/// Plan an ordered path to `target`, skipping lessons in `mastered`.
pub fn plan(target: &str, mastered: &BTreeSet<String>) -> Plan {
    let idx = index();
    let resolved = resolve_target(target, &idx);
    if resolved.is_empty() {
        return Plan {
            target: target.to_string(),
            resolved,
            path: vec![],
            skipped: vec![],
            external_prereqs: vec![],
        };
    }
    let (needed, external) = closure(&resolved, &idx);
    let order = topo_order(&needed, &idx);
    let (mut path, mut skipped) = (Vec::new(), Vec::new());
    for id in order {
        if mastered.contains(&id) {
            skipped.push(id);
        } else {
            path.push(id);
        }
    }
    Plan {
        target: target.to_string(),
        resolved,
        path,
        skipped,
        external_prereqs: external.into_iter().collect(),
    }
}
