// Chapter 1, Lesson 2: Variables and mutability
// Run: cargo run --example ch01-02

fn main() {
    // A variable is a name for a value. By default it cannot change.
    let city = "Delhi";
    println!("I live in {city}.");

    // Add `mut` when the value needs to change over time.
    let mut cups_of_tea = 1;
    println!("Cups of tea so far: {cups_of_tea}");
    cups_of_tea = cups_of_tea + 1;
    cups_of_tea += 1; // shorthand for cups_of_tea = cups_of_tea + 1
    println!("Cups of tea now: {cups_of_tea}");

    // The line below would not compile, because `city` is not `mut`:
    // city = "Mumbai";
    // error[E0384]: cannot assign twice to immutable variable `city`

    // You can declare first and assign once later.
    let greeting;
    if cups_of_tea > 2 {
        greeting = "You are well caffeinated!";
    } else {
        greeting = "Time for another cup.";
    }
    println!("{greeting}");

    // A real-world style example: a running total in a shopping cart.
    let prices = [120, 45, 300];
    let mut total = 0;
    for price in prices {
        total += price;
    }
    println!("Cart total: Rs {total}");
}
