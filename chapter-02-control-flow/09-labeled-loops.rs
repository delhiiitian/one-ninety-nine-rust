// Chapter 2, Lesson 9: nested loops and labels
// Run: cargo run --example ch02-09

fn main() {
    // Find the first free seat in a cinema (rows x seats).
    // true = taken, false = free
    let hall = [
        [true, true, true, true],
        [true, true, false, true],
        [false, false, false, false],
    ];

    let mut found = None;
    // A label starts with an apostrophe. `break 'search` leaves BOTH loops.
    'search: for (row, seats) in hall.iter().enumerate() {
        for (seat, taken) in seats.iter().enumerate() {
            if !taken {
                found = Some((row + 1, seat + 1));
                break 'search;
            }
        }
    }
    match found {
        Some((r, s)) => println!("First free seat: row {r}, seat {s}"),
        None => println!("Sold out"),
    }

    // continue 'outer jumps to the next round of the outer loop.
    'outer: for n in 2..30 {
        for d in 2..n {
            if n % d == 0 {
                continue 'outer; // not prime, try the next n
            }
        }
        print!("{n} ");
    }
    println!("are the primes below 30");

    // A labelled loop can return a value too.
    let mut tries = 0;
    let answer = 'retry: loop {
        tries += 1;
        if tries == 4 {
            break 'retry tries * 10;
        }
    };
    println!("Answer after {tries} tries: {answer}");
}
