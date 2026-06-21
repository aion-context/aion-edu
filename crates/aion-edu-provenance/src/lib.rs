//! aion-edu-provenance — the trust spine, in-process over `aion-context`.
//!
//! Seal a lesson's rubric into a tamper-evident, Ed25519-signed `.aion` policy,
//! and verify it offline. The professor grades against a rubric it cannot alter;
//! a tampered rubric fails verification. This is the showcase: no subprocess —
//! `aion-context` signs and verifies directly in the kernel.

#![forbid(unsafe_code)]

mod error;
mod federation;
mod governance;
mod student;

pub use federation::{
    advance_epoch, bind_issuer, current_epoch, delegate, delegate_scoped, endorse_program, institution, joint_policy,
    list_programs, list_recognitions, list_snapshots, mutually_recognized, recognize, recognize_scoped,
    revoke_delegation, revoke_recognition, set_joint_accreditation, take_snapshot, verify_delegation,
    verify_delegation_at, verify_issued_credential, verify_issued_credential_at, verify_joint_accreditation,
    verify_recognition, verify_recognition_at, verify_snapshot, Institution, IssuedCredential, JointAccreditation,
    ProgramPolicy, Recognition, SnapshotVerification,
};
pub use governance::{endorse, governance, set_governance, verify_quorum, Quorum};
pub use student::{
    diploma, enroll, enrollment, transcript, verify_diploma, Diploma, DiplomaVerdict, Enrollment, TranscriptEntry,
};

use std::fs;
use std::path::Path;

use aion_context::crypto::SigningKey;
use aion_context::key_registry::KeyRegistry;
use aion_context::operations::{commit_version, init_file, show_current_rules, verify_file, CommitOptions, InitOptions};
use aion_context::types::AuthorId;
use aion_edu_core::{Course, Lesson, RubricCriterion};
use serde::{Deserialize, Serialize};

pub use error::{Error, Result};

/// Result of sealing/verifying a rubric: its signed identity and validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sealed {
    pub file_id: u64,
    pub version: u64,
    pub valid: bool,
}

/// Deterministic author id for a professor (stable across runs — FNV-1a).
pub fn author_for(professor_id: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in professor_id.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    340_000 + (h % 50_000)
}

/// The exact bytes that get signed — the canonical rubric content.
#[derive(Serialize)]
struct RubricPayload<'a> {
    lesson_id: &'a str,
    professor: &'a str,
    mastery_outcomes: &'a [String],
    rubric: &'a [RubricCriterion],
}

pub(crate) fn canonical(course: &Course, lesson: &Lesson) -> Result<Vec<u8>> {
    let payload = RubricPayload {
        lesson_id: &lesson.id,
        professor: &course.professor,
        mastery_outcomes: &lesson.mastery_outcomes,
        rubric: &lesson.rubric,
    };
    Ok(serde_json::to_vec(&payload)?)
}

pub(crate) fn load_or_make_key(dir: &Path, author: u64) -> Result<SigningKey> {
    let keys = dir.join("keys");
    fs::create_dir_all(&keys)?;
    let path = keys.join(format!("{author}.key"));
    if path.exists() {
        let bytes = fs::read(&path)?;
        return Ok(SigningKey::from_bytes(&bytes)?);
    }
    let sk = SigningKey::generate();
    fs::write(&path, sk.to_bytes())?;
    Ok(sk)
}

fn registry_for(author: u64, sk: &SigningKey) -> Result<KeyRegistry> {
    let mut reg = KeyRegistry::new();
    reg.register_author(AuthorId(author), sk.verifying_key(), sk.verifying_key(), 1)?;
    Ok(reg)
}

fn rubric_path(dir: &Path, lesson_id: &str) -> std::path::PathBuf {
    dir.join("rubrics").join(format!("{lesson_id}.aion"))
}

