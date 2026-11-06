// Chapter 9, Lesson 3: Channels
// Run: cargo run --example ch09-03

use std::sync::mpsc;
use std::thread;

fn main() {
    // A channel is like a pneumatic tube in an old bank: tellers drop
    // messages in one end, and the manager collects them at the other.
    // mpsc = "multiple producer, single consumer".
    let (tx, rx) = mpsc::channel();

    for teller in 1..=3 {
        let tx = tx.clone(); // each producer gets its own sender
        thread::spawn(move || {
            for deposit in [100, 200] {
                tx.send((teller, deposit * teller)).unwrap();
            }
        });
    }
    drop(tx); // drop the original so the channel closes when tellers finish

    // Iterating over the receiver waits for messages and stops when every
    // sender has been dropped.
    let mut total = 0;
    let mut count = 0;
    for (teller, amount) in rx {
        println!("teller {teller} sent Rs {amount}");
        total += amount;
        count += 1;
    }
    println!("{count} deposits, total Rs {total}");

    // A worker that receives jobs and sends back results.
    let (job_tx, job_rx) = mpsc::channel::<u64>();
    let (result_tx, result_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        for n in job_rx {
            result_tx.send((n, n * n)).unwrap();
        }
    });
    for n in [3, 7, 12] {
        job_tx.send(n).unwrap();
    }
    drop(job_tx);
    worker.join().unwrap();
    for (n, sq) in result_rx {
        println!("{n} squared is {sq}");
    }
}
