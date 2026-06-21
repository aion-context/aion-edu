//! Course sys301 — Designing Data-Intensive Systems (Prof. Kleppmann).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Explain why consistent hashing maps a key to the first node clockwise on the ring, so adding/removing a node remaps only ~1/N of keys",
        "Implement node_for: the node with the smallest position >= h(key), wrapping around the ring",
        "Show that removing a node moves only the keys it owned, leaving every other key in place",
    ];
    lesson(
        "sys301-u1-l1", "Consistent Hashing", &o,
        "The partitioning scheme behind Dynamo, Cassandra, and every sharded cache. Nodes and keys \
         both hash onto a ring; a key is owned by the first node clockwise. The payoff is the \
         minimal-remapping invariant: when a node leaves, only its keys move. The stub returns the \
         first node always. Require the 'why', not just a passing function.",
        &[],
        practice("Implement node_for(key, ring): the first (pos,node) with pos >= h(key), else wrap to ring[0].",
                 &["ring.py", "test_ring.py"], "pytest -q test_ring.py"),
        vec![
            crit(o[0], "Explains clockwise ownership and the ~1/N minimal-remapping property."),
            crit(o[1], "node_for returns the correct clockwise owner, wrapping the ring; tests pass."),
            crit(o[2], "Shows removing a node moves only the keys it owned."),
        ],
    )
}

fn sys301() -> Course {
    Course {
        id: s("sys301"),
        title: s("Designing Data-Intensive Systems"),
        professor: s("kleppmann"),
        prerequisites: vec![],
        units: vec![unit("sys301-u1", "Partitioning", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: sys301 });
