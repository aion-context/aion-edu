//! aion-edu-teach — the live teaching loop.
//!
//! A professor (Claude, tool-use) teaches one lesson in a real workspace,
//! reaching the learner through `ask_learner`. The rubric is verified before
//! teaching (provenance gate); mastery yields a signed, rubric-bound credential.

#![forbid(unsafe_code)]

mod client;
mod error;
mod event;
mod ledger;
mod learner;
mod protocol;

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use aion_edu_core::registry;
use client::{text_of, Client};
use learner::SimulatedLearner;

pub use error::{Error, Result};
pub use event::{print_event, Event};
pub use learner::Learner;
pub use ledger::{load as load_ledger, mastered_lessons};

/// A teaching-event sink: the CLI prints, the web entry streams.
pub type Emit<'a> = dyn Fn(&Event) + 'a;

const PROF_MODEL: &str = "claude-opus-4-8";
const MAX_TURNS: usize = 16;

/// Result of teaching one lesson.
#[derive(Debug)]
pub struct Outcome {
    pub mastered: bool,
    pub credential_file_id: Option<u64>,
    pub binding: Option<aion_edu_provenance::Binding>,
}

fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn make_workspace(lesson_id: &str) -> Result<PathBuf> {
    let ws = std::env::temp_dir().join(format!("aion-edu-{lesson_id}-{}", std::process::id()));
    std::fs::create_dir_all(&ws)?;
    Ok(ws)
}

/// Embedded starter assets per lesson — `(filename, contents)`. Adding a lesson's
/// assets is one arm here; the files live under `assets/<lesson>/`.
fn lesson_assets(lesson_id: &str) -> &'static [(&'static str, &'static str)] {
    macro_rules! a {
        ($d:literal, $($f:literal),+) => {
            &[$(($f, include_str!(concat!("../assets/", $d, "/", $f)))),+]
        };
    }
    match lesson_id {
        "cs340-u1-l1" => a!("cs340-u1-l1", "process.py", "test_clock.py"),
        "cs340-u1-l2" => a!("cs340-u1-l2", "process.py", "test_vclock.py"),
        "math110-u1-l1" => a!("math110-u1-l1", "linalg.py", "test_linalg.py"),
        "math110-u1-l2" => a!("math110-u1-l2", "proj.py", "test_proj.py"),
        "math110-u1-l3" => a!("math110-u1-l3", "lstsq.py", "test_lstsq.py"),
        "math110-u2-l1" => a!("math110-u2-l1", "eig.py", "test_eig.py"),
        "phys101-u1-l1" => a!("phys101-u1-l1", "pendulum.py", "test_pendulum.py"),
        "phys101-u1-l2" => a!("phys101-u1-l2", "energy.py", "test_energy.py"),
        "phys101-u1-l3" => a!("phys101-u1-l3", "shm.py", "test_shm.py"),
        "math150-u1-l1" => a!("math150-u1-l1", "tiling.py", "test_tiling.py"),
        "math150-u1-l2" => a!("math150-u1-l2", "parity.py", "test_parity.py"),
        "math150-u1-l3" => a!("math150-u1-l3", "nim.py", "test_nim.py"),
        "cs220-u1-l1" => a!("cs220-u1-l1", "bsearch.py", "test_bsearch.py"),
        "cs250-u1-l1" => a!("cs250-u1-l1", "isqrt.py", "test_isqrt.py"),
        "ee210-u1-l1" => a!("ee210-u1-l1", "hamming.py", "test_hamming.py"),
        "astro101-u1-l1" => a!("astro101-u1-l1", "kepler.py", "test_kepler.py"),
        "cs270-u1-l1" => a!("cs270-u1-l1", "tm.py", "test_tm.py"),
        "math210-u1-l1" => a!("math210-u1-l1", "euler.py", "test_euler.py"),
        "bio101-u1-l1" => a!("bio101-u1-l1", "evolution.py", "test_evolution.py"),
        "gt101-u1-l1" => a!("gt101-u1-l1", "games.py", "test_games.py"),
        "it201-u1-l1" => a!("it201-u1-l1", "entropy.py", "test_entropy.py"),
        "math230-u1-l1" => a!("math230-u1-l1", "gcd.py", "test_gcd.py"),
        "math310-u1-l1" => a!("math310-u1-l1", "group.py", "test_group.py"),
        "logic101-u1-l1" => a!("logic101-u1-l1", "boole.py", "test_boole.py"),
        "sys301-u1-l1" => a!("sys301-u1-l1", "ring.py", "test_ring.py"),
        "sys310-u1-l1" => a!("sys310-u1-l1", "quorum.py", "test_quorum.py"),
        "sys320-u1-l1" => a!("sys320-u1-l1", "bucket.py", "test_bucket.py"),
        "sys330-u1-l1" => a!("sys330-u1-l1", "breaker.py", "test_breaker.py"),
        "cs330-u1-l1" => a!("cs330-u1-l1", "ratnum.py", "test_ratnum.py"),
        "cs440-u1-l1" => a!("cs440-u1-l1", "byzantine.py", "test_byzantine.py"),
        "cs450-u1-l1" => a!("cs450-u1-l1", "linearize.py", "test_linearize.py"),
        _ => &[],
    }
}

