// Chapter 7, Lesson 8: Documentation comments and doc tests
// Run: cargo run --example ch07-08
// Build the docs for the whole repo and open them: cargo doc --open
//
// In a LIBRARY crate, the ```code``` examples inside /// comments are
// compiled and run by `cargo test` (doc tests). Project 7 does exactly that.

//! A `//!` comment documents the thing it is inside: here, this file.
//! Use it at the top of lib.rs or main.rs to describe the whole crate.

/// A temperature in degrees Celsius.
///
/// # Examples
///
/// ```
/// let t = Celsius(100.0);
/// assert_eq!(t.to_fahrenheit(), 212.0);
/// ```
pub struct Celsius(pub f64);

impl Celsius {
    /// Converts to Fahrenheit using `F = C x 9/5 + 32`.
    pub fn to_fahrenheit(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }

    /// Returns a description of how the temperature feels.
    ///
    /// # Panics
    ///
    /// Panics if the temperature is below absolute zero (-273.15 C),
    /// because such a value means a bug in the caller.
    pub fn feels(&self) -> &'static str {
        assert!(self.0 >= -273.15, "below absolute zero");
        match self.0 {
            t if t < 10.0 => "cold",
            t if t < 25.0 => "pleasant",
            t if t < 35.0 => "warm",
            _ => "hot",
        }
    }
}

/// Splits a bill evenly. Returns `None` when there are no people.
///
/// # Errors and edge cases
///
/// Any remainder in paise is reported separately, so no money is lost.
pub fn split_bill(total_paise: u64, people: u64) -> Option<(u64, u64)> {
    if people == 0 {
        return None;
    }
    Some((total_paise / people, total_paise % people))
}

fn main() {
    let t = Celsius(31.0);
    println!("{} C = {} F, feels {}", t.0, t.to_fahrenheit(), t.feels());
    if let Some((each, left)) = split_bill(100_000, 3) {
        println!("Each pays {each} paise, {left} paise left over");
    }
    println!("Try `cargo doc --open` to see these comments as a website.");
}