/// Seal a lesson's rubric into a signed `.aion`. First seal is the genesis
/// (v1); re-sealing **changed** content appends a new signed version (v2, v3…),
/// preserving the hash-chained history; re-sealing **unchanged** content is a
/// no-op (no spurious version bump).
pub fn seal_rubric(dir: &Path, course: &Course, lesson: &Lesson) -> Result<Sealed> {
    let author = author_for(&course.professor);
    let sk = load_or_make_key(dir, author)?;
    fs::create_dir_all(dir.join("rubrics"))?;
    let path = rubric_path(dir, &lesson.id);
    let payload = canonical(course, lesson)?;
    let reg = registry_for(author, &sk)?;

    let version = if !path.exists() {
        let opts = InitOptions {
            author_id: AuthorId(author),
            signing_key: &sk,
            message: &format!("{} rubric", lesson.id),
            timestamp: None,
        };
        init_file(&path, &payload, &opts)?.version.0
    } else if show_current_rules(&path)? == payload {
        verify_file(&path, &reg)?.version_count // unchanged — keep current version
    } else {
        let opts = CommitOptions {
            author_id: AuthorId(author),
            signing_key: &sk,
            message: &format!("{} rubric revision", lesson.id),
            timestamp: None,
        };
        commit_version(&path, &payload, &opts, &reg)?.version.0
    };
    let report = verify_file(&path, &reg)?;
    Ok(Sealed { file_id: report.file_id.0, version, valid: report.is_valid })
}

/// Verify a previously sealed rubric (the offline diploma check). `Ok(None)` if
/// no sealed rubric exists for this lesson.
pub fn verify_rubric(dir: &Path, professor_id: &str, lesson_id: &str) -> Result<Option<Sealed>> {
    let path = rubric_path(dir, lesson_id);
    if !path.exists() {
        return Ok(None);
    }
    let author = author_for(professor_id);
    let sk = load_or_make_key(dir, author)?;
    let reg = registry_for(author, &sk)?;
    let report = verify_file(&path, &reg)?;
    Ok(Some(Sealed {
        file_id: report.file_id.0,
        version: report.version_count,
        valid: report.is_valid,
    }))
}

// ── credentials: a per-lesson signed attestation bound to the rubric ─────────

/// The rubric identity a credential was graded against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RubricRef {
    pub file_id: u64,
    pub version: u64,
}

/// The signed credential payload (also what we read back for binding).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CredentialDoc {
    learner: String,
    professor: String,
    lesson_id: String,
    rubric: RubricRef,
    summary: String,
    mastered: bool,
}

fn credential_path(dir: &Path, learner: &str, lesson_id: &str) -> std::path::PathBuf {
    dir.join("credentials").join(learner).join(format!("{lesson_id}.aion"))
}

