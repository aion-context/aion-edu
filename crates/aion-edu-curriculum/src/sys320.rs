//! Course sys320 — Performance at Scale (Prof. Dean).

use aion_edu_core::{Course, CourseRegistration, Lesson};

use crate::builder::{crit, lesson, practice, s, unit};

fn l1() -> Lesson {
    let o = [
        "Explain the token-bucket model: up to `capacity` tokens refilling at `rate`/sec, one per request, allowing bursts but bounding the long-run rate",
        "Implement allow(now): refill by elapsed*rate capped at capacity, then consume a token if available",
        "Show the bucket permits a burst up to capacity, refills over time, and caps refill at capacity",
    ];
    lesson(
        "sys320-u1-l1", "Rate Limiting: The Token Bucket", &o,
        "The rate limiter behind every API gateway. A bucket holds up to `capacity` tokens, refilled \
         at `rate`/sec; each request spends one. The two rates are distinct: capacity sets the burst, \
         rate sets the steady state. Time is passed in explicitly so it's deterministic. The stub \
         allows everything (no refill, no consume). Require the burst-vs-sustained distinction.",
        &[],
        practice("Implement TokenBucket.allow(now): refill min(capacity, tokens+rate*elapsed), then consume one.",
                 &["bucket.py", "test_bucket.py"], "pytest -q test_bucket.py"),
        vec![
            crit(o[0], "Explains capacity = burst, rate = sustained throughput."),
            crit(o[1], "allow refills by elapsed*rate (capped) then consumes a token; tests pass."),
            crit(o[2], "Shows burst to capacity, refill over time, and refill capped at capacity."),
        ],
    )
}

fn sys320() -> Course {
    Course {
        id: s("sys320"),
        title: s("Performance at Scale"),
        professor: s("dean"),
        prerequisites: vec![],
        units: vec![unit("sys320-u1", "Throughput & Limits", vec![l1()])],
    }
}

inventory::submit!(CourseRegistration { build: sys320 });
