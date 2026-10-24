pub struct Product {
    pub name: String,
    pub price: u32,
}

impl Product {
    pub fn new(name: &str, price: u32) -> Product {
        Product { name: name.to_string(), price }
    }
}
