# aion-edu — an educational kernel as a universe

A Rust port of Sigma University: a small, sharp **kernel** that treats a
university as a runtime. The faculty are master-teacher personas, the curriculum
is data, and **aion-context** is the trust spine — rubrics and credentials are
tamper-evident, hash-chained, Ed25519-signed artifacts, verified **in-process**.

## First principle: expansion is a one-file act
Adding a professor or a course must require **dropping a single file** — no
central registry to edit. We get this from the same `inventory` self-registration
pattern used in `vervet`: a professor `inventory::submit!`s itself as a
`&'static dyn Professor`; a course `inventory::submit!`s a builder. The kernel
*collects* them at startup. The registry has no list to maintain.

```
add a professor  →  crates/aion-edu-faculty/src/<name>.rs   (one inventory::submit!)
add a course     →  crates/aion-edu-curriculum/src/<id>.rs  (one inventory::submit!)
```

## Workspace (Tiger Style: ≤60-line fns, MSRV-pinned, no panics in libs)
| crate | role |
|-------|------|
| **aion-edu-core** | the universe: `Professor` trait, curriculum types (`Course`/`Lesson`/`Rubric`), the `inventory` registries, and the prerequisite **planner**. Pure, no I/O. |
| **aion-edu-provenance** | wraps `aion-context` (path dep) **in-process**: seal a rubric to a signed `.aion`, verify it, bind a credential to `{file_id, version}`. The showcase. |
| **aion-edu-faculty** | concrete professors — one file each, `inventory::submit!(&X as &dyn Professor)`. |
| **aion-edu-curriculum** | concrete courses — one file each, `inventory::submit!(CourseRegistration{ build })`. |
| **aion-edu-cli** | the `aion-edu` binary: `catalog`, `plan`, `faculty`, `seal`, `verify`. Force-links faculty+curriculum so their inventory entries register. |

## The trust spine (aion-context, in-process — no subprocess)
- `crypto::SigningKey::generate` / `verifying_key` — per-professor keys.
- `operations::init_file` — seal a rubric's canonical JSON into a signed `.aion`.
- `operations::verify_file(&path, &KeyRegistry)` → `VerificationReport{ file_id,
  version_count, is_valid }` — the offline diploma check.
- `operations::show_current_rules` — read the committed payload back (for binding).
- `key_registry::KeyRegistry::register_author` — pin a professor's key.

Four guarantees, same as the Python original, now native: rubric integrity
(verify-before-teach), credential provenance, rubric↔credential binding, K-of-N
governance. This crate is the layer that makes "mastery" a *proof*, not a record.

## Status / roadmap
- **Phase 0 (this milestone):** workspace + core (traits, types, registries,
  planner) + provenance (seal/verify, in-process aion-context) + one professor
  (Lamport) + one course (cs340) + CLI. `cargo build` + `cargo test` green;
  inventory proven to pick up plug-ins; planner topo-sort tested; seal→verify→
  tamper proof tested against real aion-context.
- **Phase 1:** the teaching loop — a `Backend` trait (LLM tool-use) with a Rust
  Anthropic client; mastery ledger; credential commit + binding.
- **Phase 2:** governance (K-of-N), web entry, port the full faculty/curriculum.
