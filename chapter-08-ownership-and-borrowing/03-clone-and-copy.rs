// Chapter 8, Lesson 3: Clone and Copy
// Run: cargo run --example ch08-03

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Document {
    title: String,
    pages: Vec<String>,
}

fn main() {
    // Copy types are small and live on the stack: integers, floats, bool,
    // char, and tuples/arrays/structs made only of Copy types.
    // Assigning them COPIES the bits; both variables stay usable.
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    let p1 = Point { x: 1, y: 2 };
    let mut p2 = p1; // copied, because Point derives Copy
    p2.x = 100;
    println!("p1 = {p1:?}, p2 = {p2:?} (p1.y is still {})", p1.y);

    // String, Vec and other heap types are not Copy. To get a second,
    // independent value, ask for one explicitly with .clone().
    let original = Document { title: "Draft".into(), pages: vec!["Intro".into(), "Plan".into()] };
    let mut copy = original.clone(); // deep copy: new heap memory
    copy.title.push_str(" v2");
    copy.pages.push("Budget".into());
    println!("original: {} with {} pages", original.title, original.pages.len());
    println!("copy:     {} with {} pages", copy.title, copy.pages.len());

    // Cloning has a cost, so Rust never does it silently. Seeing .clone()
    // in code tells you exactly where data is duplicated.
    let big: Vec<u64> = (0..1_000_000).collect();
    let started = std::time::Instant::now();
    let big_copy = big.clone();
    println!("Cloning 1,000,000 numbers took {:?} (len {})", started.elapsed(), big_copy.len());
}
