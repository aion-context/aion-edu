//! Accreditation federation — associating an external university's accreditation
//! with this one, entirely over `aion-context` (RFC-0034 registry-aware verify +
//! RFC-0021 K-of-N multisig). No new trust assumptions: federation widens the
//! *signer set* and the *key registry*, which is exactly what the substrate is for.
//!
//! Two layers:
//!  - **Layer 1 — Recognition.** Institution A signs a cross-certificate over
//!    institution B's name + root key. A peer/transfer vouch: anyone holding A's
//!    root key can verify, offline, that A recognizes B. Mutual recognition is two
//!    such certificates.
//!  - **Layer 2 — Co-accreditation.** A joint program is gated by a K-of-N quorum
//!    whose signers SPAN both universities. Every program endorsement is a real
//!    multisig over the program statement; `verify` reports which institutions
//!    signed, so a quorum cannot be met by one university alone.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use aion_context::crypto::hash;
use aion_context::key_registry::KeyRegistry;
use aion_context::multisig::{verify_multisig, MultiSigPolicy};
use aion_context::serializer::{SignatureEntry, VersionEntry};
use aion_context::signature_chain::sign_attestation;
use aion_context::types::{AuthorId, VersionNumber};
use serde::{Deserialize, Serialize};

use crate::{author_for, load_or_make_key, Error, Result};

const FED_TS: u64 = 1_700_000_000_000_000_000;

/// An institution's federation identity: a name and its root Ed25519 key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub name: String,
    pub author_id: u64,
    pub public_key: Vec<u8>,
}

/// Derive (or load) an institution's root accreditation identity.
pub fn institution(dir: &Path, name: &str) -> Result<Institution> {
    let author = author_for(name);
    let sk = load_or_make_key(dir, author)?;
    Ok(Institution { name: name.to_string(), author_id: author, public_key: sk.verifying_key().to_bytes().to_vec() })
}

#[derive(Serialize, Deserialize)]
struct StoredSig {
    author_id: u64,
    public_key: Vec<u8>,
    signature: Vec<u8>,
}

fn store_sig(path: &Path, entry: &SignatureEntry) -> Result<()> {
    let stored = StoredSig { author_id: entry.author_id, public_key: entry.public_key.to_vec(), signature: entry.signature.to_vec() };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec(&stored)?)?;
    Ok(())
}

fn load_sig(path: &Path) -> Result<SignatureEntry> {
    let s: StoredSig = serde_json::from_slice(&fs::read(path)?)?;
    let pk: [u8; 32] = s.public_key.try_into().map_err(|_| Error::Aion("bad public key".into()))?;
    let sig: [u8; 64] = s.signature.try_into().map_err(|_| Error::Aion("bad signature".into()))?;
    Ok(SignatureEntry::new(AuthorId(s.author_id), pk, sig))
}

/// Register authors' root keys into a fresh registry, active from `created_at`
/// (the registry epoch from which the key is authorized — RFC-0034).
fn registry_at(dir: &Path, authors: &[u64], created_at: u64) -> Result<KeyRegistry> {
    let mut reg = KeyRegistry::new();
    for &a in authors {
        let sk = load_or_make_key(dir, a)?;
        reg.register_author(AuthorId(a), sk.verifying_key(), sk.verifying_key(), created_at.max(1))
            .map_err(|e| Error::Aion(e.to_string()))?;
    }
    Ok(reg)
}

fn registry_of(dir: &Path, authors: &[u64]) -> Result<KeyRegistry> {
    registry_at(dir, authors, 1)
}

// ── federation epochs: the registry advances; grants are scoped & revocable ──

fn epoch_path(dir: &Path) -> PathBuf {
    dir.join("federation").join("epoch")
}

/// The current federation epoch (advances on lifecycle events). Default 0.
pub fn current_epoch(dir: &Path) -> Result<u64> {
    Ok(fs::read_to_string(epoch_path(dir)).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0))
}

/// Advance the federation epoch by one and return it.
pub fn advance_epoch(dir: &Path) -> Result<u64> {
    let next = current_epoch(dir)? + 1;
    fs::create_dir_all(dir.join("federation"))?;
    fs::write(epoch_path(dir), next.to_string())?;
    Ok(next)
}

/// A signed grant (recognition or delegation) with an epoch scope. `until == 0`
/// means open-ended. The scope is part of the signed statement (tamper-evident).
#[derive(Serialize, Deserialize)]
struct Grant {
    from_epoch: u64,
    until_epoch: u64,
    sig: StoredSig,
}

fn store_grant(path: &Path, from: u64, until: u64, entry: &SignatureEntry) -> Result<()> {
    let g = Grant {
        from_epoch: from,
        until_epoch: until,
        sig: StoredSig { author_id: entry.author_id, public_key: entry.public_key.to_vec(), signature: entry.signature.to_vec() },
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec(&g)?)?;
    Ok(())
}

fn load_grant(path: &Path) -> Result<Grant> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn grant_sig(g: &Grant) -> Result<SignatureEntry> {
    let pk: [u8; 32] = g.sig.public_key.clone().try_into().map_err(|_| Error::Aion("bad public key".into()))?;
    let sig: [u8; 64] = g.sig.signature.clone().try_into().map_err(|_| Error::Aion("bad signature".into()))?;
    Ok(SignatureEntry::new(AuthorId(g.sig.author_id), pk, sig))
}

