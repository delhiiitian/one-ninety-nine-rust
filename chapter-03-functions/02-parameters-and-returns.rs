// Chapter 3, Lesson 2: Parameters and return values
// Run: cargo run --example ch03-02

fn main() {
    greet("Asha");
    greet("Vikram");

    let total = add(250, 120);
    println!("Total: {total}");

    let price_with_gst = with_tax(1000.0, 18.0);
    println!("Price with 18% GST: {price_with_gst:.2}");

    let (min, max) = min_max([7, 2, 9, 4]);
    println!("Smallest {min}, largest {max}");

    println!("Is 2024 a leap year? {}", is_leap_year(2024));
    println!("Is 1900 a leap year? {}", is_leap_year(1900));
}

// Each parameter needs a name and a type.
fn greet(name: &str) {
    println!("Namaste, {name}!");
}

// `-> Type` declares what the function gives back.
// The last expression (no semicolon) is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn with_tax(price: f64, percent: f64) -> f64 {
    price * (1.0 + percent / 100.0)
}

// Return several values at once with a tuple.
fn min_max(numbers: [i32; 4]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for n in numbers {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
    }
    (min, max)
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
