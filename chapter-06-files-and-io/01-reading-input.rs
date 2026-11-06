// Chapter 6, Lesson 1: Reading what the user types (standard input)
// Run: cargo run --example ch06-01
// Or feed it input: echo "Asha 21" | cargo run --example ch06-01

use std::io::{self, BufRead, Write};

fn main() {
    // print! does not end the line, so flush to show the prompt now.
    print!("What is your name? ");
    io::stdout().flush().expect("could not flush");

    let mut name = String::new();
    // read_line appends the typed text, INCLUDING the newline, to `name`.
    // It returns how many bytes were read; 0 means input has ended.
    let bytes = io::stdin().read_line(&mut name).expect("could not read input");
    if bytes == 0 {
        println!("\n(no input given, using a guest name)");
        name = "Guest".to_string();
    }
    let name = name.trim(); // remove the newline and spaces
    println!("Hello, {name}!");

    // Read several numbers, one per line, until an empty line or end of input.
    println!("Type some expenses, one per line. Empty line to finish:");
    let mut total = 0.0;
    let mut count = 0;
    for line in io::stdin().lock().lines() {
        let line = line.expect("could not read line");
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        match line.parse::<f64>() {
            Ok(amount) => {
                total += amount;
                count += 1;
            }
            Err(_) => println!("  '{line}' is not a number, skipped"),
        }
    }
    println!("{count} expenses, total Rs {total:.2}");
}
