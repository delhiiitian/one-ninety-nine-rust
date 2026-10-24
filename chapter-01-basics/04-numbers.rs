// Chapter 1, Lesson 4: Integer and floating-point numbers
// Run: cargo run --example ch01-04

fn main() {
    // Integers: whole numbers. `i` = signed (can be negative), `u` = unsigned.
    // The number is how many bits of memory the value uses.
    let temperature: i32 = -5; // i32 is the default integer type
    let population: u64 = 1_400_000_000;
    let age: u8 = 30; // 0..=255, plenty for an age
    println!("Temperature {temperature} C, population {population}, age {age}");

    // The range of each type is available as a constant.
    println!("u8 holds {} to {}", u8::MIN, u8::MAX);
    println!("i32 holds {} to {}", i32::MIN, i32::MAX);

    // Floating-point numbers have a decimal part. f64 is the default.
    let price: f64 = 19.99;
    let discount = 0.15; // inferred as f64
    let final_price = price * (1.0 - discount);
    println!("Final price: {final_price:.2}"); // .2 = two decimal places

    // Integer division drops the remainder; % gives the remainder.
    let slices = 10;
    let people = 3;
    println!("Each person gets {} slices, {} left over", slices / people, slices % people);

    // Converting between number types is always explicit with `as`.
    let average = slices as f64 / people as f64;
    println!("Exact share: {average:.3}");

    // Floats are approximate. Never compare money with ==; use whole paise/cents.
    println!("0.1 + 0.2 = {}", 0.1 + 0.2);
    let price_in_paise: u64 = 1999;
    println!("Stored safely as {price_in_paise} paise");
}
