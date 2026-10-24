// Chapter 3, Lesson 9: Generic functions
// Run: cargo run --example ch03-09

use std::fmt::Display;

fn main() {
    // One function, many types.
    println!("Largest number: {}", largest(&[3, 9, 2]));
    println!("Largest price: {}", largest(&[19.5, 4.25, 99.0]));
    println!("Latest letter: {}", largest(&['q', 'z', 'a']));
    println!("Last word: {}", largest(&["mango", "apple", "kiwi"]));

    show_twice(42);
    show_twice("hello");

    let pair = swap(("left", 1));
    println!("Swapped: {pair:?}");

    print_all(&["red", "green"]);
    print_all(&[1.5, 2.5]);
}

// <T: PartialOrd + Copy> means: T can be any type that can be compared
// with < and > and copied. These requirements are called trait bounds.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

// impl Display is a short way to write a generic parameter.
fn show_twice(value: impl Display) {
    println!("{value} {value}");
}

// Generics with two different types A and B.
fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

// `where` moves long bounds out of the way.
fn print_all<T>(items: &[T])
where
    T: Display,
{
    let parts: Vec<String> = items.iter().map(|i| format!("<{i}>")).collect();
    println!("{}", parts.join(" "));
}
