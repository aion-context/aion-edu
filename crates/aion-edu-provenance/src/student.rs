//! Student enrollment, cryptographic identity, and the verifiable transcript.
//!
//! A student is a first-class identity in the same trust fabric as professors and
//! institutions: enrolling derives an Ed25519 key (via [`author_for`] +
//! [`load_or_make_key`]) and writes an enrollment record. The transcript is the
//! student's collection of professor-signed credentials, each re-verified and
//! checked against the live rubric lineage — a portable, offline-verifiable wallet.

use std::fs;
use std::path::{Path, PathBuf};

use aion_context::crypto::VerifyingKey;
use aion_context::key_registry::KeyRegistry;
use aion_context::operations::{show_current_rules, verify_file};
use aion_context::types::AuthorId;
use serde::{Deserialize, Serialize};

use crate::{author_for, load_or_make_key, verify_binding, Error, Result};

/// A student's enrollment: identity + the target they are working toward.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub student: String,
    pub author_id: u64,
    pub public_key: Vec<u8>,
    pub target: String,
    pub enrolled_epoch: u64,
}

fn enroll_path(dir: &Path, student: &str) -> PathBuf {
    dir.join("students").join(student).join("enrollment.json")
}

/// Enroll a student: derive (or load) their key and record the enrollment. Stamped
/// at the current federation epoch. Re-enrolling updates the target.
pub fn enroll(dir: &Path, student: &str, target: &str) -> Result<Enrollment> {
    let author = author_for(student);
    let sk = load_or_make_key(dir, author)?;
    let e = Enrollment {
        student: student.to_string(),
        author_id: author,
        public_key: sk.verifying_key().to_bytes().to_vec(),
        target: target.to_string(),
        enrolled_epoch: crate::current_epoch(dir)?,
    };
    let path = enroll_path(dir, student);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serde_json::to_vec_pretty(&e)?)?;
    Ok(e)
}

/// A student's enrollment record, if they are enrolled.
pub fn enrollment(dir: &Path, student: &str) -> Option<Enrollment> {
    fs::read(enroll_path(dir, student)).ok().and_then(|b| serde_json::from_slice(&b).ok())
}

/// One verified entry in a student's transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub lesson_id: String,
    pub professor: String,
    pub credential_file_id: u64,
    /// the professor signature holds and the credential is untampered.
    pub credential_valid: bool,
    /// bound to the current rubric lineage at the version it was graded against.
    pub bound_to_current_rubric: bool,
}

/// The student's verifiable transcript: every credential they hold, re-verified.
pub fn transcript(dir: &Path, student: &str) -> Result<Vec<TranscriptEntry>> {
    let cdir = dir.join("credentials").join(student);
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(&cdir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.extension().and_then(|x| x.to_str()) != Some("aion") {
                continue;
            }
            let lesson = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            let Some(info) = crate::read_credential(dir, student, &lesson)? else {
                continue;
            };
            let professor = info.professor.clone();
            let bound = verify_binding(dir, &professor, student, &lesson)?
                .map(|b| b.lineage_match && b.graded_against_current)
                .unwrap_or(false);
            out.push(TranscriptEntry {
                lesson_id: lesson,
                professor,
                credential_file_id: info.file_id,
                credential_valid: info.valid,
                bound_to_current_rubric: bound,
            });
        }
    }
    out.sort_by(|a, b| a.lesson_id.cmp(&b.lesson_id));
    Ok(out)
}

// ── downloadable diploma: a self-contained, offline-verifiable artifact ──────

fn hexstr(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A portable diploma: the verified facts plus the raw signed credential and the
/// professor's public key, so a third party can re-verify it entirely offline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diploma {
    pub issuer: String,
    pub student: String,
    pub student_key_fp: String,
    pub lesson_id: String,
    pub professor: String,
    pub mastered: bool,
    pub credential_file_id: u64,
    pub credential_valid: bool,
    pub rubric_file_id: u64,
    pub rubric_version: u64,
    pub bound_to_current_rubric: bool,
    pub professor_key_hex: String,
    pub credential_aion_hex: String,
    pub issued_at_epoch: u64,
    pub verify_note: String,
}