/// A grant is in scope at epoch `e` iff issued by then and not past its expiry.
fn in_scope(g: &Grant, e: u64) -> bool {
    g.from_epoch <= e && (g.until_epoch == 0 || e <= g.until_epoch)
}

// ── revocation: the granting root signs a withdrawal, stamped at an epoch ─────

#[derive(Serialize)]
struct RevocationStatement<'a> {
    kind: &'a str,
    subject: &'a str,
    revoked_at: u64,
}

#[derive(Serialize, Deserialize)]
struct Revocation {
    revoked_at: u64,
    sig: StoredSig,
}

fn revocation_version(kind: &str, subject: &str, revoked_at: u64) -> Result<VersionEntry> {
    let stmt = RevocationStatement { kind, subject, revoked_at };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(revoked_at.max(1)), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

fn revocation_path(dir: &Path, kind: &str, subject: &str) -> PathBuf {
    dir.join("federation").join("revocations").join(kind).join(format!("{subject}.json"))
}

/// The granting root signs a revocation of `subject`, stamped at the current epoch.
fn revoke(dir: &Path, root: &str, kind: &str, subject: &str) -> Result<u64> {
    let at = current_epoch(dir)?;
    let author = author_for(root);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&revocation_version(kind, subject, at)?, AuthorId(author), &sk);
    let rec = Revocation {
        revoked_at: at,
        sig: StoredSig { author_id: entry.author_id, public_key: entry.public_key.to_vec(), signature: entry.signature.to_vec() },
    };
    let path = revocation_path(dir, kind, subject);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serde_json::to_vec(&rec)?)?;
    Ok(at)
}

/// The epoch at which `subject` was revoked by `root`, if a valid revocation exists.
fn revoked_at(dir: &Path, root: &str, kind: &str, subject: &str) -> Result<Option<u64>> {
    let path = revocation_path(dir, kind, subject);
    if !path.exists() {
        return Ok(None);
    }
    let rec: Revocation = serde_json::from_slice(&fs::read(&path)?)?;
    let version = revocation_version(kind, subject, rec.revoked_at)?;
    let pk: [u8; 32] = rec.sig.public_key.clone().try_into().map_err(|_| Error::Aion("bad public key".into()))?;
    let sig: [u8; 64] = rec.sig.signature.clone().try_into().map_err(|_| Error::Aion("bad signature".into()))?;
    let sigs = vec![SignatureEntry::new(AuthorId(rec.sig.author_id), pk, sig)];
    let a = author_for(root);
    let registry = registry_at(dir, &[a], rec.revoked_at)?;
    let pol = MultiSigPolicy::m_of_n(1, vec![AuthorId(a)]).map_err(|e| Error::Aion(e.to_string()))?;
    let v = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?;
    Ok(if v.threshold_met { Some(rec.revoked_at) } else { None })
}

/// A grant is active at epoch `e` iff its signature holds, `e` is within scope,
/// and it was not revoked at or before `e`.
fn active_at(sig_ok: bool, g: &Grant, revoked: Option<u64>, e: u64) -> bool {
    sig_ok && in_scope(g, e) && revoked.map(|r| e < r).unwrap_or(true)
}

// ── Layer 1: recognition cross-certificates ──────────────────────────────────

/// The signed statement: `recognizer` recognizes `recognized` (bound to its key),
/// scoped to the epoch window `[from_epoch, until_epoch]` (`until_epoch == 0` = open).
#[derive(Serialize)]
struct RecognitionStatement<'a> {
    recognizer: &'a str,
    recognized: &'a str,
    recognized_key: &'a [u8],
    from_epoch: u64,
    until_epoch: u64,
}

fn recognition_version(recognizer: &str, recognized: &Institution, from: u64, until: u64) -> Result<VersionEntry> {
    let stmt = RecognitionStatement {
        recognizer,
        recognized: &recognized.name,
        recognized_key: &recognized.public_key,
        from_epoch: from,
        until_epoch: until,
    };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(from.max(1)), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

fn recognition_path(dir: &Path, recognizer: &str, recognized: &str) -> PathBuf {
    dir.join("federation").join("recognitions").join(format!("{recognizer}__recognizes__{recognized}.sig.json"))
}

fn recognition_subject(recognizer: &str, recognized: &str) -> String {
    format!("{recognizer}__{recognized}")
}

/// Institution `recognizer` recognizes `recognized`, valid until epoch `until`
/// (`0` = open-ended), issued at the current epoch.
pub fn recognize_scoped(dir: &Path, recognizer: &str, recognized_name: &str, until: u64) -> Result<()> {
    let recognized = institution(dir, recognized_name)?;
    let from = current_epoch(dir)?;
    let author = author_for(recognizer);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&recognition_version(recognizer, &recognized, from, until)?, AuthorId(author), &sk);
    store_grant(&recognition_path(dir, recognizer, recognized_name), from, until, &entry)
}

/// Open-ended recognition (no expiry).
pub fn recognize(dir: &Path, recognizer: &str, recognized_name: &str) -> Result<()> {
    recognize_scoped(dir, recognizer, recognized_name, 0)
}

