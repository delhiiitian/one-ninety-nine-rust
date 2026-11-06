// Chapter 5, Lesson 3: Reacting to different kinds of error
// Run: cargo run --example ch05-03

use std::fs::File;
use std::io::ErrorKind;
use std::num::IntErrorKind;

fn main() {
    // Parse errors carry a kind you can match on.
    for text in ["42", "", "abc", "99999999999"] {
        match text.parse::<i32>() {
            Ok(n) => println!("'{text}' -> {n}"),
            Err(e) => match e.kind() {
                IntErrorKind::Empty => println!("'{text}' -> please type something"),
                IntErrorKind::InvalidDigit => println!("'{text}' -> digits only please"),
                IntErrorKind::PosOverflow => println!("'{text}' -> that number is too big"),
                _ => println!("'{text}' -> error: {e}"),
            },
        }
    }

    // File errors have kinds too: NotFound, PermissionDenied, ...
    match File::open("does-not-exist.txt") {
        Ok(_) => println!("Opened the file"),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("No settings file yet, using defaults");
        }
        Err(e) => println!("Could not open the file: {e}"),
    }

    // map_err changes the error, map changes the success value.
    let age: Result<u32, String> = "17"
        .parse::<u32>()
        .map_err(|e| format!("bad age: {e}"))
        .and_then(|a| if a >= 18 { Ok(a) } else { Err(format!("{a} is under 18")) });
    println!("Age check: {age:?}");

    // ok() turns a Result into an Option, dropping the error.
    let maybe: Option<u32> = "7".parse().ok();
    println!("As an Option: {maybe:?}");
}
