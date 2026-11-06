// Chapter 8, Lesson 4: Borrowing with references
// Run: cargo run --example ch08-04

fn main() {
    let essay = String::from("Rust makes systems programming safe and productive");

    // &essay creates a reference: we lend the String without giving it away.
    let words = word_count(&essay);
    let longest = longest_word(&essay);
    println!("{words} words, longest is '{longest}'");
    println!("We still own the essay: {essay}");

    // Many shared (&) references may exist at the same time.
    let r1 = &essay;
    let r2 = &essay;
    println!("r1 and r2 both see {} bytes", r1.len() + r2.len() - essay.len());

    // References are always valid: Rust refuses to compile code where a
    // reference could outlive the value it points to (a "dangling" reference).
    let reference;
    {
        let short_lived = String::from("temporary");
        reference = short_lived.len(); // copying a number out is fine
        // reference_to_string = &short_lived; // would not compile: dropped below
    }
    println!("Length was {reference}");

    // Borrowing struct fields works the same way.
    let library = vec![String::from("Wings of Fire"), String::from("The Guide")];
    print_titles(&library);
    println!("Library still has {} books", library.len());
}

// &str: a borrowed view of text. Accepts &String and string literals alike.
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace().max_by_key(|w| w.len()).unwrap_or("")
}

fn print_titles(books: &[String]) {
    for b in books {
        println!("  - {b}");
    }
}
