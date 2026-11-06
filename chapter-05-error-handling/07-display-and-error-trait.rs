// Chapter 5, Lesson 7: Display and std::error::Error for your errors
// Run: cargo run --example ch05-07

use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum OrderError {
    OutOfStock { item: String },
    InvalidQuantity(i32),
    PaymentDeclined { reason: String },
}

// Display: the message a user sees.
impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::OutOfStock { item } => write!(f, "sorry, {item} is out of stock"),
            OrderError::InvalidQuantity(q) => write!(f, "quantity {q} is not allowed"),
            OrderError::PaymentDeclined { reason } => write!(f, "payment declined: {reason}"),
        }
    }
}

// Error: marks the type as a proper error so it works with Box<dyn Error>,
// error-reporting libraries, and `?` in main. Debug + Display are required.
impl Error for OrderError {}

fn place_order(item: &str, qty: i32, stock: i32, card_ok: bool) -> Result<String, OrderError> {
    if qty <= 0 {
        return Err(OrderError::InvalidQuantity(qty));
    }
    if qty > stock {
        return Err(OrderError::OutOfStock { item: item.to_string() });
    }
    if !card_ok {
        return Err(OrderError::PaymentDeclined { reason: "card expired".into() });
    }
    Ok(format!("Order placed: {qty} x {item}"))
}

fn main() {
    let orders = [("mango box", 2, 10, true), ("mango box", -1, 10, true), ("saree", 1, 0, true), ("phone", 1, 5, false)];
    for (item, qty, stock, card) in orders {
        match place_order(item, qty, stock, card) {
            Ok(msg) => println!("OK    {msg}"),
            Err(e) => {
                println!("ERROR {e}"); // Display, for users
                println!("      {e:?}"); // Debug, for developers and logs
            }
        }
    }

    // Because OrderError implements Error, it fits in a Box<dyn Error>.
    let boxed: Box<dyn Error> = Box::new(OrderError::InvalidQuantity(0));
    println!("Boxed: {boxed}");
}
