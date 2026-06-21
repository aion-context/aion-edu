//! aion-edu-web — the web entry.
//!
//! A thin axum app over the kernel. Catalog + prerequisite planning, and live
//! teaching streamed over Server-Sent Events. The learner is **you**: when the
//! professor asks, the loop blocks until you POST a reply (`interactive` mode);
//! `auto` mode lets the simulated student drive a demo.

#![forbid(unsafe_code)]

use std::collections::{BTreeSet, HashMap};
use std::convert::Infallible;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use axum::extract::{Path as AxPath, Query};
use axum::http::{header, StatusCode};
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

use aion_edu_core::{planner, registry};
use aion_edu_teach::{Event, Learner};

const DATA_DIR: &str = "aion-edu-data";

// ── interactive sessions ─────────────────────────────────────────────────────
/// Finished sessions, and sessions older than this, are garbage-collected.
const SESSION_TTL: Duration = Duration::from_secs(3600);

struct SessionEntry {
    events_rx: Mutex<Option<UnboundedReceiver<Value>>>,
    reply_tx: UnboundedSender<String>,
    created: Instant,
    /// Set true by the worker when teaching ends (done / halt / error).
    finished: Arc<AtomicBool>,
}

fn sessions() -> &'static Mutex<HashMap<String, Arc<SessionEntry>>> {
    static S: OnceLock<Mutex<HashMap<String, Arc<SessionEntry>>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_sid() -> String {
    static N: AtomicU64 = AtomicU64::new(1);
    format!("s{}", N.fetch_add(1, Ordering::Relaxed))
}

/// Drop finished or expired sessions. Called on every new session — so the map
/// is bounded by live sessions plus any not yet swept.
fn gc() {
    if let Ok(mut map) = sessions().lock() {
        map.retain(|_, e| !e.finished.load(Ordering::Relaxed) && e.created.elapsed() < SESSION_TTL);
    }
}

/// A human learner: emits `AwaitInput`, then blocks on the reply channel.
struct HumanLearner<'a> {
    emit: &'a dyn Fn(&Event),
    reply_rx: &'a mut UnboundedReceiver<String>,
}

impl Learner for HumanLearner<'_> {
    fn reply(&mut self, _prof_message: &str) -> aion_edu_teach::Result<String> {
        (self.emit)(&Event::AwaitInput);
        Ok(self
            .reply_rx
            .blocking_recv()
            .unwrap_or_else(|| "(the learner stepped away — please wrap up)".to_string()))
    }
}

/// Run the teaching driver, then mark the session finished (so GC can reap it).
fn worker(target: String, learner: String, mode: String, events_tx: UnboundedSender<Value>, reply_rx: UnboundedReceiver<String>, finished: Arc<AtomicBool>) {
    run_worker(target, learner, mode, events_tx, reply_rx);
    finished.store(true, Ordering::Relaxed);
}

/// Blocking teaching driver: plan the path, teach each lesson, stream events.
fn run_worker(target: String, learner: String, mode: String, events_tx: UnboundedSender<Value>, mut reply_rx: UnboundedReceiver<String>) {
    let emit_val = |v: Value| {
        let _ = events_tx.send(v);
    };
    let emit = |e: &Event| emit_val(serde_json::to_value(e).unwrap_or(Value::Null));

    let data_dir = Path::new(DATA_DIR);
    let p = planner::plan(&target, &aion_edu_teach::mastered_lessons(data_dir, &learner));
    emit_val(json!({"kind": "plan", "path": p.path, "resolved": p.resolved}));
    if p.path.is_empty() {
        emit_val(json!({"kind": "done", "text": "Nothing to teach (already mastered, or not found)."}));
        return;
    }
    for lid in &p.path {
        let res = if mode == "auto" {
            aion_edu_teach::run_lesson_simulated(data_dir, lid, &learner, &emit)
        } else {
            let mut human = HumanLearner { emit: &emit, reply_rx: &mut reply_rx };
            aion_edu_teach::run_lesson(data_dir, lid, &learner, &mut human, &emit)
        };
        match res {
            Ok(o) if o.mastered => {}
            Ok(_) => {
                emit_val(json!({"kind": "halt", "text": format!("{lid}: not mastered")}));
                return;
            }
            Err(e) => {
                emit_val(json!({"kind": "error", "text": e.to_string()}));
                return;
            }
        }
    }
    emit_val(json!({"kind": "done", "text": "Path complete — all lessons mastered."}));
}

