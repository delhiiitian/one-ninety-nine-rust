// Chapter 4, Lesson 3: Enums
// Run: cargo run --example ch04-03

// An enum is a type that is exactly ONE of several variants.
#[derive(Debug)]
enum TrafficLight {
    Red,
    Amber,
    Green,
}

// Variants can carry data, and each variant can carry different data.
#[derive(Debug)]
enum Payment {
    Cash,
    Upi(String),                              // a UPI id
    Card { last4: String, network: String },  // named fields
}

impl TrafficLight {
    fn seconds(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Amber => 5,
            TrafficLight::Green => 45,
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Amber,
            TrafficLight::Amber => TrafficLight::Red,
        }
    }
}

fn describe(payment: &Payment) -> String {
    match payment {
        Payment::Cash => "Paid in cash".to_string(),
        Payment::Upi(id) => format!("Paid by UPI from {id}"),
        Payment::Card { last4, network } => format!("Paid by {network} card ending {last4}"),
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!("{light:?} for {} seconds", light.seconds());
        light = light.next();
    }

    let payments = [
        Payment::Cash,
        Payment::Upi(String::from("riya@okbank")),
        Payment::Card { last4: String::from("4242"), network: String::from("RuPay") },
    ];
    for p in &payments {
        println!("{}", describe(p));
    }
    println!("Debug view: {:?}", payments[1]);
}
