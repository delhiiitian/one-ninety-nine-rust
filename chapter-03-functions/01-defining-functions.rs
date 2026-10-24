// Chapter 3, Lesson 1: Defining and calling functions
// Run: cargo run --example ch03-01

fn main() {
    // Call a function by writing its name followed by brackets.
    print_banner();
    greet_customer();
    greet_customer(); // write once, use many times
    print_receipt_footer();
}

// Functions can be defined before or after main; order does not matter.
// Names use snake_case: lowercase words joined by underscores.
fn print_banner() {
    println!("==============================");
    println!("    Welcome to Chai Point     ");
    println!("==============================");
}

fn greet_customer() {
    println!("Hello! What would you like today?");
}

fn print_receipt_footer() {
    print_line(); // functions can call other functions
    println!("Thank you, visit again!");
    print_line();
}

fn print_line() {
    println!("------------------------------");
}
