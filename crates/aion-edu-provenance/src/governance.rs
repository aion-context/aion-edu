//! K-of-N rubric governance via aion-context native multisig (RFC-0021).
//!
//! A lesson can require a quorum of authorized faculty before it may be taught.
//! Each professor independently signs an attestation over the **canonical rubric
//! content** (a `VersionEntry` whose `rules_hash` is BLAKE3 of that content).
//! `verify_multisig` counts distinct authorized signers whose signature verifies
//! against that exact content — so a professor who endorses a *weakened* rubric
//! signs a different hash and is excluded (Byzantine signer), and no single
//! professor can lower a course's standard alone.

use std::fs;
use std::path::{Path, PathBuf};

use aion_context::crypto::hash;
use aion_context::key_registry::KeyRegistry;
use aion_context::multisig::{verify_multisig, MultiSigPolicy};
use aion_context::serializer::{SignatureEntry, VersionEntry};
use aion_context::signature_chain::sign_attestation;
use aion_context::types::{AuthorId, VersionNumber};
use aion_edu_core::{Course, Lesson};
use serde::{Deserialize, Serialize};

use crate::{author_for, canonical, load_or_make_key, Error, Result};

const GOV_TS: u64 = 1_700_000_000_000_000_000;

/// A lesson's quorum policy: how many of which faculty must endorse.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub threshold: u32,
    pub signers: Vec<String>,
}

/// The outcome of a quorum check.
#[derive(Debug, Clone)]
pub struct Quorum {
    pub threshold: u32,
    pub valid_count: u32,
    pub met: bool,
    pub valid_signers: Vec<u64>,
    pub invalid_signers: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct StoredSig {
    author_id: u64,
    public_key: Vec<u8>,
    signature: Vec<u8>,
}

fn policy_path(dir: &Path, lesson_id: &str) -> PathBuf {
    dir.join("governance").join(format!("{lesson_id}.policy.json"))
}

/// Declare a lesson's quorum policy.
pub fn set_governance(dir: &Path, lesson_id: &str, threshold: u32, signers: &[String]) -> Result<()> {
    fs::create_dir_all(dir.join("governance"))?;
    let mut s = signers.to_vec();
    s.sort();
    s.dedup();
    let policy = Policy { threshold, signers: s };
    fs::write(policy_path(dir, lesson_id), serde_json::to_vec_pretty(&policy)?)?;
    Ok(())
}

/// Read a lesson's quorum policy, if governed.
pub fn governance(dir: &Path, lesson_id: &str) -> Option<Policy> {
    fs::read(policy_path(dir, lesson_id))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
}

/// The canonical attestation target: a `VersionEntry` over the rubric content.
fn version_for(course: &Course, lesson: &Lesson) -> Result<VersionEntry> {
    let rules_hash = hash(&canonical(course, lesson)?);
    Ok(VersionEntry::new(VersionNumber(1), [0u8; 32], rules_hash, AuthorId(0), GOV_TS, 0, 0))
}

/// An authorized professor signs their endorsement of the rubric content.
pub fn endorse(dir: &Path, course: &Course, lesson: &Lesson, professor_id: &str) -> Result<()> {
    let author = author_for(professor_id);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&version_for(course, lesson)?, AuthorId(author), &sk);
    let stored = StoredSig {
        author_id: entry.author_id,
        public_key: entry.public_key.to_vec(),
        signature: entry.signature.to_vec(),
    };
    let into = dir.join("governance").join(&lesson.id);
    fs::create_dir_all(&into)?;
    fs::write(into.join(format!("{author}.sig.json")), serde_json::to_vec(&stored)?)?;
    Ok(())
}

fn load_endorsements(dir: &Path, lesson_id: &str) -> Result<Vec<SignatureEntry>> {
    let into = dir.join("governance").join(lesson_id);
    let mut out = Vec::new();
    if !into.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(&into)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let s: StoredSig = serde_json::from_slice(&fs::read(&path)?)?;
        let pk: [u8; 32] = s.public_key.try_into().map_err(|_| Error::Aion("bad public key".into()))?;
        let sig: [u8; 64] = s.signature.try_into().map_err(|_| Error::Aion("bad signature".into()))?;
        out.push(SignatureEntry::new(AuthorId(s.author_id), pk, sig));
    }
    Ok(out)
}

/// Count distinct authorized faculty whose endorsement verifies against the
/// canonical rubric. `Ok(None)` if the lesson is ungoverned.
pub fn verify_quorum(dir: &Path, course: &Course, lesson: &Lesson) -> Result<Option<Quorum>> {
    let Some(policy) = governance(dir, &lesson.id) else {
        return Ok(None);
    };
    let version = version_for(course, lesson)?;
    let mut registry = KeyRegistry::new();
    let mut authorized = Vec::new();
    for prof in &policy.signers {
        let a = author_for(prof);
        authorized.push(AuthorId(a));
        let sk = load_or_make_key(dir, a)?;
        registry
            .register_author(AuthorId(a), sk.verifying_key(), sk.verifying_key(), 1)
            .map_err(|e| Error::Aion(e.to_string()))?;
    }
    let sigs = load_endorsements(dir, &lesson.id)?;
    let pol = MultiSigPolicy::m_of_n(policy.threshold, authorized).map_err(|e| Error::Aion(e.to_string()))?;
    let v = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?;
    Ok(Some(Quorum {
        threshold: v.required,
        valid_count: v.valid_count,
        met: v.threshold_met,
        valid_signers: v.valid_signers.iter().map(|a| a.0).collect(),
        invalid_signers: v.invalid_signers.iter().map(|a| a.0).collect(),
    }))
}
