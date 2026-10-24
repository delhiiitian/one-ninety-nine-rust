// Chapter 2, Lesson 5: for loops and ranges
// Run: cargo run --example ch02-05

fn main() {
    // a..b counts from a up to but NOT including b.
    for i in 1..4 {
        print!("{i} ");
    }
    println!("(1..4)");

    // a..=b includes b.
    for i in 1..=4 {
        print!("{i} ");
    }
    println!("(1..=4)");

    // A multiplication table.
    let n = 7;
    for i in 1..=5 {
        println!("{n} x {i} = {}", n * i);
    }

    // Loop over a collection directly.
    let fruits = ["mango", "banana", "guava"];
    for fruit in fruits {
        println!("I like {fruit}");
    }

    // enumerate() gives the position too.
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}. {fruit}", index + 1);
    }

    // rev() counts backwards, step_by() skips.
    for i in (0..=10).rev().step_by(5) {
        print!("{i} ");
    }
    println!();
}
