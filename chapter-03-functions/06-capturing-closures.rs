// Chapter 3, Lesson 6: How closures capture: Fn, FnMut, FnOnce and move
// Run: cargo run --example ch03-06

fn main() {
    // Fn: only reads what it captures. Can be called any number of times.
    let shop = String::from("Chai Point");
    let welcome = || println!("Welcome to {shop}");
    welcome();
    welcome();

    // FnMut: changes what it captures. The closure must be `mut`.
    let mut visitors = 0;
    let mut count_visitor = || {
        visitors += 1;
        println!("Visitor number {visitors}");
    };
    count_visitor();
    count_visitor();
    println!("Total visitors: {visitors}");

    // FnOnce: gives away what it captures, so it can only run once.
    let receipt = String::from("Receipt #1001");
    let send = move || {
        let sent = receipt; // takes ownership of the String
        println!("Sending {sent}");
    };
    send();
    // send(); // error: closure cannot be called twice, the receipt is gone

    // `move` makes a closure own its captured values. Needed when the
    // closure outlives the place it was made, e.g. a counter factory.
    let mut next_ticket = make_counter(100);
    println!("Ticket {}", next_ticket());
    println!("Ticket {}", next_ticket());
    println!("Ticket {}", next_ticket());
}

// Returns a closure that remembers its own `count` between calls.
fn make_counter(start: u32) -> impl FnMut() -> u32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}
