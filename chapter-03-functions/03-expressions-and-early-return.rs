// Chapter 3, Lesson 3: Statements, expressions and early return
// Run: cargo run --example ch03-03

fn main() {
    // A block { } is an expression: its value is its last line.
    let area = {
        let width = 4;
        let height = 5;
        width * height // no semicolon: this is the value of the block
    };
    println!("Area: {area}");

    println!("Fee for age 10: {}", ticket_fee(10));
    println!("Fee for age 30: {}", ticket_fee(30));
    println!("Fee for age 70: {}", ticket_fee(70));

    println!("Discount on Rs 300: {}", discount(300));
    println!("Discount on Rs 3000: {}", discount(3000));
}

// The whole function body is one `if` expression.
fn ticket_fee(age: u32) -> u32 {
    if age < 12 {
        100
    } else if age >= 60 {
        120
    } else {
        200
    }
}

// `return` leaves early. Handy for "guard" checks at the top.
fn discount(amount: u32) -> u32 {
    if amount < 500 {
        return 0; // too small for a discount, stop here
    }
    let percent = if amount >= 2000 { 10 } else { 5 };
    amount * percent / 100
}

// A common mistake: adding a semicolon to the last line.
// fn broken(a: i32) -> i32 {
//     a + 1;   // error[E0308]: mismatched types, expected i32, found ()
// }
