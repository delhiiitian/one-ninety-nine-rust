// Chapter 1, Lesson 5: Booleans and characters
// Run: cargo run --example ch01-05

fn main() {
    // A bool is either true or false. It answers yes/no questions.
    let is_raining = true;
    let has_umbrella = false;

    // Combine bools with && (and), || (or) and ! (not).
    let will_get_wet = is_raining && !has_umbrella;
    println!("Will I get wet? {will_get_wet}");

    // Comparisons produce bools.
    let age = 17;
    let can_vote = age >= 18;
    println!("Can vote at {age}? {can_vote}");

    // A char is a single Unicode character, written in single quotes.
    // It takes 4 bytes, so it can hold any letter, symbol or emoji.
    let grade = 'A';
    let rupee = '₹';
    let hindi = 'अ';
    println!("Grade {grade}, currency {rupee}, letter {hindi}");

    // chars have helpful methods.
    println!("Is '{grade}' a letter? {}", grade.is_alphabetic());
    println!("Is '7' a digit? {}", '7'.is_ascii_digit());
    println!("Lowercase of '{grade}': {}", grade.to_ascii_lowercase());

    // A char can be turned into its number (code point) and back.
    println!("'{grade}' is code point {}", grade as u32);
}
