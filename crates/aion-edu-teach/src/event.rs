//! Structured teaching events — emitted through a callback so the same loop
//! drives the CLI (print) and the web entry (stream over SSE).

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    /// A status line (provenance gate, quorum, etc.).
    Status { text: String },
    /// A new lesson is starting.
    LessonStart { lesson: String },
    /// The professor said something.
    Professor { text: String },
    /// The professor asked the learner.
    Ask { text: String },
    /// The harness is waiting for the human learner's reply.
    AwaitInput,
    /// The learner answered.
    Learner { who: String, text: String },
    /// A workspace command ran.
    Tool { cmd: String, rc: i32 },
    /// The lesson finished.
    Result {
        mastered: bool,
        credential_file_id: Option<u64>,
        binding_verified: bool,
    },
}

/// A printing emitter for the CLI.
pub fn print_event(e: &Event) {
    match e {
        Event::Status { text } => println!("[ {text} ]"),
        Event::LessonStart { lesson } => println!("\n===== lesson {lesson} ====="),
        Event::Professor { text } => println!("\n👤 prof: {text}"),
        Event::Ask { text } => println!("   ❓ prof→learner: {text}"),
        Event::AwaitInput => {}
        Event::Learner { who, text } => println!("   🎓 {who}: {text}"),
        Event::Tool { cmd, rc } => println!("   ⚙  run: {cmd}  ->  rc={rc}"),
        Event::Result { mastered, credential_file_id, binding_verified } => println!(
            "\n[ mastered={mastered}  credential_file_id={credential_file_id:?}  binding_verified={binding_verified} ]"
        ),
    }
}