// ── handlers ─────────────────────────────────────────────────────────────────
async fn landing() -> Html<&'static str> {
    Html(include_str!("../web/landing.html"))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../web/index.html"))
}

/// The shared design-system stylesheet.
async fn app_css() -> Response {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], include_str!("../web/app.css")).into_response()
}

/// The Aion Seal — favicon / served mark.
async fn favicon() -> Response {
    ([(header::CONTENT_TYPE, "image/svg+xml")], include_str!("../web/logo.svg")).into_response()
}

/// Serve narration assets (manifest + generated MP3 clips) from a runtime
/// `./narration/` directory, so audio can be regenerated without rebuilding.
async fn narration(AxPath(name): AxPath<String>) -> Response {
    let safe = !name.is_empty()
        && name.len() <= 64
        && !name.contains("..")
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'));
    if !safe {
        return (StatusCode::BAD_REQUEST, "bad name").into_response();
    }
    match std::fs::read(Path::new("narration").join(&name)) {
        Ok(bytes) => {
            let ct = if name.ends_with(".mp3") {
                "audio/mpeg"
            } else if name.ends_with(".json") {
                "application/json"
            } else {
                "application/octet-stream"
            };
            ([(header::CONTENT_TYPE, ct)], bytes).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "no such narration asset").into_response(),
    }
}

async fn catalog() -> Json<Value> {
    let courses: Vec<Value> = registry::courses()
        .into_iter()
        .map(|c| {
            json!({
                "id": c.id, "title": c.title, "professor": c.professor,
                "lessons": c.all_lessons().map(|l| json!({"id": l.id, "title": l.title})).collect::<Vec<_>>(),
            })
        })
        .collect();
    Json(json!(courses))
}

#[derive(Deserialize)]
struct PlanQuery {
    target: String,
}

async fn plan(Query(q): Query<PlanQuery>) -> Json<Value> {
    let p = planner::plan(&q.target, &BTreeSet::new());
    Json(json!({
        "target": p.target, "resolved": p.resolved, "path": p.path,
        "skipped": p.skipped, "externalPrereqs": p.external_prereqs,
    }))
}

// ── student onboarding & transcript ──────────────────────────────────────────
#[derive(Deserialize)]
struct EnrollBody {
    student: String,
    target: String,
}

/// Enroll a student (cryptographic identity) and return their guided path with
/// the teaching professor and mastery marker for each lesson (progress + resume).
async fn student_enroll(Json(b): Json<EnrollBody>) -> Json<Value> {
    let dir = Path::new(DATA_DIR);
    let enrolled = aion_edu_provenance::enroll(dir, &b.student, &b.target).ok();
    let mastered = aion_edu_teach::mastered_lessons(dir, &b.student);
    let full = planner::plan(&b.target, &BTreeSet::new());
    let path: Vec<Value> = full
        .path
        .iter()
        .map(|lid| {
            let (title, prof) = registry::find_lesson(lid).map(|(c, l)| (l.title, c.professor)).unwrap_or_default();
            json!({"lesson": lid, "title": title, "professor": prof, "mastered": mastered.contains(lid)})
        })
        .collect();
    let done = path.iter().filter(|v| v["mastered"].as_bool().unwrap_or(false)).count();
    let key_fp: String = enrolled
        .as_ref()
        .map(|e| e.public_key.iter().take(6).map(|x| format!("{x:02x}")).collect())
        .unwrap_or_default();
    Json(json!({
        "ok": enrolled.is_some() && !full.path.is_empty(),
        "student": b.student,
        "target": b.target,
        "key_fp": key_fp,
        "epoch": enrolled.as_ref().map(|e| e.enrolled_epoch).unwrap_or(0),
        "resolved": full.resolved,
        "external_prereqs": full.external_prereqs,
        "path": path,
        "done": done,
        "total": full.path.len(),
    }))
}

#[derive(Deserialize)]
struct StudentQuery {
    student: String,
}

