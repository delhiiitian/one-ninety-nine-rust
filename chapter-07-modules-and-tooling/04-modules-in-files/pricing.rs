// This file IS the `pricing` module; no `mod pricing { }` wrapper needed.

/// 10% off orders of Rs 2000 or more.
pub fn festival_discount(subtotal: u32) -> u32 {
    if subtotal >= 2000 { subtotal / 10 } else { 0 }
}
