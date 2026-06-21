//! The universal teaching protocol, tool schemas, and the lesson kickoff.
//!
//! The persona supplies *how* to teach; this supplies the loop, the tool
//! contract, and the workspace conventions — identical for every professor.

use serde_json::{json, Value};

use aion_edu_core::Lesson;

pub const UNIVERSAL_PROTOCOL: &str = "\
# Your role at aion-edu

You teach ONE lesson per session, toward its mastery outcomes, in your signature
method, inside a live workspace. Reach the learner ONLY through `ask_learner`.
Act in the workspace via `write_file` / `read_file` / `run` (the starter files are
already present). Run the lesson's verify command to check the learner's work.

## The loop
1. Diagnose briefly.
2. Teach toward the masteryOutcomes and NOTHING else, compressed.
3. Practice: have the LEARNER produce the work; write their submission to the
   file and run the verify command. Show them what happened.
4. Assess each outcome against its rubric criterion using the artifact + verify
   output as evidence. A green test alone is NOT sufficient when a criterion
   calls for a spoken argument — require it.
5. Gate: when (and only when) ALL outcomes are met, call `record_mastery` with a
   per-outcome verdict and verbatim evidence. Otherwise remediate ONE outcome and
   loop; do not call the tool yet.

End your turn after each teaching beat so the learner can respond.";

/// The five classroom tools (the harness fulfils each).
pub fn tools() -> Value {
    json!([
        { "name": "write_file", "description": "Write a file in the workspace.",
          "input_schema": { "type": "object",
            "properties": { "path": {"type":"string"}, "content": {"type":"string"} },
            "required": ["path","content"] } },
        { "name": "read_file", "description": "Read a file from the workspace.",
          "input_schema": { "type": "object",
            "properties": { "path": {"type":"string"} }, "required": ["path"] } },
        { "name": "run", "description": "Run a shell command in the workspace.",
          "input_schema": { "type": "object",
            "properties": { "cmd": {"type":"string"} }, "required": ["cmd"] } },
        { "name": "ask_learner",
          "description": "Say something to the learner and get their reply. The ONLY channel to the learner.",
          "input_schema": { "type": "object",
            "properties": { "message": {"type":"string"} }, "required": ["message"] } },
        { "name": "record_mastery",
          "description": "Record the lesson outcome to the mastery ledger. Call once, only when every outcome is met.",
          "input_schema": { "type": "object",
            "properties": {
              "lesson_id": {"type":"string"},
              "overall_mastered": {"type":"boolean"},
              "outcomes": { "type":"array", "items": { "type":"object",
                "properties": { "outcome": {"type":"string"}, "met": {"type":"boolean"}, "evidence": {"type":"string"} },
                "required": ["outcome","met","evidence"] } },
              "summary": {"type":"string"}
            },
            "required": ["lesson_id","overall_mastered","outcomes","summary"] } }
    ])
}

/// The kickoff user message: the rules plus the lesson spec.
pub fn kickoff(lesson: &Lesson) -> String {
    let spec = serde_json::to_string_pretty(lesson).unwrap_or_default();
    format!(
        "Begin the lesson. Reach the learner only via `ask_learner`; act in the \
         workspace via write_file/read_file/run; run `practice.verify` to check \
         the learner's work; call record_mastery only when every outcome is met.\n\n\
         LESSON SPEC:\n```json\n{spec}\n```"
    )
}
