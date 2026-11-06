// Chapter 5, Lesson 5: The ? operator
// Run: cargo run --example ch05-05

use std::num::ParseIntError;

// Without ?: every step needs a match.
fn add_strings_long(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

// With ?: "give me the value, or return the error from this function".
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

// ? works through chains of calls: errors travel upwards automatically.
fn parse_time(text: &str) -> Result<u32, String> {
    let (h, m) = text.split_once(':').ok_or(format!("'{text}' has no ':'"))?;
    let hours: u32 = h.parse().map_err(|_| format!("bad hours '{h}'"))?;
    let minutes: u32 = m.parse().map_err(|_| format!("bad minutes '{m}'"))?;
    if hours > 23 || minutes > 59 {
        return Err(format!("'{text}' is not a real time"));
    }
    Ok(hours * 60 + minutes)
}

fn minutes_between(start: &str, end: &str) -> Result<u32, String> {
    let s = parse_time(start)?;
    let e = parse_time(end)?;
    e.checked_sub(s).ok_or("end is before start".to_string())
}

// ? also works on Option inside functions that return Option.
fn first_initial(full_name: &str) -> Option<char> {
    let first_word = full_name.split_whitespace().next()?;
    first_word.chars().next()
}

fn main() {
    println!("{:?}", add_strings_long("4", "5"));
    println!("{:?}", add_strings("4", "5"));
    println!("{:?}", add_strings("4", "five"));

    println!("Class length: {:?}", minutes_between("09:30", "11:15"));
    println!("Bad input:    {:?}", minutes_between("09:30", "11-15"));
    println!("Not a time:   {:?}", minutes_between("25:00", "26:00"));
    println!("Backwards:    {:?}", minutes_between("12:00", "08:00"));

    println!("Initial: {:?}", first_initial("Grace Hopper"));
    println!("Initial: {:?}", first_initial("   "));
}
