// Chapter 4, Lesson 6: HashMap, looking things up by key
// Run: cargo run --example ch04-06

use std::collections::HashMap;

fn main() {
    // A phone book: name -> number
    let mut phone_book: HashMap<String, String> = HashMap::new();
    phone_book.insert("Asha".to_string(), "98100 00001".to_string());
    phone_book.insert("Dev".to_string(), "98100 00002".to_string());

    // Look up a key. get returns an Option because the key may be missing.
    match phone_book.get("Asha") {
        Some(number) => println!("Asha: {number}"),
        None => println!("Asha not found"),
    }
    println!("Has Zoya? {}", phone_book.contains_key("Zoya"));

    // insert replaces the old value and returns it.
    let old = phone_book.insert("Dev".to_string(), "98100 99999".to_string());
    println!("Dev's old number was {old:?}");

    // Remove
    phone_book.remove("Asha");
    println!("Entries left: {}", phone_book.len());

    // Counting with the entry API: the classic word counter.
    let text = "the cat sat on the mat the end";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("'the' appears {} times", counts["the"]);

    // HashMaps have no order. Sort the pairs to print them neatly.
    let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (word, n) in pairs {
        println!("{word:>4}: {n}");
    }

    // Group items: city -> list of people
    let people = [("Riya", "Delhi"), ("Kabir", "Pune"), ("Sana", "Delhi")];
    let mut by_city: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, city) in people {
        by_city.entry(city).or_default().push(name);
    }
    println!("People in Delhi: {:?}", by_city["Delhi"]);
}
