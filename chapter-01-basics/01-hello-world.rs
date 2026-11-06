// Chapter 1, Lesson 1: Hello, World and Cargo
// Run: cargo run --example ch01-01

// Every Rust program starts running at the function called `main`.
fn main() {
    // `println!` prints a line of text. The `!` means it is a macro,
    // a piece of code that writes more code for you at compile time.
    println!("Hello, world!");
    println!("Welcome to Rust in 199 Pages.");

    // `print!` (without "ln") does not add a new line at the end.
    print!("Rust is ");
    print!("fast, ");
    println!("safe and friendly.");

    // `eprintln!` prints to the error stream. Tools use it for warnings
    // so that normal output can be saved to a file without the noise.
    eprintln!("(this line went to standard error)");
}