fn seed_assets(ws: &Path, lesson_id: &str) -> Result<()> {
    for (name, contents) in lesson_assets(lesson_id) {
        std::fs::write(ws.join(name), contents)?;
    }
    Ok(())
}

fn safe_path(ws: &Path, rel: &str) -> Result<PathBuf> {
    let p = ws.join(rel);
    if !p.starts_with(ws) {
        return Err(Error::Refused("path escapes workspace".into()));
    }
    Ok(p)
}

#[allow(clippy::too_many_arguments)]
fn run_tool(name: &str, input: &Value, ws: &Path, learner: &mut dyn Learner, who: &str, recorded: &mut Option<Value>, emit: &Emit) -> Result<String> {
    match name {
        "write_file" => {
            let p = safe_path(ws, input["path"].as_str().unwrap_or(""))?;
            std::fs::write(&p, input["content"].as_str().unwrap_or("").as_bytes())?;
            Ok("written".into())
        }
        "read_file" => {
            let p = safe_path(ws, input["path"].as_str().unwrap_or(""))?;
            Ok(std::fs::read_to_string(&p).unwrap_or_default())
        }
        "run" => {
            let cmd = input["cmd"].as_str().unwrap_or("");
            let out = Command::new("sh").arg("-c").arg(cmd).current_dir(ws).output()?;
            let rc = out.status.code().unwrap_or(-1);
            emit(&Event::Tool { cmd: cmd.to_string(), rc });
            let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&out.stderr));
            Ok(format!("rc={rc}\n{}", truncate(&s, 2000)))
        }
        "ask_learner" => {
            let m = input["message"].as_str().unwrap_or("");
            emit(&Event::Ask { text: m.to_string() });
            let r = learner.reply(m)?;
            emit(&Event::Learner { who: who.to_string(), text: r.clone() });
            Ok(r)
        }
        "record_mastery" => {
            *recorded = Some(input.clone());
            Ok("recorded".into())
        }
        other => Ok(format!("unknown tool {other}")),
    }
}

fn run_loop(client: &Client, system: &str, lesson: &aion_edu_core::Lesson, ws: &Path, learner: &mut dyn Learner, who: &str, emit: &Emit) -> Result<Option<Value>> {
    let mut messages = vec![json!({"role": "user", "content": protocol::kickoff(lesson)})];
    let tools = protocol::tools();
    for _ in 0..MAX_TURNS {
        let resp = client.message(PROF_MODEL, system, Value::Array(messages.clone()), Some(tools.clone()), 2500)?;
        let text = text_of(&resp);
        if !text.trim().is_empty() {
            emit(&Event::Professor { text });
        }
        messages.push(json!({"role": "assistant", "content": resp["content"].clone()}));
        if resp["stop_reason"] != "tool_use" {
            messages.push(json!({"role": "user", "content": "Continue — use ask_learner, or record_mastery when done."}));
            continue;
        }
        let empty = vec![];
        let mut results = Vec::new();
        let mut recorded = None;
        for block in resp["content"].as_array().unwrap_or(&empty) {
            if block["type"] != "tool_use" {
                continue;
            }
            let out = run_tool(block["name"].as_str().unwrap_or(""), &block["input"], ws, learner, who, &mut recorded, emit)?;
            results.push(json!({"type": "tool_result", "tool_use_id": block["id"].as_str().unwrap_or(""), "content": out}));
        }
        messages.push(json!({"role": "user", "content": results}));
        if recorded.is_some() {
            return Ok(recorded);
        }
    }
    Ok(None)
}

