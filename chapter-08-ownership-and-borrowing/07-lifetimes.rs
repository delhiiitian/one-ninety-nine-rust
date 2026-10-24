// Chapter 8, Lesson 7: Lifetimes
// Run: cargo run --example ch08-07

// When a function returns a reference built from SEVERAL inputs, Rust needs
// to know which input it borrows from. 'a names that relationship:
// "the result lives as long as BOTH x and y are alive".
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// A struct that holds a reference needs a lifetime too: a Highlight cannot
// outlive the text it points into.
struct Highlight<'a> {
    text: &'a str,
    page: u32,
}

impl<'a> Highlight<'a> {
    fn shout(&self) -> String {
        self.text.to_uppercase()
    }
}

// Most of the time lifetimes are inferred ("elided"). With one reference
// input, the output borrows from it automatically. No 'a needed here:
fn first_sentence(text: &str) -> &str {
    text.split('.').next().unwrap_or(text)
}

fn main() {
    let a = String::from("Bengaluru");
    let result;
    {
        let b = String::from("Pune");
        result = longer(&a, &b);
        println!("Longer city name: {result}");
    }
    // Using `result` here would fail: it might point into `b`, which is gone.

    let book = String::from("It was a bright cold day in April. The clocks were striking thirteen.");
    let quote = Highlight { text: first_sentence(&book), page: 1 };
    println!("Page {}: \"{}\"", quote.page, quote.text);
    println!("Shouted: {}", quote.shout());

    // 'static: lives for the whole program. String literals are &'static str.
    let motto: &'static str = "Fearless concurrency";
    println!("{motto}");
}
