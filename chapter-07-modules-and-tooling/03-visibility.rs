// Chapter 7, Lesson 3: Visibility with pub
// Run: cargo run --example ch07-03

mod bank {
    // The struct is public, but its fields are private, so code outside
    // this module can only change the balance through our methods.
    pub struct Account {
        pub owner: String, // public: anyone can read and set it
        balance: u64,      // private: only code in `bank` can touch it
    }

    impl Account {
        pub fn open(owner: &str) -> Account {
            Account { owner: owner.to_string(), balance: 0 }
        }

        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
            audit::record("deposit", amount);
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }
    }

    mod audit {
        // pub(super): visible to the parent module (`bank`) only.
        pub(super) fn record(action: &str, amount: u64) {
            println!("  [audit] {action} of {amount}");
        }
    }

    // pub(crate): visible anywhere in this crate, but not to other crates.
    pub(crate) fn bank_name() -> &'static str {
        "Rust Co-operative Bank"
    }

    // A public enum makes ALL its variants public.
    pub enum Kind {
        Savings,
        Current,
    }
}

fn main() {
    println!("Welcome to {}", bank::bank_name());
    let mut acct = bank::Account::open("Meera");
    acct.deposit(500);
    acct.owner = String::from("Meera Iyer"); // allowed: pub field
    println!("{} has {}", acct.owner, acct.balance());

    // acct.balance = 1_000_000;   // error[E0616]: field `balance` is private
    // bank::Account { owner: "x".into(), balance: 9 };  // error: private field

    for kind in [bank::Kind::Savings, bank::Kind::Current] {
        match kind {
            bank::Kind::Savings => println!("Savings account"),
            bank::Kind::Current => println!("Current account"),
        }
    }
}
