// Chapter 7, Lesson 4: Splitting modules into files
// Run: cargo run --example ch07-04
//
// Folder layout:
//   04-modules-in-files/
//   ├── main.rs          this file
//   ├── pricing.rs       `mod pricing;`  -> pricing.rs
//   └── shop/
//       ├── mod.rs       `mod shop;`     -> shop/mod.rs
//       ├── cart.rs      `mod cart;` inside shop -> shop/cart.rs
//       └── product.rs

mod pricing;
mod shop;

use shop::cart::Cart;
use shop::product::Product;

fn main() {
    let mut cart = Cart::new();
    cart.add(Product::new("Kurta", 1299), 1);
    cart.add(Product::new("Dupatta", 499), 2);

    for line in cart.lines() {
        println!("{line}");
    }
    let subtotal = cart.subtotal();
    let discount = pricing::festival_discount(subtotal);
    println!("Subtotal Rs {subtotal}, festival discount Rs {discount}, pay Rs {}", subtotal - discount);
}
