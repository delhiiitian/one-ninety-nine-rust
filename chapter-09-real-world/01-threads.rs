// Chapter 9, Lesson 1: Threads
// Run: cargo run --example ch09-01

use std::thread;
use std::time::Duration;

fn main() {
    // A thread is like an extra cook in a kitchen: each one works on its own
    // dish at the same time. thread::spawn starts one and returns a handle.
    let mut cooks = Vec::new();
    for (id, dish) in ["dal", "rice", "roti"].into_iter().enumerate() {
        // `move` gives the thread its own copy of `id` and `dish`.
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * (3 - id as u64)));
            format!("cook {id} finished {dish}")
        });
        cooks.push(handle);
    }

    // join() waits for a thread to finish and gives back its return value.
    // The order of finishing may differ from the order of starting.
    for handle in cooks {
        let report = handle.join().expect("a cook panicked");
        println!("{report}");
    }

    // Scoped threads can borrow local data, because Rust knows they all end
    // before the scope does.
    let orders = [120, 340, 90, 410];
    let (low, high) = orders.split_at(2);
    let (a, b) = thread::scope(|s| {
        let t1 = s.spawn(|| low.iter().sum::<i32>());
        let t2 = s.spawn(|| high.iter().sum::<i32>());
        (t1.join().unwrap(), t2.join().unwrap())
    });
    println!("First half Rs {a}, second half Rs {b}, total Rs {}", a + b);
}
