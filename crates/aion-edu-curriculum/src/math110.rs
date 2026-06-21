//! Course math110 — Linear Algebra: The Column Picture (Prof. Strang).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Compute Ax as a linear combination of the columns of A (the column picture), not row dot products",
        "Implement columns(A) and matvec(A,x) consistent with the column picture",
        "Decide whether Ax=b is solvable for a 2x2 system by asking whether b lies in the column space",
    ];
    lesson(
        "math110-u1-l1", "Ax as a Combination of Columns", &o,
        "Learner defaults to row dot products. Force the column picture: Ax = x0*col0 + x1*col1. \
         Solvability of Ax=b is 'is b in the column space?'. The third outcome is the one they miss.",
        &[],
        practice("Implement columns(A), matvec(A,x) (built from columns), and solvable_2x2(A,b).",
                 &["linalg.py", "test_linalg.py"], "pytest -q test_linalg.py"),
        vec![
            crit(o[0], "Explains Ax as x0*col0 + x1*col1 and builds matvec from scaled columns."),
            crit(o[1], "columns(A) returns the actual columns; test_linalg.py passes."),
            crit(o[2], "solvable_2x2 False for dependent columns with b off the line; justified via column space."),
        ],
    )
}

fn l2() -> Lesson {
    let o = [
        "Project a vector b onto the line through a using p = (a·b / a·a) a",
        "Show the error e = b - p is orthogonal to a (a·e = 0)",
        "Explain the projection as the closest point on the line (least squares in one dimension)",
    ];
    lesson(
        "math110-u1-l2", "Projection onto a Line", &o,
        "Builds on the column picture: projection is 'the part of b along a'. The key fact: the \
         error is orthogonal to a — that orthogonality IS the minimization. Check a·e = 0 in code.",
        &["math110-u1-l1"],
        practice("Implement project(a,b) = (a·b/a·a) a; confirm error is orthogonal to a.",
                 &["proj.py", "test_proj.py"], "pytest -q test_proj.py"),
        vec![
            crit(o[0], "project() implements the formula; axis and idempotence cases pass."),
            crit(o[1], "Verifies a·e = 0 and explains why orthogonality makes p closest."),
            crit(o[2], "States p minimizes ||b - p|| over the line, i.e. 1-D least squares."),
        ],
    )
}

fn l3() -> Lesson {
    let o = [
        "Set up the normal equations A^T A x = A^T b for fitting a line y = c + d x",
        "Solve the 2x2 normal equations to recover (c, d) for given points",
        "Show the residual is orthogonal to the column space (sum r = 0 and sum x*r = 0)",
    ];
    lesson(
        "math110-u1-l3", "Least Squares: Best-Fit Line", &o,
        "Generalize l2's projection from a line to a 2-D column space (columns 1 and x). Set up \
         A^T A x = A^T b and solve the 2x2 system. Residual orthogonal to BOTH columns. No black-box solver.",
        &["math110-u1-l2"],
        practice("Implement fit_line(points) -> (c,d) via the normal equations.",
                 &["lstsq.py", "test_lstsq.py"], "pytest -q test_lstsq.py"),
        vec![
            crit(o[0], "Writes the 2x2 normal equations (n, Sx, Sxx, Sy, Sxy), not a black-box solver."),
            crit(o[1], "fit_line recovers an exact line and the mean for a horizontal set; tests pass."),
            crit(o[2], "Demonstrates sum(r)=0 and sum(x*r)=0 and ties it back to projection."),
        ],
    )
}

fn u2l1() -> Lesson {
    let o = [
        "Explain an eigenvector as a direction A only scales, and the eigenvalue as that scale factor (Av = λv)",
        "Compute the eigenvalues of a 2x2 matrix from the characteristic equation λ² − (trace)λ + det = 0",
        "Verify numerically that A v = λ v for a computed eigenpair",
    ];
    lesson(
        "math110-u2-l1", "Eigenvalues: Ax = λx", &o,
        "Unit 2 builds on the column picture: most directions get rotated by A, but eigenvectors are \
         the special directions A merely scales. For a 2x2 the eigenvalues solve the characteristic \
         quadratic from trace and det. Make them verify Av = λv, not just compute roots.",
        &["math110-u1-l1"],
        practice("Implement trace, det, eigenvalues_2x2(A) via the characteristic equation, and check Av = λv.",
                 &["eig.py", "test_eig.py"], "pytest -q test_eig.py"),
        vec![
            crit(o[0], "Explains eigenvector as a scaled-only direction and eigenvalue as the factor."),
            crit(o[1], "eigenvalues_2x2 solves λ²−tλ+d=0 from trace t and det d; tests pass."),
            crit(o[2], "Verifies A v = λ v numerically for an eigenpair (e.g. λ=3, v=(1,1) of [[2,1],[1,2]])."),
        ],
    )
}

fn math110() -> Course {
    Course {
        id: s("math110"),
        title: s("Linear Algebra: The Column Picture"),
        professor: s("strang"),
        prerequisites: vec![],
        units: vec![
            unit("math110-u1", "Ax and the Column Space", vec![l1(), l2(), l3()]),
            unit("math110-u2", "Eigenvalues", vec![u2l1()]),
        ],
    }
}

inventory::submit!(CourseRegistration { build: math110 });
