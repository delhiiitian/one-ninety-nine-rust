// Chapter 4, Lesson 7: HashSet, BTreeMap and VecDeque
// Run: cargo run --example ch04-07

use std::collections::{BTreeMap, HashSet, VecDeque};

fn main() {
    // HashSet: a bag of unique values. Adding a duplicate does nothing.
    let mut visited: HashSet<&str> = HashSet::new();
    for city in ["Delhi", "Goa", "Delhi", "Jaipur", "Goa"] {
        let is_new = visited.insert(city);
        println!("{city}: {}", if is_new { "first visit" } else { "been here" });
    }
    println!("Unique cities: {}", visited.len());

    // Set operations: what two friends have in common
    let asha: HashSet<&str> = ["cricket", "music", "chess"].into_iter().collect();
    let dev: HashSet<&str> = ["music", "chess", "football"].into_iter().collect();
    let mut both: Vec<_> = asha.intersection(&dev).collect();
    both.sort();
    println!("Shared hobbies: {both:?}");
    let mut either: Vec<_> = asha.union(&dev).collect();
    either.sort();
    println!("All hobbies: {either:?}");

    // BTreeMap: like HashMap but keeps keys SORTED.
    let mut marks: BTreeMap<&str, u32> = BTreeMap::new();
    marks.insert("Zoya", 81);
    marks.insert("Aman", 92);
    marks.insert("Meera", 77);
    for (name, m) in &marks {
        println!("{name}: {m}"); // always in alphabetical order
    }
    // Range queries are possible because keys are sorted.
    let first_half: Vec<_> = marks.range("A".."N").map(|(n, _)| *n).collect();
    println!("Names A to M: {first_half:?}");

    // VecDeque: a queue you can push and pop at both ends.
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("customer 1");
    queue.push_back("customer 2");
    queue.push_front("VIP customer");
    while let Some(next) = queue.pop_front() {
        println!("Serving {next}");
    }
}
