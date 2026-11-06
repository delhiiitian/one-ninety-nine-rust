// Chapter 9, Lesson 2: Sharing data between threads with Arc and Mutex
// Run: cargo run --example ch09-02

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc is the thread-safe Rc: several threads share one value.
    // Mutex is a lock, like the one key to a shared locker: only the
    // thread holding the key can open it.
    let ticket_counter = Arc::new(Mutex::new(0u32));
    let mut workers = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&ticket_counter);
        workers.push(thread::spawn(move || {
            for _ in 0..250 {
                let mut tickets = counter.lock().unwrap(); // take the key
                *tickets += 1;
            } // the key is returned automatically when `tickets` goes away
        }));
    }
    for w in workers {
        w.join().unwrap();
    }
    println!("Tickets sold: {}", *ticket_counter.lock().unwrap());

    // A Mutex can guard any type, such as a Vec of log lines.
    let log = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (1..=3)
        .map(|id| {
            let log = Arc::clone(&log);
            thread::spawn(move || log.lock().unwrap().push(format!("worker {id} done")))
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mut lines = log.lock().unwrap().clone();
    lines.sort();
    println!("Log: {lines:?}");
    println!("Owners of the log now: {}", Arc::strong_count(&log));
}
