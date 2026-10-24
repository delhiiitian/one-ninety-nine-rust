// Chapter 2, Lesson 8: if let, while let and let else
// Run: cargo run --example ch02-08

fn main() {
    // Many operations might not have an answer. They return Option:
    // Some(value) when there is one, None when there is not.
    let prices = [250, 120, 980];

    // if let: run code only when the pattern matches.
    if let Some(first) = prices.first() {
        println!("First price: {first}");
    }
    if let Some(p) = prices.get(10) {
        println!("Price at 10: {p}");
    } else {
        println!("No price at position 10");
    }

    // let else: unpack or leave early. Great for input checks.
    let input = "42";
    let Ok(age) = input.parse::<u32>() else {
        println!("'{input}' is not a valid age");
        return;
    };
    println!("Age next year: {}", age + 1);

    // while let: keep looping while the pattern matches.
    let mut stack = vec!["plate 1", "plate 2", "plate 3"];
    while let Some(top) = stack.pop() {
        println!("Washing {top}");
    }
    println!("All plates washed");
}
