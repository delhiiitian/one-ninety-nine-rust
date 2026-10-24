// Chapter 4, Lesson 2: Methods and impl blocks
// Run: cargo run --example ch04-02

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (no self): called with Rectangle::new(...).
    // Often used as a constructor.
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: f64) -> Self {
        // `Self` means "the type this impl is for", here Rectangle.
        Self { width: size, height: size }
    }

    // &self: the method reads the rectangle.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // &mut self: the method changes the rectangle.
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

struct BankAccount {
    owner: String,
    balance: u64, // in paise
}

impl BankAccount {
    fn open(owner: &str) -> Self {
        BankAccount { owner: owner.to_string(), balance: 0 }
    }
    fn deposit(&mut self, paise: u64) {
        self.balance += paise;
    }
    fn balance_rupees(&self) -> f64 {
        self.balance as f64 / 100.0
    }
}

fn main() {
    let mut room = Rectangle::new(4.0, 3.5);
    let table = Rectangle::square(1.2);
    println!("Room area: {} sq m", room.area());
    println!("Table fits in room? {}", room.can_hold(&table));
    room.scale(2.0);
    println!("Room after scaling: {} x {}", room.width, room.height);

    let mut account = BankAccount::open("Meera");
    account.deposit(150_000);
    account.deposit(2_550);
    println!("{} has Rs {:.2}", account.owner, account.balance_rupees());
}