/// A student's verifiable transcript — every credential they hold, re-verified.
async fn student_transcript(Query(q): Query<StudentQuery>) -> Json<Value> {
    let entries: Vec<Value> = aion_edu_provenance::transcript(Path::new(DATA_DIR), &q.student)
        .unwrap_or_default()
        .iter()
        .map(|e| json!({"lesson": e.lesson_id, "professor": e.professor, "file_id": e.credential_file_id, "valid": e.credential_valid, "bound": e.bound_to_current_rubric}))
        .collect();
    Json(json!({"student": q.student, "credentials": entries}))
}

#[derive(Deserialize)]
struct DiplomaQuery {
    student: String,
    lesson: String,
}

/// Download a portable, self-contained diploma (the verified facts + the raw
/// signed credential + the professor key) as a JSON attachment.
async fn student_diploma(Query(q): Query<DiplomaQuery>) -> Response {
    match aion_edu_provenance::diploma(Path::new(DATA_DIR), &q.student, &q.lesson, "aion-edu") {
        Ok(Some(d)) => {
            let body = serde_json::to_vec_pretty(&d).unwrap_or_default();
            let fname = format!("diploma-{}-{}.json", q.student, q.lesson);
            (
                [
                    (header::CONTENT_TYPE, "application/json".to_string()),
                    (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{fname}\"")),
                ],
                body,
            )
                .into_response()
        }
        _ => (StatusCode::NOT_FOUND, "no diploma for this student/lesson").into_response(),
    }
}