/// Assemble a downloadable diploma for `student`'s `lesson_id` credential, issued
/// under `issuer`. `Ok(None)` if the student holds no such credential.
pub fn diploma(dir: &Path, student: &str, lesson_id: &str, issuer: &str) -> Result<Option<Diploma>> {
    let Some(info) = crate::read_credential(dir, student, lesson_id)? else {
        return Ok(None);
    };
    let professor = info.professor.clone();
    let binding = verify_binding(dir, &professor, student, lesson_id)?;
    let raw = crate::credential_raw(dir, student, lesson_id)?.unwrap_or_default();
    let prof_key = load_or_make_key(dir, author_for(&professor))?.verifying_key().to_bytes();
    let student_key = load_or_make_key(dir, author_for(student))?.verifying_key().to_bytes();
    let issued_at_epoch = enrollment(dir, student).map(|e| e.enrolled_epoch).unwrap_or(crate::current_epoch(dir)?);

    let (rubric_file_id, rubric_version, bound) = binding
        .map(|b| (b.bound.file_id, b.bound.version, b.lineage_match && b.graded_against_current))
        .unwrap_or((0, 0, false));

    Ok(Some(Diploma {
        issuer: issuer.to_string(),
        student: student.to_string(),
        student_key_fp: hexstr(&student_key[..6]),
        lesson_id: lesson_id.to_string(),
        professor,
        mastered: true,
        credential_file_id: info.file_id,
        credential_valid: info.valid,
        rubric_file_id,
        rubric_version,
        bound_to_current_rubric: bound,
        professor_key_hex: hexstr(&prof_key),
        credential_aion_hex: hexstr(&raw),
        issued_at_epoch,
        verify_note: "Re-verify offline: the credential_aion_hex is the signed .aion file; \
            check it against professor_key_hex. Acceptance also requires the issuer to have \
            backed it (delegation or vouch) and your institution to recognize the issuer."
            .to_string(),
    }))
}

// ── offline diploma verification (no data dir, no key store) ─────────────────

fn hex_decode(s: &str) -> Result<Vec<u8>> {
    if s.len() % 2 != 0 {
        return Err(Error::Aion("odd-length hex".into()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| Error::Aion("invalid hex".into())))
        .collect()
}

/// The verdict of verifying a diploma purely from its own contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaVerdict {
    /// the bundled signed credential verifies against the bundled professor key.
    pub credential_verifies: bool,
    /// the credential's file_id matches the diploma's claim.
    pub file_id_match: bool,
    /// the diploma's human-readable facts match the signed credential's content.
    pub claims_match: bool,
    /// all of the above — the diploma is genuine and self-consistent.
    pub authentic: bool,
    pub detail: String,
}

