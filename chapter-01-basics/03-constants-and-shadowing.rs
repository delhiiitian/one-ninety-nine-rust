// Chapter 1, Lesson 3: Constants and shadowing
// Run: cargo run --example ch01-03

// A constant never changes and must have its type written out.
// By convention constant names use UPPER_SNAKE_CASE.
const MAX_LOGIN_ATTEMPTS: u32 = 3;
const SPEED_OF_LIGHT_KM_S: f64 = 299_792.458; // underscores help readability

fn main() {
    println!("You may try to log in {MAX_LOGIN_ATTEMPTS} times.");
    println!("Light travels {SPEED_OF_LIGHT_KM_S} km every second.");

    // Shadowing: a new `let` with the same name creates a NEW variable
    // that hides the old one. The old value is not changed.
    let input = "42"; // text typed by a user
    let input: i32 = input.parse().expect("not a number"); // now a number
    let input = input * 2; // a new value built from the old one
    println!("Twice the input is {input}");

    // Shadowing lasts until the end of the block `{ }` it lives in.
    let level = 1;
    {
        let level = level + 10;
        println!("Inside the block, level = {level}");
    }
    println!("Outside the block, level = {level}");
}
