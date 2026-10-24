// `super` means the parent module (shop); `crate` means the top (main.rs).
use super::product::Product;

pub struct Cart {
    items: Vec<(Product, u32)>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    pub fn add(&mut self, product: Product, qty: u32) {
        self.items.push((product, qty));
    }

    pub fn subtotal(&self) -> u32 {
        self.items.iter().map(|(p, q)| p.price * q).sum()
    }

    pub fn lines(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|(p, q)| format!("{q} x {:<8} Rs {}", p.name, p.price * q))
            .collect()
    }
}