/// Verify a diploma **offline** — using only the diploma itself, no data
/// directory and no key store. Reconstructs the signed `.aion` credential from
/// the bundled hex, verifies it against the bundled professor key, and checks the
/// diploma's claims against the credential's actual signed content.
pub fn verify_diploma(d: &Diploma) -> Result<DiplomaVerdict> {
    let raw = hex_decode(&d.credential_aion_hex)?;
    let vk1 = VerifyingKey::from_bytes(&hex_decode(&d.professor_key_hex)?)?;
    let vk2 = VerifyingKey::from_bytes(&hex_decode(&d.professor_key_hex)?)?;
    let author = author_for(&d.professor);
    let mut reg = KeyRegistry::new();
    reg.register_author(AuthorId(author), vk1, vk2, 1).map_err(|e| Error::Aion(e.to_string()))?;

    // write the bundled signed credential to a temp file and verify it offline
    let tmp = std::env::temp_dir().join(format!("aion-diploma-{}-{}.aion", std::process::id(), d.credential_file_id));
    fs::write(&tmp, &raw)?;
    let report = verify_file(&tmp, &reg);
    let content = show_current_rules(&tmp);
    let _ = fs::remove_file(&tmp);
    let report = report?;
    let content = content?;

    let credential_verifies = report.is_valid;
    let file_id_match = report.file_id.0 == d.credential_file_id;

    let v: serde_json::Value = serde_json::from_slice(&content)?;
    let claims_match = v["learner"].as_str() == Some(d.student.as_str())
        && v["professor"].as_str() == Some(d.professor.as_str())
        && v["lesson_id"].as_str() == Some(d.lesson_id.as_str())
        && v["mastered"].as_bool() == Some(d.mastered)
        && v["rubric"]["file_id"].as_u64() == Some(d.rubric_file_id)
        && v["rubric"]["version"].as_u64() == Some(d.rubric_version);

    let authentic = credential_verifies && file_id_match && claims_match;
    let detail = if authentic {
        format!("{}'s '{}' credential — signed by Prof. {} under {} — verifies.", d.student, d.lesson_id, d.professor, d.issuer)
    } else if !credential_verifies {
        "the signed credential does not verify against the professor key (tampered or wrong key)".to_string()
    } else if !file_id_match {
        "the credential file_id does not match the diploma's claim".to_string()
    } else {
        "the diploma's facts do not match the signed credential's content".to_string()
    };
    Ok(DiplomaVerdict { credential_verifies, file_id_match, claims_match, authentic, detail })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enroll_then_transcript_reflects_credentials() {
        let dir = std::env::temp_dir().join(format!("aion-edu-student-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);

        let e = enroll(&dir, "alice", "cs440-u1-l1").unwrap();
        assert_eq!(e.student, "alice");
        assert!(!e.public_key.is_empty(), "student gets a cryptographic identity");
        assert!(enrollment(&dir, "alice").is_some());
        assert!(transcript(&dir, "alice").unwrap().is_empty(), "no credentials yet");

        // a professor issues a credential to alice
        let rref = crate::RubricRef { file_id: 1, version: 1 };
        crate::commit_credential(&dir, "alice", "lynch", "cs440-u1-l1", "ok", rref).unwrap();
        let t = transcript(&dir, "alice").unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].lesson_id, "cs440-u1-l1");
        assert_eq!(t[0].professor, "lynch");
        assert!(t[0].credential_valid, "the credential verifies under the professor key");

        // a downloadable diploma bundles the verified facts + raw artifact + prof key
        let d = diploma(&dir, "alice", "cs440-u1-l1", "aion-edu").unwrap().unwrap();
        assert_eq!(d.student, "alice");
        assert_eq!(d.professor, "lynch");
        assert_eq!(d.issuer, "aion-edu");
        assert!(d.credential_valid && d.mastered);
        assert!(!d.credential_aion_hex.is_empty(), "carries the raw signed credential");
        assert_eq!(d.professor_key_hex.len(), 64, "32-byte professor key as hex");
        assert!(diploma(&dir, "alice", "no-such-lesson", "aion-edu").unwrap().is_none());

        // verify the diploma OFFLINE — using only the diploma itself
        let ok = verify_diploma(&d).unwrap();
        assert!(ok.authentic && ok.credential_verifies && ok.file_id_match && ok.claims_match);

        // a forged claim (different lesson) is caught — credential still verifies, claims don't match
        let mut forged = d.clone();
        forged.lesson_id = "cs999-u1-l1".to_string();
        let bad = verify_diploma(&forged).unwrap();
        assert!(bad.credential_verifies && !bad.claims_match && !bad.authentic);

        // a tampered signed credential (flip a byte) fails cryptographic verification
        let mut tampered = d.clone();
        let mut h: Vec<char> = tampered.credential_aion_hex.chars().collect();
        let mid = h.len() / 2;
        h[mid] = if h[mid] == 'a' { 'b' } else { 'a' };
        tampered.credential_aion_hex = h.into_iter().collect();
        // verify_diploma either returns not-authentic or errors on a corrupt envelope — both reject it
        let rejected = verify_diploma(&tampered).map(|v| !v.authentic).unwrap_or(true);
        assert!(rejected, "a tampered credential must not verify");

        let _ = fs::remove_dir_all(&dir);
    }
}
