// Chapter 2, Lesson 3: loop, break and continue
// Run: cargo run --example ch02-03

fn main() {
    // `loop` repeats forever until you `break`.
    let mut countdown = 3;
    loop {
        if countdown == 0 {
            println!("Lift off!");
            break;
        }
        println!("{countdown}...");
        countdown -= 1;
    }

    // `break value` makes the whole loop produce a value.
    // Simulate retrying a flaky network request.
    let mut attempt = 0;
    let result = loop {
        attempt += 1;
        let success = attempt == 3; // pretend the 3rd try works
        println!("Attempt {attempt}: {}", if success { "ok" } else { "failed" });
        if success {
            break attempt;
        }
    };
    println!("Connected after {result} attempts");

    // `continue` skips the rest of this round and starts the next one.
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        if n % 2 == 0 {
            continue; // skip even numbers
        }
        print!("{n} ");
    }
    println!();
}