/// `recognizer` withdraws its recognition of `recognized`, effective this epoch.
pub fn revoke_recognition(dir: &Path, recognizer: &str, recognized_name: &str) -> Result<u64> {
    revoke(dir, recognizer, "recognition", &recognition_subject(recognizer, recognized_name))
}

/// The verdict of checking a recognition cross-certificate.
#[derive(Debug, Clone)]
pub struct Recognition {
    pub recognizer: String,
    pub recognized: String,
    pub valid: bool,
}

/// Verify `recognizer`'s recognition of `recognized` **as of epoch `epoch`**: the
/// signature must hold, `epoch` must be within the grant's scope, and it must not
/// have been revoked at or before `epoch`.
pub fn verify_recognition_at(dir: &Path, recognizer: &str, recognized_name: &str, epoch: u64) -> Result<Option<Recognition>> {
    let path = recognition_path(dir, recognizer, recognized_name);
    if !path.exists() {
        return Ok(None);
    }
    let recognized = institution(dir, recognized_name)?;
    let g = load_grant(&path)?;
    let version = recognition_version(recognizer, &recognized, g.from_epoch, g.until_epoch)?;
    let sigs = vec![grant_sig(&g)?];
    let r_author = author_for(recognizer);
    let registry = registry_at(dir, &[r_author], g.from_epoch)?;
    let pol = MultiSigPolicy::m_of_n(1, vec![AuthorId(r_author)]).map_err(|e| Error::Aion(e.to_string()))?;
    let sig_ok = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?.threshold_met;
    let revoked = revoked_at(dir, recognizer, "recognition", &recognition_subject(recognizer, recognized_name))?;
    Ok(Some(Recognition {
        recognizer: recognizer.to_string(),
        recognized: recognized_name.to_string(),
        valid: active_at(sig_ok, &g, revoked, epoch),
    }))
}

/// Verify a recognition as of the current epoch (reflects expiry and revocation).
pub fn verify_recognition(dir: &Path, recognizer: &str, recognized_name: &str) -> Result<Option<Recognition>> {
    let e = current_epoch(dir)?;
    verify_recognition_at(dir, recognizer, recognized_name, e)
}

/// True iff `a` and `b` each hold a valid recognition certificate of the other.
pub fn mutually_recognized(dir: &Path, a: &str, b: &str) -> Result<bool> {
    let ab = verify_recognition(dir, a, b)?.map(|r| r.valid).unwrap_or(false);
    let ba = verify_recognition(dir, b, a)?.map(|r| r.valid).unwrap_or(false);
    Ok(ab && ba)
}

// ── Layer 2: co-accreditation (cross-institution quorum) ─────────────────────

/// A joint program's accreditation policy: a K-of-N quorum of institutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramPolicy {
    pub program: String,
    pub threshold: u32,
    pub signers: Vec<String>,
}

#[derive(Serialize)]
struct ProgramStatement<'a> {
    program: &'a str,
    signers: &'a [String],
}

fn program_version(policy: &ProgramPolicy) -> Result<VersionEntry> {
    let stmt = ProgramStatement { program: &policy.program, signers: &policy.signers };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(1), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

fn program_dir(dir: &Path, program: &str) -> PathBuf {
    dir.join("federation").join("programs").join(program)
}

/// Declare a joint program accredited by a K-of-N quorum of institutions.
pub fn set_joint_accreditation(dir: &Path, program: &str, threshold: u32, institutions: &[String]) -> Result<()> {
    let mut s = institutions.to_vec();
    s.sort();
    s.dedup();
    let policy = ProgramPolicy { program: program.to_string(), threshold, signers: s };
    let pdir = program_dir(dir, program);
    fs::create_dir_all(&pdir)?;
    fs::write(pdir.join("policy.json"), serde_json::to_vec_pretty(&policy)?)?;
    Ok(())
}

/// Read a joint program's accreditation policy, if any.
pub fn joint_policy(dir: &Path, program: &str) -> Option<ProgramPolicy> {
    fs::read(program_dir(dir, program).join("policy.json")).ok().and_then(|b| serde_json::from_slice(&b).ok())
}

/// An institution endorses (co-signs) a joint program.
pub fn endorse_program(dir: &Path, program: &str, institution_name: &str) -> Result<()> {
    let policy = joint_policy(dir, program).ok_or_else(|| Error::Refused(format!("no joint program {program}")))?;
    let author = author_for(institution_name);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&program_version(&policy)?, AuthorId(author), &sk);
    store_sig(&program_dir(dir, program).join(format!("{author}.sig.json")), &entry)
}

fn load_program_sigs(dir: &Path, program: &str) -> Result<Vec<SignatureEntry>> {
    let pdir = program_dir(dir, program);
    let mut out = Vec::new();
    if !pdir.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(&pdir)? {
        let path = entry?.path();
        if path.file_name().and_then(|n| n.to_str()).map(|n| n.ends_with(".sig.json")) != Some(true) {
            continue;
        }
        out.push(load_sig(&path)?);
    }
    Ok(out)
}

