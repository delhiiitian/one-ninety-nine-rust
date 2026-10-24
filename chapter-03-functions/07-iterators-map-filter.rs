// Chapter 3, Lesson 7: Iterators with closures: map, filter, sum, collect
// Run: cargo run --example ch03-07

fn main() {
    let marks = [78, 45, 92, 61, 33, 88];

    // filter keeps items for which the closure returns true.
    let passed: Vec<i32> = marks.iter().copied().filter(|&m| m >= 50).collect();
    println!("Passed: {passed:?}");

    // map transforms every item.
    let percentages: Vec<String> = marks.iter().map(|m| format!("{m}%")).collect();
    println!("As text: {percentages:?}");

    // Chain steps together like an assembly line.
    let total_bonus: i32 = marks
        .iter()
        .filter(|&&m| m < 50) // only failing marks
        .map(|m| 50 - m) // how many marks short
        .sum();
    println!("Marks needed for everyone to pass: {total_bonus}");

    // Handy consumers: count, max, min, any, all, position
    println!("Count above 80: {}", marks.iter().filter(|&&m| m > 80).count());
    println!("Highest: {:?}", marks.iter().max());
    println!("Anyone below 35? {}", marks.iter().any(|&m| m < 35));
    println!("All above 30? {}", marks.iter().all(|&m| m > 30));
    println!("First 90+ at index {:?}", marks.iter().position(|&m| m >= 90));

    // A real-world style pipeline: cart totals.
    let cart = [("pen", 20, 3), ("notebook", 55, 2), ("bag", 799, 1)];
    let total: i32 = cart.iter().map(|(_, price, qty)| price * qty).sum();
    let names: Vec<&str> = cart.iter().map(|(name, _, _)| *name).collect();
    println!("Cart {names:?} costs Rs {total}");

    // Iterators are lazy: nothing happens until a consumer (sum, collect...)
    let first_three_squares: Vec<u64> = (1..).map(|n| n * n).take(3).collect();
    println!("First three squares: {first_three_squares:?}");
}
