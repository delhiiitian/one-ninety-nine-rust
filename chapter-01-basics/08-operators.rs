// Chapter 1, Lesson 8: Operators and overflow
// Run: cargo run --example ch01-08

fn main() {
    // Arithmetic operators
    let a = 17;
    let b = 5;
    println!("{a} + {b} = {}", a + b);
    println!("{a} - {b} = {}", a - b);
    println!("{a} * {b} = {}", a * b);
    println!("{a} / {b} = {}", a / b);
    println!("{a} % {b} = {}", a % b);

    // Comparison operators: == != < > <= >=
    println!("{a} > {b}? {}", a > b);
    println!("{a} == {b}? {}", a == b);

    // Compound assignment: += -= *= /= %=
    let mut balance = 1000;
    balance -= 250;
    balance *= 2;
    println!("Balance: {balance}");

    // Overflow: a u8 can hold at most 255. Rust makes you choose
    // what happens when a result does not fit.
    let stock: u8 = 250;
    println!("checked_add(10): {:?}", stock.checked_add(10)); // None = did not fit
    println!("saturating_add(10): {}", stock.saturating_add(10)); // stops at 255
    println!("wrapping_add(10): {}", stock.wrapping_add(10)); // wraps round to 4

    // Powers and other maths live as methods.
    println!("2 to the power 10 = {}", 2_i32.pow(10));
    println!("Square root of 2 = {:.4}", 2.0_f64.sqrt());
    println!("Absolute value of -7 = {}", (-7_i32).abs());
}
