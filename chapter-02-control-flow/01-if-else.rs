// Chapter 2, Lesson 1: if, else if, else
// Run: cargo run --example ch02-01

fn main() {
    let temperature = 34;

    // The condition must be a bool. Rust will not treat 0 or "" as false.
    if temperature > 30 {
        println!("It's hot. Drink water!");
    }

    // else if / else choose one branch out of many, top to bottom.
    let marks = 72;
    if marks >= 90 {
        println!("Grade A");
    } else if marks >= 75 {
        println!("Grade B");
    } else if marks >= 60 {
        println!("Grade C");
    } else {
        println!("Needs improvement");
    }

    // Combine conditions with && and ||.
    let age = 25;
    let has_ticket = true;
    if age >= 18 && has_ticket {
        println!("Welcome to the concert.");
    } else {
        println!("Sorry, you can't enter.");
    }

    // A real-world rule: free delivery on orders of Rs 499 or more.
    let order_total = 450;
    if order_total >= 499 {
        println!("Free delivery!");
    } else {
        println!("Add Rs {} more for free delivery.", 499 - order_total);
    }
}