/// The outcome of checking a joint program's cross-institution quorum.
#[derive(Debug, Clone)]
pub struct JointAccreditation {
    pub program: String,
    pub threshold: u32,
    pub valid_count: u32,
    pub met: bool,
    pub signing_institutions: Vec<String>,
}

/// Verify a joint program's accreditation: count distinct authorized institutions
/// whose endorsement of the program statement verifies. `Ok(None)` if no policy.
pub fn verify_joint_accreditation(dir: &Path, program: &str) -> Result<Option<JointAccreditation>> {
    let Some(policy) = joint_policy(dir, program) else {
        return Ok(None);
    };
    let version = program_version(&policy)?;
    let mut authorized = Vec::new();
    let mut by_id: HashMap<u64, String> = HashMap::new();
    for inst in &policy.signers {
        let a = author_for(inst);
        authorized.push(AuthorId(a));
        by_id.insert(a, inst.clone());
    }
    let registry = registry_of(dir, &authorized.iter().map(|a| a.0).collect::<Vec<_>>())?;
    let sigs = load_program_sigs(dir, program)?;
    let pol = MultiSigPolicy::m_of_n(policy.threshold, authorized).map_err(|e| Error::Aion(e.to_string()))?;
    let v = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?;
    let mut signing_institutions: Vec<String> = v.valid_signers.iter().filter_map(|a| by_id.get(&a.0).cloned()).collect();
    signing_institutions.sort();
    Ok(Some(JointAccreditation {
        program: policy.program,
        threshold: v.required,
        valid_count: v.valid_count,
        met: v.threshold_met,
        signing_institutions,
    }))
}

// ── Layer 3: issuer binding — an institution vouches for a credential ────────

/// The statement an issuing institution counter-signs over a credential.
#[derive(Serialize)]
struct IssuanceStatement<'a> {
    issuer: &'a str,
    learner: &'a str,
    lesson_id: &'a str,
    credential_file_id: u64,
    credential_hash: Vec<u8>,
}

fn issuance_version(issuer: &str, learner: &str, lesson_id: &str, file_id: u64, content: &[u8]) -> Result<VersionEntry> {
    let stmt = IssuanceStatement {
        issuer,
        learner,
        lesson_id,
        credential_file_id: file_id,
        credential_hash: hash(content).to_vec(),
    };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(1), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

fn issuance_path(dir: &Path, learner: &str, lesson_id: &str) -> PathBuf {
    dir.join("federation").join("issued").join(learner).join(format!("{lesson_id}.issuer.sig.json"))
}

/// An institution root counter-signs (vouches for) a professor-issued credential,
/// binding it to the issuer. The signature covers the exact credential content, so
/// any later tampering breaks it.
pub fn bind_issuer(dir: &Path, issuer: &str, learner: &str, lesson_id: &str) -> Result<()> {
    let info = crate::read_credential(dir, learner, lesson_id)?
        .ok_or_else(|| Error::Refused(format!("no credential for {learner}/{lesson_id}")))?;
    let author = author_for(issuer);
    let sk = load_or_make_key(dir, author)?;
    let version = issuance_version(issuer, learner, lesson_id, info.file_id, &info.content)?;
    let entry = sign_attestation(&version, AuthorId(author), &sk);
    store_sig(&issuance_path(dir, learner, lesson_id), &entry)
}

// ── Layer 4: delegation — the root vouches for a faculty key ONCE ────────────

/// The statement an issuing institution signs to delegate a faculty key, scoped
/// to the epoch window `[from_epoch, until_epoch]` (`until_epoch == 0` = open).
#[derive(Serialize)]
struct DelegationStatement<'a> {
    issuer: &'a str,
    professor: &'a str,
    professor_key: &'a [u8],
    from_epoch: u64,
    until_epoch: u64,
}

fn delegation_version(issuer: &str, professor: &str, professor_key: &[u8], from: u64, until: u64) -> Result<VersionEntry> {
    let stmt = DelegationStatement { issuer, professor, professor_key, from_epoch: from, until_epoch: until };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(from.max(1)), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

fn delegation_path(dir: &Path, issuer: &str, professor: &str) -> PathBuf {
    let a = author_for(professor);
    dir.join("federation").join("delegations").join(issuer).join(format!("{a}.deleg.sig.json"))
}

fn delegation_subject(issuer: &str, professor: &str) -> String {
    format!("{issuer}__{}", author_for(professor))
}

fn professor_key(dir: &Path, professor: &str) -> Result<Vec<u8>> {
    let sk = load_or_make_key(dir, author_for(professor))?;
    Ok(sk.verifying_key().to_bytes().to_vec())
}

/// Institution `issuer` delegates `professor`'s faculty key, valid until epoch
/// `until` (`0` = open-ended), issued at the current epoch — vouched ONCE.
pub fn delegate_scoped(dir: &Path, issuer: &str, professor: &str, until: u64) -> Result<()> {
    let p_key = professor_key(dir, professor)?;
    let from = current_epoch(dir)?;
    let author = author_for(issuer);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&delegation_version(issuer, professor, &p_key, from, until)?, AuthorId(author), &sk);
    store_grant(&delegation_path(dir, issuer, professor), from, until, &entry)
}

