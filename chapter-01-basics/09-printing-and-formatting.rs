// Chapter 1, Lesson 9: Comments, printing and formatting
// Run: cargo run --example ch01-09

/// A doc comment (three slashes) describes the item below it.
/// Tools turn these into web documentation (see Chapter 7).
fn main() {
    // A line comment explains a line.
    /* A block comment can
       span several lines. */

    let item = "Notebook";
    let qty = 3;
    let price = 49.5;

    // Named values inside braces
    println!("{qty} x {item} at {price}");

    // Positional and named arguments
    println!("{0} costs {1}. Yes, {0} costs {1}.", item, price);
    println!("{name} has {n} items", name = "Riya", n = qty);

    // Width and alignment: < left, > right, ^ centre
    println!("|{:<10}|{:>6}|{:^8}|", "Item", "Qty", "Price");
    println!("|{:<10}|{:>6}|{:^8.2}|", item, qty, price);
    println!("|{:<10}|{:>6}|{:^8.2}|", "Pen", 10, 12.0);

    // Padding numbers with zeros, and other bases
    println!("Order #{:05}", 42);
    println!("255 in binary {:b}, hex {:x}, octal {:o}", 255, 255, 255);

    // Debug printing with {:?} and pretty debug with {:#?}
    let point = (3, 4);
    println!("Debug: {point:?}");
    println!("Pretty: {point:#?}");

    // Escape braces by doubling them
    println!("Use {{}} to print a value");
}
