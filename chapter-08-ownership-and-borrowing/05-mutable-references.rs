// Chapter 8, Lesson 5: Mutable references and the borrowing rules
// Run: cargo run --example ch08-05
//
// The rules, checked at compile time by the "borrow checker":
//   * EITHER any number of shared references (&T)
//   * OR exactly one mutable reference (&mut T)
//   * never both at the same time.

fn main() {
    let mut cart: Vec<String> = Vec::new();

    // One mutable borrow at a time.
    add(&mut cart, "rice");
    add(&mut cart, "dal");
    println!("Cart: {cart:?}");

    // A borrow ends after its last use, so this sequence is fine:
    let first = &cart[0]; // shared borrow starts
    println!("First item: {first}"); // ...and ends here (last use)
    cart.push(String::from("ghee")); // so a mutable use is allowed again

    // This would NOT compile, because `first` is used after the push:
    //   let first = &cart[0];
    //   cart.push(String::from("salt"));   // error[E0502]: cannot borrow `cart`
    //   println!("{first}");                // as mutable because it is also
    //                                       // borrowed as immutable
    // Why? push may move the Vec's items to a bigger memory block, which
    // would leave `first` pointing at freed memory. Rust stops the bug.

    // Mutable references to different parts at once: split the borrow.
    let mut marks = [40, 55, 70, 85];
    let (low, high) = marks.split_at_mut(2);
    low[0] += 5;
    high[1] += 5;
    println!("Marks: {marks:?}");

    // Changing through a reference with *
    let mut balance = 1000;
    apply_interest(&mut balance, 10);
    println!("Balance after interest: {balance}");

    // Iterating mutably
    for item in cart.iter_mut() {
        item.make_ascii_uppercase();
    }
    println!("Cart: {cart:?}");
}

fn add(cart: &mut Vec<String>, item: &str) {
    cart.push(item.to_string());
}

fn apply_interest(balance: &mut i32, percent: i32) {
    *balance += *balance * percent / 100;
}