/// Open-ended delegation (no expiry).
pub fn delegate(dir: &Path, issuer: &str, professor: &str) -> Result<()> {
    delegate_scoped(dir, issuer, professor, 0)
}

/// `issuer` withdraws `professor`'s delegation, effective this epoch.
pub fn revoke_delegation(dir: &Path, issuer: &str, professor: &str) -> Result<u64> {
    revoke(dir, issuer, "delegation", &delegation_subject(issuer, professor))
}

/// Verify the issuer's delegation of `professor`'s faculty key **as of epoch
/// `epoch`** (binds the exact key; respects scope and revocation). `Ok(None)` if absent.
pub fn verify_delegation_at(dir: &Path, issuer: &str, professor: &str, epoch: u64) -> Result<Option<bool>> {
    let path = delegation_path(dir, issuer, professor);
    if !path.exists() {
        return Ok(None);
    }
    let g = load_grant(&path)?;
    let version = delegation_version(issuer, professor, &professor_key(dir, professor)?, g.from_epoch, g.until_epoch)?;
    let sigs = vec![grant_sig(&g)?];
    let a = author_for(issuer);
    let registry = registry_at(dir, &[a], g.from_epoch)?;
    let pol = MultiSigPolicy::m_of_n(1, vec![AuthorId(a)]).map_err(|e| Error::Aion(e.to_string()))?;
    let sig_ok = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?.threshold_met;
    let revoked = revoked_at(dir, issuer, "delegation", &delegation_subject(issuer, professor))?;
    Ok(Some(active_at(sig_ok, &g, revoked, epoch)))
}

/// Verify a delegation as of the current epoch (reflects expiry and revocation).
pub fn verify_delegation(dir: &Path, issuer: &str, professor: &str) -> Result<Option<bool>> {
    let e = current_epoch(dir)?;
    verify_delegation_at(dir, issuer, professor, e)
}

/// The verdict of verifying a credential under a (possibly recognized) issuer.
#[derive(Debug, Clone)]
pub struct IssuedCredential {
    pub issuer: String,
    pub verifier: String,
    /// professor signature valid + credential untampered.
    pub credential_valid: bool,
    /// the issuing institution's root counter-signed this specific credential.
    pub issuer_vouched: bool,
    /// the signing faculty key is covered by a one-time issuer delegation.
    pub faculty_delegated: bool,
    /// the verifier holds a valid recognition cross-certificate of the issuer.
    pub issuer_recognized: bool,
    /// recognized issuer + valid credential + (per-credential vouch OR delegated faculty).
    pub accepted: bool,
}

/// Verify a credential as `verifier` would, **as of epoch `epoch`**: the professor
/// signature must hold, the `issuer` must have backed it (per-credential vouch OR a
/// faculty delegation active at `epoch`), and `verifier` must recognize `issuer` at
/// `epoch`. Checking as of the credential's issue epoch keeps a diploma valid even
/// after a later revocation; checking at the current epoch reflects withdrawals.
pub fn verify_issued_credential_at(
    dir: &Path,
    verifier: &str,
    issuer: &str,
    learner: &str,
    lesson_id: &str,
    epoch: u64,
) -> Result<Option<IssuedCredential>> {
    let Some(info) = crate::read_credential(dir, learner, lesson_id)? else {
        return Ok(None);
    };
    let ipath = issuance_path(dir, learner, lesson_id);
    let issuer_vouched = if ipath.exists() {
        let version = issuance_version(issuer, learner, lesson_id, info.file_id, &info.content)?;
        let sigs = vec![load_sig(&ipath)?];
        let a = author_for(issuer);
        let registry = registry_of(dir, &[a])?;
        let pol = MultiSigPolicy::m_of_n(1, vec![AuthorId(a)]).map_err(|e| Error::Aion(e.to_string()))?;
        verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?.threshold_met
    } else {
        false
    };
    let faculty_delegated = verify_delegation_at(dir, issuer, &info.professor, epoch)?.unwrap_or(false);
    let issuer_recognized = verify_recognition_at(dir, verifier, issuer, epoch)?.map(|r| r.valid).unwrap_or(false);
    let credential_valid = info.valid;
    Ok(Some(IssuedCredential {
        issuer: issuer.to_string(),
        verifier: verifier.to_string(),
        credential_valid,
        issuer_vouched,
        faculty_delegated,
        issuer_recognized,
        accepted: credential_valid && issuer_recognized && (issuer_vouched || faculty_delegated),
    }))
}

/// Verify a credential as of the current epoch (reflects expiry and revocation).
pub fn verify_issued_credential(
    dir: &Path,
    verifier: &str,
    issuer: &str,
    learner: &str,
    lesson_id: &str,
) -> Result<Option<IssuedCredential>> {
    let e = current_epoch(dir)?;
    verify_issued_credential_at(dir, verifier, issuer, learner, lesson_id, e)
}

// ── Layer 6: snapshots — a signed, point-in-time checkpoint of the state ─────

fn collect_files(root: &Path, base: &Path, skip: &str, out: &mut Vec<(String, Vec<u8>)>) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let p = entry?.path();
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != skip {
                collect_files(&p, base, skip, out)?;
            }
        } else {
            let rel = p.strip_prefix(base).unwrap_or(&p).to_string_lossy().to_string();
            out.push((rel, fs::read(&p)?));
        }
    }
    Ok(())
}

