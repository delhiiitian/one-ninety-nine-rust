// Chapter 9, Lesson 8: Measuring time with Instant and Duration
// Run: cargo run --example ch09-08

use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn slow_sum(n: u64) -> u64 {
    (1..=n).sum()
}

fn fast_sum(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn main() {
    // Instant is a stopwatch: start it, do the work, read the elapsed time.
    let start = Instant::now();
    let a = slow_sum(5_000_000);
    let slow = start.elapsed();

    let start = Instant::now();
    let b = fast_sum(5_000_000);
    let fast = start.elapsed();

    assert_eq!(a, b);
    println!("Both sums are {a}");
    println!("Loop took {} microseconds, formula took {} ns", slow.as_micros(), fast.as_nanos());

    // Duration is an amount of time; it supports maths.
    let lunch = Duration::from_secs(45 * 60);
    let tea = Duration::from_secs(15 * 60);
    let breaks = lunch + tea;
    println!("Breaks: {} minutes ({breaks:?})", breaks.as_secs() / 60);

    // A simple timeout: stop waiting after a limit.
    let limit = Duration::from_millis(50);
    let begun = Instant::now();
    let mut tries = 0;
    while begun.elapsed() < limit {
        tries += 1;
        thread::sleep(Duration::from_millis(10));
    }
    println!("Gave up after {tries} tries (limit {limit:?})");

    // SystemTime is the wall clock, used for timestamps.
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("Seconds since 1970: more than 1.7 billion? {}", secs > 1_700_000_000);
}
