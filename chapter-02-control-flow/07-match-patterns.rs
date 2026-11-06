// Chapter 2, Lesson 7: richer match patterns
// Run: cargo run --example ch02-07

fn main() {
    // Ranges in patterns
    for age in [3, 15, 30, 70] {
        let ticket = match age {
            0..=4 => "free",
            5..=17 => "child",
            18..=59 => "adult",
            _ => "senior",
        };
        println!("Age {age}: {ticket} ticket");
    }

    // Match guards: an extra `if` condition on an arm
    for amount in [-50, 0, 300, 25_000] {
        let note = match amount {
            n if n < 0 => "refund",
            0 => "nothing to pay",
            n if n > 10_000 => "needs manager approval",
            _ => "approved",
        };
        println!("Payment of {amount}: {note}");
    }

    // Matching tuples and binding parts to names
    let points = [(0, 0), (3, 0), (0, -2), (4, 5)];
    for point in points {
        match point {
            (0, 0) => println!("{point:?} is the origin"),
            (x, 0) => println!("{point:?} is on the x axis at {x}"),
            (0, y) => println!("{point:?} is on the y axis at {y}"),
            (x, y) => println!("{point:?} is somewhere else ({x}, {y})"),
        }
    }

    // @ binds a value while also testing it against a range
    let score = 87;
    match score {
        s @ 90..=100 => println!("{s}: excellent"),
        s @ 75..=89 => println!("{s}: very good"),
        s => println!("{s}: keep going"),
    }
}