/// A deterministic digest of the entire federation state (recognitions,
/// delegations, revocations, programs, epoch) — excluding the snapshots themselves.
fn state_hash(dir: &Path) -> Result<[u8; 32]> {
    let fed = dir.join("federation");
    let mut files = Vec::new();
    collect_files(&fed, &fed, "snapshots", &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut acc = Vec::new();
    for (rel, content) in &files {
        acc.extend_from_slice(rel.as_bytes());
        acc.push(0);
        acc.extend_from_slice(&hash(content));
    }
    Ok(hash(&acc))
}

#[derive(Serialize)]
struct SnapshotStatement {
    epoch: u64,
    state_hash: Vec<u8>,
}

fn snapshot_version(epoch: u64, state_hash: &[u8]) -> Result<VersionEntry> {
    let stmt = SnapshotStatement { epoch, state_hash: state_hash.to_vec() };
    let rules_hash = hash(&serde_json::to_vec(&stmt)?);
    Ok(VersionEntry::new(VersionNumber(epoch.max(1)), [0u8; 32], rules_hash, AuthorId(0), FED_TS, 0, 0))
}

#[derive(Serialize, Deserialize)]
struct SnapshotRecord {
    id: u64,
    epoch: u64,
    state_hash: Vec<u8>,
    signers: Vec<String>,
    sigs: Vec<StoredSig>,
}

fn snapshots_dir(dir: &Path) -> PathBuf {
    dir.join("federation").join("snapshots")
}

/// Institution `by` signs a checkpoint of the current federation state at the
/// current epoch. Signing an already-checkpointed (epoch, state) co-signs the same
/// snapshot — so both parties can attest the exact same state. Returns the id.
pub fn take_snapshot(dir: &Path, by: &str) -> Result<u64> {
    let epoch = current_epoch(dir)?;
    let sh = state_hash(dir)?.to_vec();
    let sdir = snapshots_dir(dir);
    fs::create_dir_all(&sdir)?;

    let mut existing: Option<SnapshotRecord> = None;
    let mut max_id = 0u64;
    for e in fs::read_dir(&sdir)? {
        let p = e?.path();
        if p.extension().and_then(|x| x.to_str()) != Some("json") {
            continue;
        }
        let rec: SnapshotRecord = serde_json::from_slice(&fs::read(&p)?)?;
        max_id = max_id.max(rec.id);
        if rec.epoch == epoch && rec.state_hash == sh {
            existing = Some(rec);
        }
    }

    let author = author_for(by);
    let sk = load_or_make_key(dir, author)?;
    let entry = sign_attestation(&snapshot_version(epoch, &sh)?, AuthorId(author), &sk);
    let stored = StoredSig { author_id: entry.author_id, public_key: entry.public_key.to_vec(), signature: entry.signature.to_vec() };

    let mut rec = existing.unwrap_or(SnapshotRecord { id: max_id + 1, epoch, state_hash: sh, signers: vec![], sigs: vec![] });
    if !rec.signers.iter().any(|s| s == by) {
        rec.signers.push(by.to_string());
        rec.sigs.push(stored);
    }
    let id = rec.id;
    fs::write(sdir.join(format!("{id}.json")), serde_json::to_vec_pretty(&rec)?)?;
    Ok(id)
}

/// The result of verifying a snapshot: whether the on-disk state still matches it,
/// and which institutions' signatures over the checkpoint are valid.
#[derive(Debug, Clone)]
pub struct SnapshotVerification {
    pub id: u64,
    pub epoch: u64,
    pub state_hash: Vec<u8>,
    /// the current federation state still equals the checkpointed state.
    pub matches_current: bool,
    pub valid_signers: Vec<String>,
    pub invalid_signers: Vec<String>,
}

/// Verify a snapshot: confirm each signer's signature over `(epoch, state_hash)`,
/// and report whether the current state still matches (drift detection). The
/// signatures stay valid over the checkpointed state even after the state moves on.
pub fn verify_snapshot(dir: &Path, id: u64) -> Result<Option<SnapshotVerification>> {
    let path = snapshots_dir(dir).join(format!("{id}.json"));
    if !path.exists() {
        return Ok(None);
    }
    let rec: SnapshotRecord = serde_json::from_slice(&fs::read(&path)?)?;
    let version = snapshot_version(rec.epoch, &rec.state_hash)?;
    let mut valid = Vec::new();
    let mut invalid = Vec::new();
    for (name, s) in rec.signers.iter().zip(rec.sigs.iter()) {
        let pk: [u8; 32] = s.public_key.clone().try_into().map_err(|_| Error::Aion("bad public key".into()))?;
        let sig: [u8; 64] = s.signature.clone().try_into().map_err(|_| Error::Aion("bad signature".into()))?;
        let sigs = vec![SignatureEntry::new(AuthorId(s.author_id), pk, sig)];
        let a = author_for(name);
        let registry = registry_at(dir, &[a], rec.epoch)?;
        let pol = MultiSigPolicy::m_of_n(1, vec![AuthorId(a)]).map_err(|e| Error::Aion(e.to_string()))?;
        let ok = verify_multisig(&version, &sigs, &pol, &registry).map_err(|e| Error::Aion(e.to_string()))?.threshold_met;
        if ok {
            valid.push(name.clone());
        } else {
            invalid.push(name.clone());
        }
    }
    let matches_current = state_hash(dir)?.to_vec() == rec.state_hash;
    Ok(Some(SnapshotVerification { id: rec.id, epoch: rec.epoch, state_hash: rec.state_hash, matches_current, valid_signers: valid, invalid_signers: invalid }))
}

// ── enumeration (for dashboards) ─────────────────────────────────────────────

/// All recognition cross-certificates on disk, as `(recognizer, recognized)`.
pub fn list_recognitions(dir: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir.join("federation").join("recognitions")) {
        for e in rd.flatten() {
            if let Some(stem) = e.path().file_name().and_then(|n| n.to_str()).and_then(|n| n.strip_suffix(".sig.json")) {
                if let Some((by, of)) = stem.split_once("__recognizes__") {
                    out.push((by.to_string(), of.to_string()));
                }
            }
        }
    }
    out.sort();
    out
}

