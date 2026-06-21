//! Per-learner mastery ledger (flat JSON). The transcript and the source of
//! truth for "what has this learner mastered".

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ledger {
    pub learner: String,
    /// outcome -> state ("mastered").
    pub outcomes: BTreeMap<String, String>,
    /// lesson id -> mastered?
    pub lessons: BTreeMap<String, bool>,
}

fn path(dir: &Path, learner: &str) -> std::path::PathBuf {
    let safe: String = learner.chars().filter(|c| c.is_alphanumeric() || "-_.".contains(*c)).collect();
    dir.join("ledger").join(format!("{safe}.json"))
}

pub fn load(dir: &Path, learner: &str) -> Ledger {
    let p = path(dir, learner);
    std::fs::read(&p)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_else(|| Ledger { learner: learner.to_string(), ..Default::default() })
}

/// Which of `lesson_outcomes` this learner has already mastered.
pub fn mastered_lessons(dir: &Path, learner: &str) -> BTreeSet<String> {
    load(dir, learner)
        .lessons
        .into_iter()
        .filter(|(_, m)| *m)
        .map(|(k, _)| k)
        .collect()
}

pub fn record(dir: &Path, learner: &str, lesson_id: &str, outcomes: &[String]) -> Result<()> {
    let mut led = load(dir, learner);
    for o in outcomes {
        led.outcomes.insert(o.clone(), "mastered".to_string());
    }
    led.lessons.insert(lesson_id.to_string(), true);
    let p = path(dir, learner);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&p, serde_json::to_vec_pretty(&led)?)?;
    Ok(())
}
