// Chapter 2, Lesson 2: if is an expression
// Run: cargo run --example ch02-02

fn main() {
    // In Rust, `if` produces a value, so you can assign its result.
    let hour = 14;
    let greeting = if hour < 12 { "Good morning" } else { "Good afternoon" };
    println!("{greeting}!");

    // Both branches must give the same type.
    let items_in_cart = 3;
    let shipping = if items_in_cart == 0 { 0 } else { 40 };
    println!("Shipping: Rs {shipping}");

    // Longer branches: the LAST expression (no semicolon) is the value.
    let bmi = 27.4;
    let category = if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    };
    println!("BMI {bmi} is in the '{category}' range");

    // Using an if expression inside format!
    let unread = 1;
    let message = format!(
        "You have {unread} unread message{}",
        if unread == 1 { "" } else { "s" }
    );
    println!("{message}");
}
