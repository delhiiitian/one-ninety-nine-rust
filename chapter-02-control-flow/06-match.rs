// Chapter 2, Lesson 6: match
// Run: cargo run --example ch02-06

fn main() {
    // match compares a value against patterns, top to bottom.
    let day = 6;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Not a day", // _ matches anything else
    };
    println!("Day {day} is {name}");

    // Several values in one arm with |
    let kind = match day {
        1 | 2 | 3 | 4 | 5 => "weekday",
        6 | 7 => "weekend",
        _ => "unknown",
    };
    println!("It's a {kind}");

    // match on text, like a simple command menu
    let command = "stop";
    match command {
        "start" => println!("Starting the engine"),
        "stop" => println!("Stopping the engine"),
        other => println!("Unknown command: {other}"),
    }

    // match must cover every possibility. Try deleting the `_` arm above:
    // error[E0004]: non-exhaustive patterns: `i32::MIN..=0_i32` and ...
}
