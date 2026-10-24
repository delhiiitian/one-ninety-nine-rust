// Chapter 1, Lesson 6: Strings, &str and String
// Run: cargo run --example ch01-06

fn main() {
    // A string literal has type &str: a read-only view of text that is
    // baked into the program, like words printed on a sign.
    let sign: &str = "Welcome";

    // String is growable text that the program owns, like a whiteboard.
    let mut board = String::from("Today: ");
    board.push_str("Rust lesson");
    board.push('!');
    println!("{sign} | {board}");

    // Useful methods.
    let name = "  Ada Lovelace  ";
    let clean = name.trim();
    println!("Trimmed: '{clean}'");
    println!("Length in bytes: {}", clean.len());
    println!("Upper: {}", clean.to_uppercase());
    println!("Contains 'Love'? {}", clean.contains("Love"));
    println!("Replace: {}", clean.replace("Ada", "Augusta Ada"));

    // Join strings with format! (it does not print, it returns a String).
    let first = "Grace";
    let last = "Hopper";
    let full = format!("{first} {last}");
    println!("Full name: {full}");

    // Walk over words and characters.
    for word in "the quick brown fox".split_whitespace() {
        print!("[{word}] ");
    }
    println!();
    println!("'héllo' has {} chars but {} bytes", "héllo".chars().count(), "héllo".len());
}