/// Issue a professor-signed credential for one lesson, binding the rubric
/// `{file_id, version}` it was graded against into the signed payload.
pub fn commit_credential(
    dir: &Path,
    learner: &str,
    professor_id: &str,
    lesson_id: &str,
    summary: &str,
    rubric: RubricRef,
) -> Result<u64> {
    let author = author_for(professor_id);
    let sk = load_or_make_key(dir, author)?;
    let path = credential_path(dir, learner, lesson_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if path.exists() {
        fs::remove_file(&path)?;
    }
    let doc = CredentialDoc {
        learner: learner.to_string(),
        professor: professor_id.to_string(),
        lesson_id: lesson_id.to_string(),
        rubric,
        summary: summary.to_string(),
        mastered: true,
    };
    let bytes = serde_json::to_vec(&doc)?;
    let message = format!("mastery: {lesson_id}");
    let opts = InitOptions {
        author_id: AuthorId(author),
        signing_key: &sk,
        message: &message,
        timestamp: None,
    };
    let res = init_file(&path, &bytes, &opts)?;
    Ok(res.file_id.0)
}

/// Proof a credential names the real, still-valid rubric it was graded against.
#[derive(Debug, Clone, Copy)]
pub struct Binding {
    pub credential_valid: bool,
    pub lineage_match: bool,
    pub graded_against_current: bool,
    pub rubric_revised_since: bool,
    pub bound: RubricRef,
    pub live: RubricRef,
}

/// Verify a credential is untampered and that its bound rubric matches the live
/// rubric lineage. `Ok(None)` if no credential exists.
pub fn verify_binding(
    dir: &Path,
    professor_id: &str,
    learner: &str,
    lesson_id: &str,
) -> Result<Option<Binding>> {
    let path = credential_path(dir, learner, lesson_id);
    if !path.exists() {
        return Ok(None);
    }
    let author = author_for(professor_id);
    let sk = load_or_make_key(dir, author)?;
    let reg = registry_for(author, &sk)?;
    let report = verify_file(&path, &reg)?;

    let doc: CredentialDoc = serde_json::from_slice(&show_current_rules(&path)?)?;
    let bound = doc.rubric;
    let live = verify_rubric(dir, professor_id, lesson_id)?
        .map(|s| RubricRef { file_id: s.file_id, version: s.version })
        .unwrap_or(RubricRef { file_id: 0, version: 0 });
    let lineage_match = bound.file_id != 0 && bound.file_id == live.file_id;
    Ok(Some(Binding {
        credential_valid: report.is_valid,
        lineage_match,
        graded_against_current: lineage_match && bound.version == live.version,
        rubric_revised_since: lineage_match && bound.version < live.version,
        bound,
        live,
    }))
}

/// Internal: a credential's signed identity, for the federation layer to vouch
/// over and counter-sign. `valid` reflects the professor signature + integrity.
pub(crate) struct CredentialInfo {
    pub file_id: u64,
    pub valid: bool,
    pub professor: String,
    pub content: Vec<u8>,
}

/// The raw signed `.aion` credential bytes (the verifiable artifact itself).
pub(crate) fn credential_raw(dir: &Path, learner: &str, lesson_id: &str) -> Result<Option<Vec<u8>>> {
    let path = credential_path(dir, learner, lesson_id);
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(fs::read(&path)?))
}

