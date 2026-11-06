// Chapter 7, Lesson 9: rustfmt and Clippy
// Run: cargo run --example ch07-09
//
//   cargo fmt            reformats every file to the standard Rust style
//   cargo fmt --check    reports unformatted files without changing them (CI)
//   cargo clippy         hundreds of extra checks for likely mistakes
//
// Below, each "before" is something Clippy warns about, with the
// improved "after" version that the program actually runs.

fn main() {
    let names = vec!["Asha", "Dev", "Zoya"];

    // Clippy: needless_range_loop. Before:
    //   for i in 0..names.len() { println!("{}", names[i]); }
    // After: loop over the items directly.
    for name in &names {
        println!("Hello, {name}");
    }

    // Clippy: len_zero. Before: if names.len() == 0 { ... }
    if names.is_empty() {
        println!("nobody here");
    }

    // Clippy: manual_contains / iter_any. Before:
    //   let mut found = false; for n in &names { if *n == "Dev" { found = true; } }
    let found = names.contains(&"Dev");
    println!("Dev present? {found}");

    // Clippy: redundant_clone. Before: let copy = names.clone(); (never used again)
    let total_letters: usize = names.iter().map(|n| n.len()).sum();
    println!("Total letters: {total_letters}");

    // Clippy: bool_comparison. Before: if found == true { ... }
    if found {
        println!("Found Dev without comparing to `true`");
    }

    // You can silence one lint on purpose, with a reason for readers.
    #[allow(clippy::approx_constant)]
    let pi_ish = 3.14; // deliberately rough value for a quick estimate
    println!("Rough circle area for r=2: {}", pi_ish * 2.0 * 2.0);
}
