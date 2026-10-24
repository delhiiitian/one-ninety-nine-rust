// Chapter 3, Lesson 5: Closures
// Run: cargo run --example ch03-05

fn main() {
    // A closure is a small function without a name, written |args| body.
    let square = |x: i32| x * x;
    println!("5 squared is {}", square(5));

    // Types can usually be inferred.
    let add = |a, b| a + b;
    println!("2 + 3 = {}", add(2, 3));

    // Several lines need braces.
    let describe = |temp: f64| {
        let label = if temp > 30.0 { "hot" } else { "pleasant" };
        format!("{temp} C is {label}")
    };
    println!("{}", describe(34.5));

    // Closures can use variables from around them.
    let gst_rate = 0.18;
    let with_gst = |price: f64| price * (1.0 + gst_rate);
    println!("Rs 500 with GST: {:.2}", with_gst(500.0));

    // Closures are often passed to other functions.
    let mut names = vec!["Zara", "amit", "Bina", "chetan"];
    names.sort_by_key(|name| name.to_lowercase()); // sort ignoring case
    println!("Sorted: {names:?}");

    println!("Apply twice: {}", apply_twice(|n| n + 10, 1));
}

// A function that takes a closure. `impl Fn(i32) -> i32` means
// "any function-like thing that takes an i32 and returns an i32".
fn apply_twice(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    f(f(value))
}
