// Chapter 8, Lesson 1: The stack and the heap
// Run: cargo run --example ch08-01

use std::mem::size_of;

fn main() {
    // Values with a fixed, known size live on the STACK: fast, automatic,
    // and freed the moment the function returns.
    let age: u8 = 30;
    let price: f64 = 99.5;
    let point: (i32, i32) = (3, 4);
    println!("u8 takes {} byte, f64 {} bytes, (i32, i32) {} bytes", size_of::<u8>(), size_of::<f64>(), size_of::<(i32, i32)>());
    println!("{age} {price} {point:?}");

    // Values that can grow live on the HEAP. The variable on the stack is a
    // small fixed-size handle: a pointer, a length and a capacity.
    let mut name = String::from("Asha");
    println!("A String handle is {} bytes on the stack", size_of::<String>());
    println!("'{name}': len {}, capacity {}", name.len(), name.capacity());
    name.push_str(" Verma, Delhi");
    println!("'{name}': len {}, capacity {}", name.len(), name.capacity());

    let scores: Vec<u32> = (1..=1000).collect();
    println!("A Vec handle is {} bytes; its 1000 numbers use {} bytes on the heap", size_of::<Vec<u32>>(), scores.len() * size_of::<u32>());

    // When `name` and `scores` go out of scope at the end of main, Rust
    // frees their heap memory automatically. No garbage collector, no free().
    {
        let temporary = String::from("I live only inside these braces");
        println!("{temporary}");
    } // <- `temporary` is dropped here and its heap memory is returned
    println!("End of main: name and scores are dropped next");
}
