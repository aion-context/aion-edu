//! Course math210 — Graphs and Networks (Prof. Euler).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "State Euler's condition: an Eulerian circuit needs all even degrees; an Eulerian path needs exactly 0 or 2 odd-degree vertices",
        "Implement odd_degree_count and has_eulerian_path / has_eulerian_circuit correctly",
        "Explain why the Seven Bridges of Königsberg (four odd-degree landmasses) has no walk crossing every bridge once",
    ];
    lesson(
        "math210-u1-l1", "The Seven Bridges: Eulerian Paths", &o,
        "Abstract the city to a graph: landmasses are vertices, bridges are edges. The whole \
         question reduces to counting odd-degree vertices: 0 -> a circuit, 2 -> a path, anything \
         else -> impossible. Königsberg has four odd, so no walk exists. The stub counts even \
         vertices and only checks circuits. Require the structural 'why'.",
        &[],
        practice("Implement odd_degree_count(n,edges) and has_eulerian_path / has_eulerian_circuit.",
                 &["euler.py", "test_euler.py"], "pytest -q test_euler.py"),
        vec![
            crit(o[0], "States 0 odd -> circuit, 2 odd -> path, otherwise no Eulerian walk."),
            crit(o[1], "odd_degree_count and the path/circuit checks are correct; tests pass."),
            crit(o[2], "Explains Königsberg's four odd-degree vertices rule out any single-pass walk."),
        ],
    )
}

fn math210() -> Course {
    Course {
        id: s("math210"),
        title: s("Graphs and Networks"),
        professor: s("euler"),
        prerequisites: vec![],
        units: vec![unit("math210-u1", "Walks & Degrees", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: math210 });
