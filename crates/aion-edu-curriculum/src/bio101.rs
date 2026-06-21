//! Course bio101 — Descent with Modification (Prof. Darwin).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State the Hardy-Weinberg genotype frequencies (p², 2pq, q²) and that they sum to one",
        "Implement one generation of selection: p' = (p²·w_AA + p·q·w_Aa) / w̄",
        "Show that selection against a recessive (w_aa < the others) increases the dominant allele's frequency",
    ];
    lesson(
        "bio101-u1-l1", "Selection and Allele Frequencies", &o,
        "Selection follows from variation + heredity + differential survival. Two alleles A(p), a(q). \
         Under random mating, genotype freqs are p², 2pq, q² (the heterozygote carries a factor of 2 — \
         the stub forgets it). One generation: p' = (p²·w_AA + pq·w_Aa)/w̄. Require why selection against \
         the recessive raises p.",
        &[],
        practice("Implement hardy_weinberg(p) = (p², 2pq, q²) and next_p(p, w) for one generation of selection.",
                 &["evolution.py", "test_evolution.py"], "pytest -q test_evolution.py"),
        vec![
            crit(o[0], "hardy_weinberg returns (p², 2pq, q²) summing to 1 (the 2 on the heterozygote)."),
            crit(o[1], "next_p includes the heterozygote term; neutral fitness leaves p unchanged."),
            crit(o[2], "Shows w_aa=0 raises p, and explains why (the recessive is selected out)."),
        ],
    )
}

fn bio101() -> Course {
    Course {
        id: s("bio101"),
        title: s("Descent with Modification"),
        professor: s("darwin"),
        prerequisites: vec![],
        units: vec![unit("bio101-u1", "Population Genetics", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: bio101 });