/// All declared joint programs.
pub fn list_programs(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir.join("federation").join("programs")) {
        for e in rd.flatten() {
            if e.path().is_dir() {
                if let Some(n) = e.path().file_name().and_then(|n| n.to_str()) {
                    out.push(n.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

/// All snapshot ids on disk.
pub fn list_snapshots(dir: &Path) -> Vec<u64> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(snapshots_dir(dir)) {
        for e in rd.flatten() {
            if let Some(id) = e.path().file_stem().and_then(|s| s.to_str()).and_then(|s| s.parse::<u64>().ok()) {
                out.push(id);
            }
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("aion-edu-fed-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn mutual_recognition_round_trips() {
        let dir = tmp("recog");
        recognize(&dir, "aion-edu", "partner-u").unwrap();
        recognize(&dir, "partner-u", "aion-edu").unwrap();
        assert!(verify_recognition(&dir, "aion-edu", "partner-u").unwrap().unwrap().valid);
        assert!(mutually_recognized(&dir, "aion-edu", "partner-u").unwrap());
        // an unsigned direction is not recognized
        assert!(verify_recognition(&dir, "aion-edu", "rogue-u").unwrap().is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn credential_accepted_only_under_a_recognized_vouching_issuer() {
        let dir = tmp("issued");
        // an aion-edu professor issues a credential to alice
        let rref = crate::RubricRef { file_id: 1, version: 1 };
        crate::commit_credential(&dir, "alice", "lamport", "cs340-u1-l1", "ok", rref).unwrap();

        // partner-u recognizes aion-edu, but the issuer hasn't vouched yet
        recognize(&dir, "partner-u", "aion-edu").unwrap();
        let v0 = verify_issued_credential(&dir, "partner-u", "aion-edu", "alice", "cs340-u1-l1").unwrap().unwrap();
        assert!(v0.credential_valid && v0.issuer_recognized && !v0.issuer_vouched && !v0.accepted,
            "a recognized issuer that hasn't vouched is not enough");

        // aion-edu's root counter-signs the credential -> now accepted by partner-u
        bind_issuer(&dir, "aion-edu", "alice", "cs340-u1-l1").unwrap();
        let v1 = verify_issued_credential(&dir, "partner-u", "aion-edu", "alice", "cs340-u1-l1").unwrap().unwrap();
        assert!(v1.accepted, "valid + vouched + recognized -> accepted");

        // a verifier that does NOT recognize aion-edu rejects the same credential
        let v2 = verify_issued_credential(&dir, "stranger-u", "aion-edu", "alice", "cs340-u1-l1").unwrap().unwrap();
        assert!(v2.issuer_vouched && !v2.issuer_recognized && !v2.accepted,
            "an unrecognized issuer is not accepted, even though it vouched");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn snapshot_checkpoints_and_detects_drift() {
        let dir = tmp("snap");
        recognize(&dir, "partner-u", "aion-edu").unwrap();
        delegate(&dir, "aion-edu", "lynch").unwrap();

        // both parties co-sign one checkpoint of the agreed state
        let id = take_snapshot(&dir, "aion-edu").unwrap();
        let id2 = take_snapshot(&dir, "partner-u").unwrap();
        assert_eq!(id, id2, "co-signing the same (epoch, state) shares one snapshot");
        let v = verify_snapshot(&dir, id).unwrap().unwrap();
        assert!(v.matches_current, "a fresh snapshot matches the current state");
        assert_eq!(v.valid_signers.len(), 2);
        assert!(v.valid_signers.contains(&"aion-edu".to_string()) && v.valid_signers.contains(&"partner-u".to_string()));

        // the state moves on -> drift detected, but the signatures still attest the OLD state
        advance_epoch(&dir).unwrap();
        revoke_delegation(&dir, "aion-edu", "lynch").unwrap();
        let v2 = verify_snapshot(&dir, id).unwrap().unwrap();
        assert!(!v2.matches_current, "state drifted from the checkpoint");
        assert_eq!(v2.valid_signers.len(), 2, "signatures remain valid over the checkpointed state");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scope_and_revoke_via_epochs() {
        let dir = tmp("epoch");
        crate::commit_credential(&dir, "alice", "lynch", "cs440-u1-l1", "ok", crate::RubricRef { file_id: 1, version: 1 }).unwrap();

        // epoch 0: partner-u recognizes aion-edu and aion-edu delegates lynch
        recognize(&dir, "partner-u", "aion-edu").unwrap();
        delegate(&dir, "aion-edu", "lynch").unwrap();
        let v = verify_issued_credential(&dir, "partner-u", "aion-edu", "alice", "cs440-u1-l1").unwrap().unwrap();
        assert!(v.accepted, "accepted while the delegation is live");

        // revoke the delegation at epoch 1
        advance_epoch(&dir).unwrap();
        let r = revoke_delegation(&dir, "aion-edu", "lynch").unwrap();
        assert_eq!(r, 1);

        // present NOW (epoch 1): the delegation is withdrawn -> rejected
        let now = verify_issued_credential(&dir, "partner-u", "aion-edu", "alice", "cs440-u1-l1").unwrap().unwrap();
        assert!(!now.faculty_delegated && !now.accepted, "revoked delegation blocks new acceptance");

        // present AS OF epoch 0 (when it was earned): the diploma still verifies
        let then = verify_issued_credential_at(&dir, "partner-u", "aion-edu", "alice", "cs440-u1-l1", 0).unwrap().unwrap();
        assert!(then.faculty_delegated && then.accepted, "a diploma earned before revocation stays valid as-of issue");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scoped_recognition_expires() {
        let dir = tmp("expire");
        recognize_scoped(&dir, "partner-u", "aion-edu", 2).unwrap(); // valid through epoch 2
        assert!(verify_recognition(&dir, "partner-u", "aion-edu").unwrap().unwrap().valid, "active at epoch 0");
        advance_epoch(&dir).unwrap();
        advance_epoch(&dir).unwrap();
        assert!(verify_recognition(&dir, "partner-u", "aion-edu").unwrap().unwrap().valid, "still active at epoch 2");
        advance_epoch(&dir).unwrap(); // epoch 3 > 2
        assert!(!verify_recognition(&dir, "partner-u", "aion-edu").unwrap().unwrap().valid, "expired past epoch 2");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn delegation_lets_the_root_vouch_for_faculty_once() {
        let dir = tmp("deleg");
        let rr = |f| crate::RubricRef { file_id: f, version: 1 };
        crate::commit_credential(&dir, "bob", "lamport", "cs340-u1-l1", "ok", rr(1)).unwrap();
        crate::commit_credential(&dir, "carol", "lamport", "cs440-u1-l1", "ok", rr(2)).unwrap();
        recognize(&dir, "partner-u", "aion-edu").unwrap();

        // not delegated, not vouched -> rejected
        let v0 = verify_issued_credential(&dir, "partner-u", "aion-edu", "bob", "cs340-u1-l1").unwrap().unwrap();
        assert!(!v0.faculty_delegated && !v0.accepted);

        // the root delegates lamport's faculty key ONCE
        delegate(&dir, "aion-edu", "lamport").unwrap();

        // BOTH of lamport's credentials are now accepted — no per-credential vouch
        let a = verify_issued_credential(&dir, "partner-u", "aion-edu", "bob", "cs340-u1-l1").unwrap().unwrap();
        let b = verify_issued_credential(&dir, "partner-u", "aion-edu", "carol", "cs440-u1-l1").unwrap().unwrap();
        assert!(a.faculty_delegated && a.accepted && !a.issuer_vouched, "accepted via delegation, not countersignature");
        assert!(b.faculty_delegated && b.accepted);

        // a credential by a NON-delegated professor is not accepted via delegation
        crate::commit_credential(&dir, "dave", "strang", "math110-u1-l1", "ok", rr(3)).unwrap();
        let c = verify_issued_credential(&dir, "partner-u", "aion-edu", "dave", "math110-u1-l1").unwrap().unwrap();
        assert!(!c.faculty_delegated && !c.accepted, "only delegated faculty are issuer-backed");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn joint_quorum_must_span_both_institutions() {
        let dir = tmp("joint");
        let signers = ["aion-edu".to_string(), "aion-edu-dean".to_string(), "partner-u".to_string()];
        set_joint_accreditation(&dir, "jd-systems", 3, &signers).unwrap();

        // only the two aion-edu signers endorse -> 2 of 3, NOT met
        endorse_program(&dir, "jd-systems", "aion-edu").unwrap();
        endorse_program(&dir, "jd-systems", "aion-edu-dean").unwrap();
        let j = verify_joint_accreditation(&dir, "jd-systems").unwrap().unwrap();
        assert!(!j.met && j.valid_count == 2, "one university alone cannot meet the joint quorum");

        // the partner co-signs -> quorum met, spanning both institutions
        endorse_program(&dir, "jd-systems", "partner-u").unwrap();
        let j2 = verify_joint_accreditation(&dir, "jd-systems").unwrap().unwrap();
        assert!(j2.met && j2.valid_count == 3);
        assert!(j2.signing_institutions.contains(&"partner-u".to_string()));
        assert!(j2.signing_institutions.contains(&"aion-edu".to_string()));

        let _ = fs::remove_dir_all(&dir);
    }
}
