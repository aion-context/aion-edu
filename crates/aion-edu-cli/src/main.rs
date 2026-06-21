//! `aion-edu` — the kernel CLI.
//!
//! Force-links the faculty and curriculum crates so their `inventory::submit!`
//! entries register at startup, then drives the registry, planner, and the
//! aion-context provenance layer.

#![forbid(unsafe_code)]

// Force-link the plug-in crates (otherwise the linker drops them and their
// inventory entries never register). Same pattern as vervet's technique crate.
use aion_edu_curriculum as _;
use aion_edu_faculty as _;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::ExitCode;

use aion_edu_core::{planner, registry};

const DATA_DIR: &str = "aion-edu-data";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let rest = &args.get(1..).unwrap_or(&[]);
    match cmd {
        "catalog" => cmd_catalog(),
        "faculty" => cmd_faculty(),
        "plan" => return cmd_plan(rest),
        "seal" => return cmd_seal(),
        "verify" => return cmd_verify(rest),
        "teach" => return cmd_teach(rest),
        "govern" => return cmd_govern(rest),
        "endorse" => return cmd_endorse(rest),
        "verify-diploma" => return cmd_verify_diploma(rest),
        "federate" => return cmd_federate(rest),
        "serve" => return cmd_serve(rest),
        _ => usage(),
    }
    ExitCode::SUCCESS
}

fn usage() {
    println!(
        "aion-edu — educational kernel\n\n\
         catalog            list courses and lessons\n\
         faculty            list registered professors\n\
         plan <target>      prerequisite path to a lesson/course/concept\n\
         seal               sign every lesson rubric (aion-context)\n\
         verify <lesson>    verify a sealed rubric offline\n\
         teach <lesson> --learner <name>   teach a lesson live (needs ANTHROPIC_API_KEY)\n\
         govern <lesson> --threshold K --signers a,b,c   set a K-of-N rubric quorum\n\
         endorse <lesson> --by <prof>      faculty signs endorsement of a rubric\n\
         verify-diploma <file.json>   verify a downloaded diploma offline (no data dir needed)\n\
         federate <subcommand>   accreditation federation (see `federate`)\n\
         serve [--port N]   launch the web entry (streams lessons over SSE)"
    );
}

fn federate_usage() {
    println!(
        "aion-edu federate — accreditation federation (aion-context)\n\n\
         recognize <by> <of> [--until <epoch>]   <by> cross-certifies <of>, optionally expiring (Layer 1)\n\
         revoke-recognition <by> <of>   <by> withdraws its recognition of <of>\n\
         status <a> <b>               show mutual recognition between two institutions\n\
         epoch | advance              show / advance the federation epoch\n\
         accredit <program> --threshold K --signers a,b,c   declare a joint K-of-N program (Layer 2)\n\
         endorse <program> --by <institution>               an institution co-signs a joint program\n\
         verify <program>             check a joint program's cross-institution quorum\n\
         delegate <professor> --by <institution> [--until <epoch>]   vouch for a faculty key ONCE (all its credentials)\n\
         revoke-delegation <professor> --by <institution>   withdraw a faculty delegation\n\
         issue <learner> <lesson> --by <institution>        institution counter-signs one specific credential\n\
         present <learner> <lesson> --issuer <i> --to <v> [--as-of <epoch>]   verify a credential (default: now)\n\
         snapshot --by <institution>   sign a checkpoint of the whole federation state\n\
         snapshot-verify <id>          verify a snapshot's signatures + detect state drift"
    );
}

