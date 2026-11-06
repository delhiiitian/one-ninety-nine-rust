// Chapter 8, Lesson 2: Ownership rules and moves
// Run: cargo run --example ch08-02
//
// The three rules:
//   1. Each value has exactly one owner (a variable).
//   2. There can only be one owner at a time.
//   3. When the owner goes out of scope, the value is dropped (freed).

fn main() {
    let ticket = String::from("Seat 14C, Delhi to Goa");

    // Assigning a heap value MOVES it. `ticket` is no longer valid.
    let passenger_ticket = ticket;
    println!("Passenger holds: {passenger_ticket}");
    // println!("{ticket}");  // error[E0382]: borrow of moved value: `ticket`

    // Passing to a function moves too.
    board_plane(passenger_ticket);
    // println!("{passenger_ticket}"); // error: value moved into board_plane

    // A function can hand ownership back by returning the value.
    let receipt = print_receipt(String::from("Order #771"));
    println!("Got back: {receipt}");

    // Moving out of a Vec in a loop: `for x in v` takes each item.
    let parcels = vec![String::from("books"), String::from("shoes")];
    for parcel in parcels {
        deliver(parcel); // each String moves into deliver
    }
    // println!("{parcels:?}"); // error: `parcels` was moved by the loop

    // Drop order: values are dropped in reverse order of creation.
    let _first = Noisy("first");
    let _second = Noisy("second");
    println!("End of main");
}

fn board_plane(t: String) {
    println!("Boarding with {t}");
} // `t` is dropped here; the String's memory is freed

fn print_receipt(order: String) -> String {
    println!("Printing {order}");
    order // ownership moves back to the caller
}

fn deliver(item: String) {
    println!("Delivered {item}");
}

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