pub(crate) fn read_credential(dir: &Path, learner: &str, lesson_id: &str) -> Result<Option<CredentialInfo>> {
    let path = credential_path(dir, learner, lesson_id);
    if !path.exists() {
        return Ok(None);
    }
    let content = show_current_rules(&path)?;
    let doc: CredentialDoc = serde_json::from_slice(&content)?;
    let author = author_for(&doc.professor);
    let sk = load_or_make_key(dir, author)?;
    let reg = registry_for(author, &sk)?;
    let report = verify_file(&path, &reg)?;
    Ok(Some(CredentialInfo { file_id: report.file_id.0, valid: report.is_valid, professor: doc.professor, content }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Course, Lesson) {
        let lesson = Lesson {
            id: "t-l1".into(),
            title: "T".into(),
            mastery_outcomes: vec!["o1".into(), "o2".into()],
            tutor_brief: "b".into(),
            prerequisites: vec![],
            practice: aion_edu_core::Practice {
                prompt: "p".into(),
                starter_files: vec![],
                verify: "true".into(),
            },
            rubric: vec![RubricCriterion { outcome: "o1".into(), criterion: "c1".into() }],
        };
        let course = Course {
            id: "t".into(),
            title: "T".into(),
            professor: "lamport".into(),
            prerequisites: vec![],
            units: vec![aion_edu_core::Unit {
                id: "t-u1".into(),
                title: "U".into(),
                lessons: vec![lesson.clone()],
            }],
        };
        (course, lesson)
    }

    #[test]
    fn seal_then_verify_is_valid() {
        let dir = std::env::temp_dir().join(format!("aion-edu-prov-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (course, lesson) = fixture();
        let sealed = seal_rubric(&dir, &course, &lesson).unwrap();
        assert!(sealed.valid, "freshly sealed rubric must verify");
        assert_eq!(sealed.version, 1);

        let again = verify_rubric(&dir, "lamport", "t-l1").unwrap().unwrap();
        assert!(again.valid);
        assert_eq!(again.file_id, sealed.file_id);

        // tamper one byte mid-file -> verification must fail
        let path = rubric_path(&dir, "t-l1");
        let mut bytes = fs::read(&path).unwrap();
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0xFF;
        fs::write(&path, &bytes).unwrap();
        let tampered = verify_rubric(&dir, "lamport", "t-l1").unwrap().unwrap();
        assert!(!tampered.valid, "a tampered rubric must NOT verify");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn credential_binds_to_rubric() {
        let dir = std::env::temp_dir().join(format!("aion-edu-cred-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (course, lesson) = fixture();
        let sealed = seal_rubric(&dir, &course, &lesson).unwrap();
        let rref = RubricRef { file_id: sealed.file_id, version: sealed.version };

        commit_credential(&dir, "dj", "lamport", "t-l1", "ok", rref).unwrap();
        let b = verify_binding(&dir, "lamport", "dj", "t-l1").unwrap().unwrap();
        assert!(b.credential_valid, "credential must verify");
        assert!(b.lineage_match, "bound rubric must match live rubric file_id");
        assert!(b.graded_against_current, "bound version == live version");
        assert_eq!(b.bound, rref);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rubric_revision_versions_and_binding_tracks_it() {
        let dir = std::env::temp_dir().join(format!("aion-edu-rev-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (course, mut lesson) = fixture();

        let s1 = seal_rubric(&dir, &course, &lesson).unwrap();
        assert_eq!(s1.version, 1, "first seal is genesis v1");
        assert_eq!(seal_rubric(&dir, &course, &lesson).unwrap().version, 1, "unchanged re-seal stays v1");

        // a credential graded against v1
        let rref = RubricRef { file_id: s1.file_id, version: s1.version };
        commit_credential(&dir, "dj", "lamport", "t-l1", "ok", rref).unwrap();
        let b1 = verify_binding(&dir, "lamport", "dj", "t-l1").unwrap().unwrap();
        assert!(b1.graded_against_current && !b1.rubric_revised_since);

        // revise the rubric content -> v2 (same file lineage)
        lesson.mastery_outcomes.push("an added outcome".to_string());
        let s2 = seal_rubric(&dir, &course, &lesson).unwrap();
        assert_eq!(s2.version, 2, "changed re-seal commits v2");
        assert_eq!(s2.file_id, s1.file_id, "same file_id across versions");

        // the v1 credential now reads as graded against an earlier, revised rubric
        let b2 = verify_binding(&dir, "lamport", "dj", "t-l1").unwrap().unwrap();
        assert!(b2.lineage_match, "same file_id");
        assert!(!b2.graded_against_current, "bound to v1, live is v2");
        assert!(b2.rubric_revised_since, "rubric revised since the credential");
        assert_eq!((b2.bound.version, b2.live.version), (1, 2));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn quorum_gates_and_excludes_weakened_signer() {
        let dir = std::env::temp_dir().join(format!("aion-edu-quorum-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (course, lesson) = fixture();
        let signers = ["lamport".to_string(), "strang".to_string(), "pike".to_string()];
        set_governance(&dir, "t-l1", 2, &signers).unwrap();

        // 0 endorsements -> not met
        assert!(!verify_quorum(&dir, &course, &lesson).unwrap().unwrap().met);
        endorse(&dir, &course, &lesson, "lamport").unwrap();
        assert!(!verify_quorum(&dir, &course, &lesson).unwrap().unwrap().met); // 1 of 2
        endorse(&dir, &course, &lesson, "strang").unwrap();
        let q = verify_quorum(&dir, &course, &lesson).unwrap().unwrap();
        assert!(q.met && q.valid_count == 2, "2-of-3 quorum met");

        // rogue pike endorses a WEAKENED rubric (drop an outcome) -> excluded
        let mut weak = lesson.clone();
        weak.mastery_outcomes.truncate(1);
        weak.rubric.truncate(1);
        endorse(&dir, &course, &weak, "pike").unwrap();
        let q2 = verify_quorum(&dir, &course, &lesson).unwrap().unwrap();
        assert_eq!(q2.valid_count, 2, "rogue weakened signer must not count");
        assert!(q2.invalid_signers.contains(&author_for("pike")), "pike flagged byzantine");

        let _ = fs::remove_dir_all(&dir);
    }
}
