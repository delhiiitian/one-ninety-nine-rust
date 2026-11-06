// Chapter 2, Lesson 4: while loops
// Run: cargo run --example ch02-04

fn main() {
    // `while` repeats as long as its condition is true.
    let mut fuel = 50.0;
    let mut km = 0;
    while fuel > 0.0 {
        fuel -= 6.5; // litres used per 100 km
        km += 100;
    }
    println!("The car drove about {km} km before running out of fuel");

    // A savings goal: how many months until we reach Rs 10,000?
    let goal = 10_000;
    let monthly = 1_200;
    let mut saved = 0;
    let mut months = 0;
    while saved < goal {
        saved += monthly;
        months += 1;
    }
    println!("Goal reached in {months} months (saved Rs {saved})");

    // Digits of a number, peeled off from the right.
    let mut number = 2026;
    let mut digit_sum = 0;
    while number > 0 {
        digit_sum += number % 10;
        number /= 10;
    }
    println!("Sum of the digits of 2026 is {digit_sum}");
}
