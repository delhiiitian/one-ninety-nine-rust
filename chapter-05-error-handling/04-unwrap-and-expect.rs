// Chapter 5, Lesson 4: unwrap, expect and safe defaults
// Run: cargo run --example ch05-04

fn main() {
    // unwrap: give me the value, or panic. Fine in quick experiments and
    // tests, risky in real programs.
    let n: i32 = "10".parse().unwrap();
    println!("unwrap: {n}");

    // expect: same, but with your own panic message. Write the message
    // as what you EXPECTED to be true.
    let port: u16 = "8080".parse().expect("PORT should be a number");
    println!("expect: {port}");

    // unwrap_or: a fallback value instead of a panic.
    let volume: u8 = "loud".parse().unwrap_or(5);
    println!("unwrap_or: {volume}");

    // unwrap_or_default: the type's default (0, "", false...).
    let count: u32 = "".parse().unwrap_or_default();
    println!("unwrap_or_default: {count}");

    // unwrap_or_else: compute the fallback only when needed.
    let retries: u32 = "x".parse().unwrap_or_else(|e| {
        println!("  (could not parse retries: {e}, using 3)");
        3
    });
    println!("unwrap_or_else: {retries}");

    // Options have the same family of methods.
    let names = ["Asha", "Dev"];
    let third = names.get(2).copied().unwrap_or("nobody");
    println!("Third name: {third}");

    // Rule of thumb:
    //   unwrap/expect -> when failure means a bug, or in tests
    //   unwrap_or*    -> when there is a sensible default
    //   match / ?     -> when the caller should decide (next lesson)
}
