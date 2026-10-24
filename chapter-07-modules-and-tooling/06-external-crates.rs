// Chapter 7, Lesson 6: Using external crates (serde and serde_json)
// Run: cargo run --example ch07-06
//
// The repository's Cargo.toml lists these dependencies:
//   [dependencies]
//   serde = { version = "1", features = ["derive"] }
//   serde_json = "1"
// Cargo downloads them from crates.io the first time you build.
// In your own project, `cargo add serde --features derive` and
// `cargo add serde_json` write those lines for you.

use serde::{Deserialize, Serialize};

// derive(Serialize, Deserialize) writes the JSON conversion code for us.
#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    customer: String,
    items: Vec<Item>,
    paid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    qty: u32,
    price: f64,
}

fn main() -> Result<(), serde_json::Error> {
    let order = Order {
        id: 501,
        customer: "Asha".to_string(),
        items: vec![
            Item { name: "Mango box".into(), qty: 2, price: 450.0 },
            Item { name: "Jaggery".into(), qty: 1, price: 120.0 },
        ],
        paid: false,
    };

    // Rust value -> JSON text
    let json = serde_json::to_string_pretty(&order)?;
    println!("As JSON:\n{json}");

    // JSON text -> Rust value. Wrong or missing fields are reported as errors.
    let incoming = r#"{"id": 502, "customer": "Dev", "items": [{"name": "Tea", "qty": 3, "price": 90.5}], "paid": true}"#;
    let parsed: Order = serde_json::from_str(incoming)?;
    let total: f64 = parsed.items.iter().map(|i| i.qty as f64 * i.price).sum();
    println!("Order {} for {} totals Rs {total:.2}, paid: {}", parsed.id, parsed.customer, parsed.paid);

    let broken = r#"{"id": "five"}"#;
    match serde_json::from_str::<Order>(broken) {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("Bad JSON rejected: {e}"),
    }
    Ok(())
}