/// Verify a diploma from the submitted document alone (no disk lookup) — exactly
/// what an external party with only the JSON would do.
async fn verify_diploma_doc(Json(d): Json<aion_edu_provenance::Diploma>) -> Json<Value> {
    match aion_edu_provenance::verify_diploma(&d) {
        Ok(v) => Json(json!({
            "ok": true, "authentic": v.authentic, "credential_verifies": v.credential_verifies,
            "file_id_match": v.file_id_match, "claims_match": v.claims_match, "detail": v.detail,
        })),
        Err(e) => Json(json!({"ok": false, "authentic": false, "detail": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct StartBody {
    learner: String,
    target: String,
    #[serde(default)]
    mode: Option<String>,
}

async fn session_start(Json(b): Json<StartBody>) -> Json<Value> {
    gc();
    let (etx, erx) = unbounded_channel::<Value>();
    let (rtx, rrx) = unbounded_channel::<String>();
    let finished = Arc::new(AtomicBool::new(false));
    let sid = next_sid();
    sessions().lock().expect("sessions lock").insert(
        sid.clone(),
        Arc::new(SessionEntry {
            events_rx: Mutex::new(Some(erx)),
            reply_tx: rtx,
            created: Instant::now(),
            finished: finished.clone(),
        }),
    );
    let mode = b.mode.unwrap_or_else(|| "interactive".to_string());
    std::thread::spawn(move || worker(b.target, b.learner, mode, etx, rrx, finished));
    Json(json!({"sid": sid}))
}

fn entry(sid: &str) -> Option<Arc<SessionEntry>> {
    sessions().lock().ok()?.get(sid).cloned()
}

async fn session_stream(AxPath(sid): AxPath<String>) -> Response {
    let Some(e) = entry(&sid) else {
        return (StatusCode::NOT_FOUND, "no such session").into_response();
    };
    let Some(rx) = e.events_rx.lock().expect("rx lock").take() else {
        return (StatusCode::CONFLICT, "stream already taken").into_response();
    };
    let stream = UnboundedReceiverStream::new(rx)
        .map(|v| Ok::<_, Infallible>(SseEvent::default().data(v.to_string())));
    Sse::new(stream).keep_alive(KeepAlive::default()).into_response()
}

#[derive(Deserialize)]
struct ReplyBody {
    text: String,
}

async fn session_reply(AxPath(sid): AxPath<String>, Json(b): Json<ReplyBody>) -> StatusCode {
    match entry(&sid) {
        Some(e) => {
            let _ = e.reply_tx.send(b.text);
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
}

// ── federation UI ────────────────────────────────────────────────────────────
async fn federate_page() -> Html<&'static str> {
    Html(include_str!("../web/federate.html"))
}

fn ok_msg(r: aion_edu_provenance::Result<()>, msg: String) -> Json<Value> {
    match r {
        Ok(()) => Json(json!({"ok": true, "msg": msg})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

async fn fed_state() -> Json<Value> {
    let dir = Path::new(DATA_DIR);
    let epoch = aion_edu_provenance::current_epoch(dir).unwrap_or(0);
    let recognitions: Vec<Value> = aion_edu_provenance::list_recognitions(dir)
        .into_iter()
        .map(|(by, of)| {
            let valid = aion_edu_provenance::verify_recognition(dir, &by, &of).ok().flatten().map(|r| r.valid).unwrap_or(false);
            json!({"by": by, "of": of, "valid": valid})
        })
        .collect();
    let programs: Vec<Value> = aion_edu_provenance::list_programs(dir)
        .into_iter()
        .map(|p| match aion_edu_provenance::verify_joint_accreditation(dir, &p).ok().flatten() {
            Some(j) => json!({"program": p, "met": j.met, "valid": j.valid_count, "threshold": j.threshold, "institutions": j.signing_institutions}),
            None => json!({"program": p, "met": false}),
        })
        .collect();
    let snapshots: Vec<Value> = aion_edu_provenance::list_snapshots(dir)
        .into_iter()
        .filter_map(|id| {
            aion_edu_provenance::verify_snapshot(dir, id)
                .ok()
                .flatten()
                .map(|v| json!({"id": v.id, "epoch": v.epoch, "matches": v.matches_current, "signers": v.valid_signers}))
        })
        .collect();
    Json(json!({"epoch": epoch, "recognitions": recognitions, "programs": programs, "snapshots": snapshots}))
}

#[derive(Deserialize)]
struct RecBody {
    by: String,
    of: String,
    #[serde(default)]
    until: u64,
}

async fn fed_recognize(Json(b): Json<RecBody>) -> Json<Value> {
    ok_msg(aion_edu_provenance::recognize_scoped(Path::new(DATA_DIR), &b.by, &b.of, b.until), format!("{} recognized {}", b.by, b.of))
}

async fn fed_revoke_recognition(Json(b): Json<RecBody>) -> Json<Value> {
    match aion_edu_provenance::revoke_recognition(Path::new(DATA_DIR), &b.by, &b.of) {
        Ok(at) => Json(json!({"ok": true, "msg": format!("{} revoked recognition of {} (epoch {at})", b.by, b.of)})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

async fn fed_advance() -> Json<Value> {
    match aion_edu_provenance::advance_epoch(Path::new(DATA_DIR)) {
        Ok(e) => Json(json!({"ok": true, "epoch": e, "msg": format!("epoch advanced to {e}")})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct AccreditBody {
    program: String,
    threshold: u32,
    signers: Vec<String>,
}

async fn fed_accredit(Json(b): Json<AccreditBody>) -> Json<Value> {
    let n = b.signers.len();
    ok_msg(
        aion_edu_provenance::set_joint_accreditation(Path::new(DATA_DIR), &b.program, b.threshold, &b.signers),
        format!("{} declared {}-of-{n}", b.program, b.threshold),
    )
}

#[derive(Deserialize)]
struct EndorseBody {
    program: String,
    by: String,
}

async fn fed_endorse(Json(b): Json<EndorseBody>) -> Json<Value> {
    ok_msg(aion_edu_provenance::endorse_program(Path::new(DATA_DIR), &b.program, &b.by), format!("{} endorsed {}", b.by, b.program))
}

#[derive(Deserialize)]
struct DelegateBody {
    professor: String,
    by: String,
    #[serde(default)]
    until: u64,
}

async fn fed_delegate(Json(b): Json<DelegateBody>) -> Json<Value> {
    ok_msg(aion_edu_provenance::delegate_scoped(Path::new(DATA_DIR), &b.by, &b.professor, b.until), format!("{} delegated {}", b.by, b.professor))
}

async fn fed_revoke_delegation(Json(b): Json<DelegateBody>) -> Json<Value> {
    match aion_edu_provenance::revoke_delegation(Path::new(DATA_DIR), &b.by, &b.professor) {
        Ok(at) => Json(json!({"ok": true, "msg": format!("{} revoked {}'s delegation (epoch {at})", b.by, b.professor)})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct IssueBody {
    learner: String,
    lesson: String,
    by: String,
}

async fn fed_issue(Json(b): Json<IssueBody>) -> Json<Value> {
    ok_msg(
        aion_edu_provenance::bind_issuer(Path::new(DATA_DIR), &b.by, &b.learner, &b.lesson),
        format!("{} vouched for {}'s {}", b.by, b.learner, b.lesson),
    )
}

#[derive(Deserialize)]
struct PresentBody {
    learner: String,
    lesson: String,
    issuer: String,
    to: String,
    as_of: Option<u64>,
}

async fn fed_present(Json(b): Json<PresentBody>) -> Json<Value> {
    let dir = Path::new(DATA_DIR);
    let res = match b.as_of {
        Some(e) => aion_edu_provenance::verify_issued_credential_at(dir, &b.to, &b.issuer, &b.learner, &b.lesson, e),
        None => aion_edu_provenance::verify_issued_credential(dir, &b.to, &b.issuer, &b.learner, &b.lesson),
    };
    match res {
        Ok(Some(c)) => Json(json!({
            "ok": true, "credential_valid": c.credential_valid, "faculty_delegated": c.faculty_delegated,
            "issuer_vouched": c.issuer_vouched, "issuer_recognized": c.issuer_recognized, "accepted": c.accepted,
        })),
        Ok(None) => Json(json!({"ok": false, "msg": "no such credential (teach it first)"})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct SnapBody {
    by: String,
}

async fn fed_snapshot(Json(b): Json<SnapBody>) -> Json<Value> {
    match aion_edu_provenance::take_snapshot(Path::new(DATA_DIR), &b.by) {
        Ok(id) => Json(json!({"ok": true, "id": id, "msg": format!("snapshot #{id} signed by {}", b.by)})),
        Err(e) => Json(json!({"ok": false, "msg": e.to_string()})),
    }
}

fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/learn", get(index))
        .route("/app.css", get(app_css))
        .route("/favicon.svg", get(favicon))
        .route("/favicon.ico", get(favicon))
        .route("/narration/{name}", get(narration))
        .route("/api/catalog", get(catalog))
        .route("/api/plan", get(plan))
        .route("/api/enroll", post(student_enroll))
        .route("/api/transcript", get(student_transcript))
        .route("/api/diploma", get(student_diploma))
        .route("/api/verify-diploma", post(verify_diploma_doc))
        .route("/api/session", post(session_start))
        .route("/api/session/{sid}/stream", get(session_stream))
        .route("/api/session/{sid}/reply", post(session_reply))
        .route("/federate", get(federate_page))
        .route("/api/fed/state", get(fed_state))
        .route("/api/fed/recognize", post(fed_recognize))
        .route("/api/fed/revoke_recognition", post(fed_revoke_recognition))
        .route("/api/fed/advance", post(fed_advance))
        .route("/api/fed/accredit", post(fed_accredit))
        .route("/api/fed/endorse", post(fed_endorse))
        .route("/api/fed/delegate", post(fed_delegate))
        .route("/api/fed/revoke_delegation", post(fed_revoke_delegation))
        .route("/api/fed/issue", post(fed_issue))
        .route("/api/fed/present", post(fed_present))
        .route("/api/fed/snapshot", post(fed_snapshot))
}

/// Run the web server (blocking; builds its own Tokio runtime).
pub fn serve(host: &str, port: u16) -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let listener = tokio::net::TcpListener::bind((host, port)).await?;
        axum::serve(listener, router()).await
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy(finished: bool) -> Arc<SessionEntry> {
        let (_etx, erx) = unbounded_channel::<Value>();
        let (rtx, _rrx) = unbounded_channel::<String>();
        Arc::new(SessionEntry {
            events_rx: Mutex::new(Some(erx)),
            reply_tx: rtx,
            created: Instant::now(),
            finished: Arc::new(AtomicBool::new(finished)),
        })
    }

    #[test]
    fn gc_reaps_finished_keeps_live() {
        sessions().lock().unwrap().insert("gc-done".into(), dummy(true));
        sessions().lock().unwrap().insert("gc-live".into(), dummy(false));
        gc();
        let map = sessions().lock().unwrap();
        assert!(!map.contains_key("gc-done"), "finished session must be reaped");
        assert!(map.contains_key("gc-live"), "live session must survive");
        drop(map);
        sessions().lock().unwrap().remove("gc-live"); // tidy global
    }
}
