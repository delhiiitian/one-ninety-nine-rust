// Chapter 9, Lesson 6: JSON with serde
// Run: cargo run --example ch09-06

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// JSON is the common language apps use to send data, like a standard
// shipping form every courier understands. serde fills the form for us.
#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    customer: String,
    items: Vec<Item>,
    #[serde(default)]
    paid: bool, // missing in the JSON? use false
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    price: u32,
    qty: u32,
}

fn main() -> Result<(), serde_json::Error> {
    // Text -> struct (deserialize)
    let text = r#"{
        "id": 42,
        "customer": "Meera",
        "items": [
            {"name": "notebook", "price": 60, "qty": 3},
            {"name": "pen", "price": 15, "qty": 4}
        ]
    }"#;
    let order: Order = serde_json::from_str(text)?;
    let total: u32 = order.items.iter().map(|i| i.price * i.qty).sum();
    println!("Order {} for {}: total Rs {total}, paid: {}", order.id, order.customer, order.paid);

    // Struct -> text (serialize)
    let mut done = order;
    done.paid = true;
    println!("{}", serde_json::to_string_pretty(&done)?);

    // Bad JSON gives an error, not a crash.
    match serde_json::from_str::<Order>(r#"{"id": "forty-two"}"#) {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("Could not read order: {e}"),
    }

    // When the shape is unknown, use serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(r#"{"city":"Pune","temp":31.5}"#)?;
    println!("{} is {} degrees", v["city"].as_str().unwrap_or("?"), v["temp"]);

    let mut stock = HashMap::new();
    stock.insert("chai", 40);
    println!("Map as JSON: {}", serde_json::to_string(&stock)?);
    Ok(())
}
