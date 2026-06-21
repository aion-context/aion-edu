//! aion-edu-faculty — the concrete professors.
//!
//! Each professor is one module that ends in `inventory::submit!`. To add a
//! professor: create `src/<name>.rs`, implement [`aion_edu_core::Professor`],
//! submit it, and add a `mod` line here. The kernel discovers it automatically.

#![forbid(unsafe_code)]

mod boole;
mod brewer;
mod darwin;
mod dean;
mod dijkstra;
mod euler;
mod feynman;
mod gauss;
mod hamming;
mod herlihy;
mod kleppmann;
mod knuth;
mod lamport;
mod liskov;
mod lynch;
mod noether;
mod polya;
mod sagan;
mod shannon;
mod strang;
mod turing;
mod vogels;
mod vonneumann;
