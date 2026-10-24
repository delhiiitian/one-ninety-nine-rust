// Chapter 5, Lesson 1: panic! for bugs that should never happen
// Run: cargo run --example ch05-01
// Run with RUST_BACKTRACE=1 to see where a panic came from.

fn main() {
    // A panic stops the program (well, the current thread) with a message.
    // Use it for BUGS: situations that mean the code itself is wrong.

    let seats = [10, 20, 30];
    println!("Seat count at 1: {}", seats[1]);

    // Out-of-bounds indexing panics instead of reading garbage memory.
    // Uncomment to see it:
    // let i = 7;
    // println!("{}", seats[i]);
    // thread 'main' panicked at ... index out of bounds: the len is 3 but the index is 7

    // You can panic on purpose when an "impossible" state is reached.
    let percentage = 85;
    let grade = grade_for(percentage);
    println!("{percentage}% is grade {grade}");

    // assert! panics if a condition is false: a built-in sanity check.
    let total = 40 + 60;
    assert!(total == 100, "totals must add up to 100, got {total}");
    assert_eq!(seats.len(), 3);
    println!("All checks passed");

    // std::panic::catch_unwind exists, but panics are not meant for normal
    // errors like "file not found". Those use Result (next lesson).
    let result = std::panic::catch_unwind(|| grade_for(150));
    println!("Did grade_for(150) panic? {}", result.is_err());
}

fn grade_for(percentage: u32) -> char {
    match percentage {
        90..=100 => 'A',
        75..=89 => 'B',
        50..=74 => 'C',
        0..=49 => 'F',
        _ => panic!("percentage {percentage} is above 100: this is a bug in the caller"),
    }
}