/// Teach one lesson with the autonomous simulated learner (CLI default).
pub fn run_lesson_simulated(data_dir: &Path, lesson_id: &str, learner: &str, emit: &Emit) -> Result<Outcome> {
    let client = Client::from_env()?;
    let mut sim = SimulatedLearner::new(&client, learner, lesson_id);
    run_lesson(data_dir, lesson_id, learner, &mut sim, emit)
}

/// Teach one lesson end-to-end, emitting events through `emit`, with the given
/// `learner` channel (simulated or human): verify the rubric (+ quorum), run the
/// loop, and on mastery record the ledger and issue a signed, rubric-bound credential.
pub fn run_lesson(data_dir: &Path, lesson_id: &str, learner: &str, chan: &mut dyn Learner, emit: &Emit) -> Result<Outcome> {
    let (course, lesson) = registry::find_lesson(lesson_id)
        .ok_or_else(|| Error::Refused(format!("unknown lesson {lesson_id}")))?;
    emit(&Event::LessonStart { lesson: lesson_id.to_string() });
    let sealed = aion_edu_provenance::verify_rubric(data_dir, &course.professor, lesson_id)?
        .ok_or_else(|| Error::Refused(format!("rubric {lesson_id} not sealed — run `aion-edu seal`")))?;
    if !sealed.valid {
        return Err(Error::Refused(format!("rubric {lesson_id} failed verification")));
    }
    emit(&Event::Status { text: format!("rubric provenance VERIFIED (file_id={} v{})", sealed.file_id, sealed.version) });
    if let Some(q) = aion_edu_provenance::verify_quorum(data_dir, &course, &lesson)? {
        if !q.met {
            return Err(Error::Refused(format!(
                "rubric {lesson_id} quorum {}/{} not met — refusing to teach",
                q.valid_count, q.threshold
            )));
        }
        emit(&Event::Status { text: format!("governance quorum {}/{} met", q.valid_count, q.threshold) });
    }
    let prof = registry::professor(&course.professor)
        .ok_or_else(|| Error::Refused(format!("no professor {}", course.professor)))?;

    let ws = make_workspace(lesson_id)?;
    seed_assets(&ws, lesson_id)?;
    let client = Client::from_env()?;
    let system = format!("{}\n\n{}", prof.persona(), protocol::UNIVERSAL_PROTOCOL);

    let recorded = run_loop(&client, &system, &lesson, &ws, chan, learner, emit)?;
    let _ = std::fs::remove_dir_all(&ws);

    let Some(rec) = recorded else {
        emit(&Event::Result { mastered: false, credential_file_id: None, binding_verified: false });
        return Ok(Outcome { mastered: false, credential_file_id: None, binding: None });
    };
    ledger::record(data_dir, learner, lesson_id, &lesson.mastery_outcomes)?;
    let summary = rec["summary"].as_str().unwrap_or("mastered");
    let rref = aion_edu_provenance::RubricRef { file_id: sealed.file_id, version: sealed.version };
    let cred = aion_edu_provenance::commit_credential(data_dir, learner, &course.professor, lesson_id, summary, rref)?;
    let binding = aion_edu_provenance::verify_binding(data_dir, &course.professor, learner, lesson_id)?;
    let binding_verified = binding.map(|b| b.credential_valid && b.lineage_match).unwrap_or(false);
    let mastered = rec["overall_mastered"].as_bool().unwrap_or(false);
    emit(&Event::Result { mastered, credential_file_id: Some(cred), binding_verified });
    Ok(Outcome { mastered, credential_file_id: Some(cred), binding })
}
