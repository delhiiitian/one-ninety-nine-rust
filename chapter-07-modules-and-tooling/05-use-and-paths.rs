// Chapter 7, Lesson 5: use, paths, aliases and re-exports
// Run: cargo run --example ch07-05

// `use` brings a name into scope so you can write it short.
use std::collections::HashMap;
// Several names from one place, with braces.
use std::fmt::{self, Display};
// `as` renames, useful when two names clash.
use std::io::Result as IoResult;

mod geometry {
    pub mod shapes {
        pub struct Circle {
            pub r: f64,
        }
        impl Circle {
            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.r * self.r
            }
        }
    }
    pub mod units {
        pub fn cm_to_m(cm: f64) -> f64 {
            cm / 100.0
        }
    }
    // Re-export: users can write geometry::Circle instead of the long path.
    pub use self::shapes::Circle;
}

use geometry::units; // bring a module, then call units::cm_to_m
use geometry::Circle; // thanks to the re-export

struct Temperature(f64);

impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} C", self.0)
    }
}

fn save() -> IoResult<()> {
    Ok(())
}

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("tea", 40);
    println!("Tea in stock: {}", stock["tea"]);

    let c = Circle { r: units::cm_to_m(50.0) };
    println!("Area of a 50 cm circle: {:.3} sq m", c.area());

    // The full path always works, with or without `use`.
    let c2 = geometry::shapes::Circle { r: 1.0 };
    println!("Unit circle area: {:.3}", c2.area());

    println!("Today: {}", Temperature(31.25));
    println!("Saved? {:?}", save().is_ok());
}