fn cmd_verify_diploma(rest: &[String]) -> ExitCode {
    let Some(file) = rest.first() else {
        eprintln!("usage: aion-edu verify-diploma <diploma.json>");
        return ExitCode::FAILURE;
    };
    let bytes = match std::fs::read(file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {file}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let diploma: aion_edu_provenance::Diploma = match serde_json::from_slice(&bytes) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{file} is not a diploma document: {e}");
            return ExitCode::FAILURE;
        }
    };
    match aion_edu_provenance::verify_diploma(&diploma) {
        Ok(v) => {
            println!(
                "diploma: {} · {} · Prof. {} · issued by {}",
                diploma.student, diploma.lesson_id, diploma.professor, diploma.issuer
            );
            println!("  signed credential verifies: {}", v.credential_verifies);
            println!("  file_id matches claim:      {}", v.file_id_match);
            println!("  claims match credential:    {}", v.claims_match);
            println!("  => {}", if v.authentic { "AUTHENTIC" } else { "NOT AUTHENTIC" });
            println!("  {}", v.detail);
            if v.authentic {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(e) => {
            eprintln!("verify-diploma failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_federate(rest: &[String]) -> ExitCode {
    let dir = Path::new(DATA_DIR);
    let sub = rest.first().map(String::as_str).unwrap_or("help");
    let a = &rest.get(1..).unwrap_or(&[]);
    match sub {
        "recognize" => {
            let (Some(by), Some(of)) = (a.first(), a.get(1)) else {
                eprintln!("usage: aion-edu federate recognize <by> <of> [--until <epoch>]");
                return ExitCode::FAILURE;
            };
            let until = flag(a, "--until").and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            match aion_edu_provenance::recognize_scoped(dir, by, of, until) {
                Ok(()) => {
                    let scope = if until == 0 { "open-ended".to_string() } else { format!("until epoch {until}") };
                    println!("{by} recognized {of} ({scope})");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("recognize failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "revoke-recognition" => {
            let (Some(by), Some(of)) = (a.first(), a.get(1)) else {
                eprintln!("usage: aion-edu federate revoke-recognition <by> <of>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::revoke_recognition(dir, by, of) {
                Ok(at) => {
                    println!("{by} revoked its recognition of {of}, effective epoch {at}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("revoke failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "epoch" => {
            println!("current federation epoch: {}", aion_edu_provenance::current_epoch(dir).unwrap_or(0));
            ExitCode::SUCCESS
        }
        "advance" => match aion_edu_provenance::advance_epoch(dir) {
            Ok(e) => {
                println!("federation epoch advanced to {e}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("advance failed: {e}");
                ExitCode::FAILURE
            }
        },
        "status" => {
            let (Some(x), Some(y)) = (a.first(), a.get(1)) else {
                eprintln!("usage: aion-edu federate status <a> <b>");
                return ExitCode::FAILURE;
            };
            let fwd = aion_edu_provenance::verify_recognition(dir, x, y).ok().flatten().map(|r| r.valid).unwrap_or(false);
            let rev = aion_edu_provenance::verify_recognition(dir, y, x).ok().flatten().map(|r| r.valid).unwrap_or(false);
            let mutual = aion_edu_provenance::mutually_recognized(dir, x, y).unwrap_or(false);
            println!("{x} -> {y}: {}", if fwd { "recognized" } else { "—" });
            println!("{y} -> {x}: {}", if rev { "recognized" } else { "—" });
            println!("mutual: {}", if mutual { "YES" } else { "no" });
            ExitCode::SUCCESS
        }
        "accredit" => {
            let (Some(program), Some(threshold), Some(signers)) = (a.first(), flag(a, "--threshold"), flag(a, "--signers")) else {
                eprintln!("usage: aion-edu federate accredit <program> --threshold K --signers a,b,c");
                return ExitCode::FAILURE;
            };
            let Ok(k) = threshold.parse::<u32>() else {
                eprintln!("threshold must be a number");
                return ExitCode::FAILURE;
            };
            let signers: Vec<String> = signers.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            match aion_edu_provenance::set_joint_accreditation(dir, program, k, &signers) {
                Ok(()) => {
                    println!("joint program {program}: {k}-of-{} institutions={signers:?}", signers.len());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("accredit failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "endorse" => {
            let (Some(program), Some(inst)) = (a.first(), flag(a, "--by")) else {
                eprintln!("usage: aion-edu federate endorse <program> --by <institution>");
                return ExitCode::FAILURE;
            };
            if let Err(e) = aion_edu_provenance::endorse_program(dir, program, inst) {
                eprintln!("endorse failed: {e}");
                return ExitCode::FAILURE;
            }
            match aion_edu_provenance::verify_joint_accreditation(dir, program) {
                Ok(Some(j)) => {
                    println!(
                        "{inst} co-signed {program}. quorum {}/{} ({}) — institutions: {:?}",
                        j.valid_count,
                        j.threshold,
                        if j.met { "MET" } else { "not yet" },
                        j.signing_institutions
                    );
                    ExitCode::SUCCESS
                }
                _ => ExitCode::SUCCESS,
            }
        }
        "verify" => {
            let Some(program) = a.first() else {
                eprintln!("usage: aion-edu federate verify <program>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::verify_joint_accreditation(dir, program) {
                Ok(Some(j)) => {
                    println!(
                        "{program}: quorum {}/{} ({}) — co-accredited by {:?}",
                        j.valid_count,
                        j.threshold,
                        if j.met { "MET" } else { "NOT MET" },
                        j.signing_institutions
                    );
                    if j.met { ExitCode::SUCCESS } else { ExitCode::FAILURE }
                }
                Ok(None) => {
                    eprintln!("{program}: no joint accreditation policy (run `federate accredit`)");
                    ExitCode::FAILURE
                }
                Err(e) => {
                    eprintln!("verify failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "delegate" => {
            let (Some(professor), Some(inst)) = (a.first(), flag(a, "--by")) else {
                eprintln!("usage: aion-edu federate delegate <professor> --by <institution> [--until <epoch>]");
                return ExitCode::FAILURE;
            };
            let until = flag(a, "--until").and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            match aion_edu_provenance::delegate_scoped(dir, inst, professor, until) {
                Ok(()) => {
                    let scope = if until == 0 { "open-ended".to_string() } else { format!("until epoch {until}") };
                    println!("{inst} delegated faculty key for {professor} ({scope}; covers all its credentials)");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("delegate failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "revoke-delegation" => {
            let (Some(professor), Some(inst)) = (a.first(), flag(a, "--by")) else {
                eprintln!("usage: aion-edu federate revoke-delegation <professor> --by <institution>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::revoke_delegation(dir, inst, professor) {
                Ok(at) => {
                    println!("{inst} revoked {professor}'s delegation, effective epoch {at}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("revoke failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "issue" => {
            let (Some(learner), Some(lesson), Some(inst)) = (a.first(), a.get(1), flag(a, "--by")) else {
                eprintln!("usage: aion-edu federate issue <learner> <lesson> --by <institution>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::bind_issuer(dir, inst, learner, lesson) {
                Ok(()) => {
                    println!("{inst} vouched for (counter-signed) {learner}'s {lesson} credential");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("issue failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "present" => {
            let (Some(learner), Some(lesson), Some(issuer), Some(verifier)) =
                (a.first(), a.get(1), flag(a, "--issuer"), flag(a, "--to"))
            else {
                eprintln!("usage: aion-edu federate present <learner> <lesson> --issuer <i> --to <v>");
                return ExitCode::FAILURE;
            };
            let result = match flag(a, "--as-of").and_then(|s| s.parse::<u64>().ok()) {
                Some(e) => aion_edu_provenance::verify_issued_credential_at(dir, verifier, issuer, learner, lesson, e),
                None => aion_edu_provenance::verify_issued_credential(dir, verifier, issuer, learner, lesson),
            };
            match result {
                Ok(Some(c)) => {
                    let asof = flag(a, "--as-of").map(|e| format!(" (as of epoch {e})")).unwrap_or_default();
                    println!("{verifier} verifies {learner}'s {lesson} credential issued by {issuer}{asof}:");
                    println!("  credential valid (professor signature):  {}", c.credential_valid);
                    println!("  faculty delegated (root vouched once):   {}", c.faculty_delegated);
                    println!("  issuer vouched (per-credential countersig): {}", c.issuer_vouched);
                    println!("  issuer recognized by {verifier}:           {}", c.issuer_recognized);
                    println!("  => {}", if c.accepted { "ACCEPTED" } else { "REJECTED" });
                    if c.accepted { ExitCode::SUCCESS } else { ExitCode::FAILURE }
                }
                Ok(None) => {
                    eprintln!("no credential for {learner}/{lesson} (teach it first)");
                    ExitCode::FAILURE
                }
                Err(e) => {
                    eprintln!("present failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "snapshot" => {
            let Some(inst) = flag(a, "--by") else {
                eprintln!("usage: aion-edu federate snapshot --by <institution>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::take_snapshot(dir, inst) {
                Ok(id) => match aion_edu_provenance::verify_snapshot(dir, id) {
                    Ok(Some(v)) => {
                        let h: String = v.state_hash.iter().take(8).map(|b| format!("{b:02x}")).collect();
                        println!("snapshot #{id} @ epoch {} signed by {:?}", v.epoch, v.valid_signers);
                        println!("  state digest: {h}…  (matches current: {})", v.matches_current);
                        ExitCode::SUCCESS
                    }
                    _ => ExitCode::SUCCESS,
                },
                Err(e) => {
                    eprintln!("snapshot failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "snapshot-verify" => {
            let Some(id) = a.first().and_then(|s| s.parse::<u64>().ok()) else {
                eprintln!("usage: aion-edu federate snapshot-verify <id>");
                return ExitCode::FAILURE;
            };
            match aion_edu_provenance::verify_snapshot(dir, id) {
                Ok(Some(v)) => {
                    let h: String = v.state_hash.iter().take(8).map(|b| format!("{b:02x}")).collect();
                    println!("snapshot #{} @ epoch {}", v.id, v.epoch);
                    println!("  checkpointed digest: {h}…");
                    println!("  matches current state: {}", v.matches_current);
                    println!("  valid signers:   {:?}", v.valid_signers);
                    if !v.invalid_signers.is_empty() {
                        println!("  INVALID signers: {:?}", v.invalid_signers);
                    }
                    ExitCode::SUCCESS
                }
                Ok(None) => {
                    eprintln!("no snapshot #{id}");
                    ExitCode::FAILURE
                }
                Err(e) => {
                    eprintln!("snapshot-verify failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            federate_usage();
            ExitCode::SUCCESS
        }
    }
}

fn cmd_serve(rest: &[String]) -> ExitCode {
    let port = flag(rest, "--port").and_then(|p| p.parse::<u16>().ok()).unwrap_or(8080);
    println!("aion-edu  ->  http://127.0.0.1:{port}");
    match aion_edu_web::serve("127.0.0.1", port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("serve failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn flag<'a>(rest: &'a [String], name: &str) -> Option<&'a str> {
    rest.iter().position(|a| a == name).and_then(|i| rest.get(i + 1)).map(String::as_str)
}

fn cmd_govern(rest: &[String]) -> ExitCode {
    let (Some(lesson), Some(threshold), Some(signers)) =
        (rest.first(), flag(rest, "--threshold"), flag(rest, "--signers"))
    else {
        eprintln!("usage: aion-edu govern <lesson> --threshold K --signers a,b,c");
        return ExitCode::FAILURE;
    };
    let Ok(k) = threshold.parse::<u32>() else {
        eprintln!("threshold must be a number");
        return ExitCode::FAILURE;
    };
    let signers: Vec<String> = signers.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    match aion_edu_provenance::set_governance(Path::new(DATA_DIR), lesson, k, &signers) {
        Ok(()) => {
            println!("governance for {lesson}: {k}-of-{} signers={signers:?}", signers.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("govern failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_endorse(rest: &[String]) -> ExitCode {
    let (Some(lesson_id), Some(prof)) = (rest.first(), flag(rest, "--by")) else {
        eprintln!("usage: aion-edu endorse <lesson> --by <prof>");
        return ExitCode::FAILURE;
    };
    let Some((course, lesson)) = registry::find_lesson(lesson_id) else {
        eprintln!("unknown lesson: {lesson_id}");
        return ExitCode::FAILURE;
    };
    if let Err(e) = aion_edu_provenance::endorse(Path::new(DATA_DIR), &course, &lesson, prof) {
        eprintln!("endorse failed: {e}");
        return ExitCode::FAILURE;
    }
    match aion_edu_provenance::verify_quorum(Path::new(DATA_DIR), &course, &lesson) {
        Ok(Some(q)) => {
            println!("endorsed by {prof}. quorum: {}/{} ({})", q.valid_count, q.threshold, if q.met { "MET" } else { "not yet" });
            ExitCode::SUCCESS
        }
        _ => {
            println!("endorsed by {prof} (lesson ungoverned — no quorum required)");
            ExitCode::SUCCESS
        }
    }
}

fn cmd_teach(rest: &[String]) -> ExitCode {
    let Some(lesson_id) = rest.first() else {
        eprintln!("usage: aion-edu teach <lesson> --learner <name>");
        return ExitCode::FAILURE;
    };
    let learner = rest
        .iter()
        .position(|a| a == "--learner")
        .and_then(|i| rest.get(i + 1))
        .map(String::as_str)
        .unwrap_or("learner");
    match aion_edu_teach::run_lesson_simulated(Path::new(DATA_DIR), lesson_id, learner, &aion_edu_teach::print_event) {
        Ok(o) => {
            println!(
                "\n[ mastered={}  credential_file_id={:?}  binding_verified={} ]",
                o.mastered,
                o.credential_file_id,
                o.binding.map(|b| b.credential_valid && b.lineage_match).unwrap_or(false)
            );
            if o.mastered { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Err(e) => {
            eprintln!("teach failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_faculty() {
    println!("Faculty ({} registered):", registry::professors().len());
    for p in registry::professors() {
        println!("  {:10} {}  —  {}", p.id(), p.name(), p.department());
    }
}

fn cmd_catalog() {
    for course in registry::courses() {
        println!("{}  {}  (prof: {})", course.id, course.title, course.professor);
        for unit in &course.units {
            println!("  · {}  {}", unit.id, unit.title);
            for l in &unit.lessons {
                println!("      {}  {}", l.id, l.title);
            }
        }
    }
}

fn cmd_plan(rest: &[String]) -> ExitCode {
    let Some(target) = rest.first() else {
        eprintln!("usage: aion-edu plan <target>");
        return ExitCode::FAILURE;
    };
    let p = planner::plan(target, &BTreeSet::new());
    if p.resolved.is_empty() {
        println!("no catalog match for '{target}'");
        return ExitCode::FAILURE;
    }
    println!("target: {}  ->  resolved: {:?}", p.target, p.resolved);
    if !p.external_prereqs.is_empty() {
        println!("  assumes prerequisites: {:?}", p.external_prereqs);
    }
    println!("  path (in order): {}", p.path.join(" -> "));
    ExitCode::SUCCESS
}

fn cmd_seal() -> ExitCode {
    let dir = Path::new(DATA_DIR);
    let mut n = 0;
    for course in registry::courses() {
        for lesson in course.all_lessons() {
            match aion_edu_provenance::seal_rubric(dir, &course, lesson) {
                Ok(s) => {
                    println!("  sealed {}  file_id={} v{}  valid={}", lesson.id, s.file_id, s.version, s.valid);
                    n += 1;
                }
                Err(e) => {
                    eprintln!("  FAILED {}: {e}", lesson.id);
                    return ExitCode::FAILURE;
                }
            }
        }
    }
    println!("sealed {n} rubrics under {DATA_DIR}/");
    ExitCode::SUCCESS
}

fn cmd_verify(rest: &[String]) -> ExitCode {
    let Some(lesson_id) = rest.first() else {
        eprintln!("usage: aion-edu verify <lesson>");
        return ExitCode::FAILURE;
    };
    let Some((course, _)) = registry::find_lesson(lesson_id) else {
        eprintln!("unknown lesson: {lesson_id}");
        return ExitCode::FAILURE;
    };
    match aion_edu_provenance::verify_rubric(Path::new(DATA_DIR), &course.professor, lesson_id) {
        Ok(Some(s)) => {
            println!("{lesson_id}: valid={} file_id={} v{}", s.valid, s.file_id, s.version);
            if s.valid { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Ok(None) => {
            eprintln!("{lesson_id}: not sealed (run `aion-edu seal`)");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("{lesson_id}: {e}");
            ExitCode::FAILURE
        }
    }
}
