// Chapter 8, Lesson 6: Slices, borrowed views into data
// Run: cargo run --example ch08-06

fn main() {
    // A string slice &str points into part of a String: no copying.
    let address = String::from("221B Baker Street, London");
    let house = &address[0..4];
    let street = &address[5..17];
    let city = &address[19..];
    println!("House '{house}', street '{street}', city '{city}'");

    // Functions that return slices borrow from their input.
    println!("First word: {}", first_word(&address));
    println!("Domain of asha@example.com: {:?}", domain("asha@example.com"));

    // Array and Vec slices: &[T]
    let temps = [31, 33, 35, 34, 30, 29, 28];
    let weekdays = &temps[..5];
    let weekend = &temps[5..];
    println!("Weekday average {:.1}, weekend average {:.1}", average(weekdays), average(weekend));

    // Handy slice methods
    println!("Weekdays: first {:?}, last {:?}, contains 35? {}", weekdays.first(), weekdays.last(), weekdays.contains(&35));
    for window in temps.windows(3) {
        print!("{:?} ", window); // every run of 3 consecutive days
    }
    println!();
    for chunk in temps.chunks(3) {
        print!("{:?} ", chunk);
    }
    println!();

    // Slicing text must happen on character boundaries. Byte 1 is inside
    // the two-byte 'é', so this would panic:  &"café"[0..4]
    let word = "café";
    println!("'{word}' first 3 chars: {}", word.chars().take(3).collect::<String>());
    println!("get(0..4) is safe: {:?}", word.get(0..4));
}

fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn domain(email: &str) -> Option<&str> {
    let at = email.find('@')?;
    Some(&email[at + 1..])
}

fn average(values: &[i32]) -> f64 {
    values.iter().sum::<i32>() as f64 / values.len() as f64
}
