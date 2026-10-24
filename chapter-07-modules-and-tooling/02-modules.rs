// Chapter 7, Lesson 2: Organising code with modules
// Run: cargo run --example ch07-02

// `mod` creates a named box of code. Items inside are private to the box
// unless marked `pub`.
mod kitchen {
    pub fn make_chai(cups: u32) -> String {
        let water = boil_water(cups); // private helpers are fine inside
        format!("{cups} cups of chai made with {water} ml of water")
    }

    fn boil_water(cups: u32) -> u32 {
        cups * 150
    }

    // Modules can nest.
    pub mod bakery {
        pub fn bake(item: &str) -> String {
            format!("Fresh {item} from the oven")
        }
    }
}

mod billing {
    pub const GST_PERCENT: u32 = 5;

    pub fn total(prices: &[u32]) -> u32 {
        let sum: u32 = prices.iter().sum();
        sum + sum * GST_PERCENT / 100
    }
}

fn main() {
    // Reach items with a path: module::item
    println!("{}", kitchen::make_chai(2));
    println!("{}", kitchen::bakery::bake("bun"));
    println!("Bill with {}% GST: Rs {}", billing::GST_PERCENT, billing::total(&[30, 30, 20]));

    // kitchen::boil_water(1);  // error[E0603]: function `boil_water` is private
}
