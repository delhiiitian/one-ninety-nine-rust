// Chapter 3, Lesson 8: Recursion
// Run: cargo run --example ch03-08

fn main() {
    println!("5! = {}", factorial(5));
    println!("Fibonacci(10) = {}", fibonacci(10));
    println!("Sum of digits of 98765 = {}", digit_sum(98765));
    println!("Power 2^10 = {}", power(2, 10));

    // A folder tree: each folder has a name, a size and child folders.
    let photos = Folder { name: "photos", size: 0, children: vec![
        Folder { name: "goa.jpg", size: 300, children: vec![] },
        Folder { name: "wedding", size: 0, children: vec![
            Folder { name: "a.jpg", size: 500, children: vec![] },
            Folder { name: "b.jpg", size: 450, children: vec![] },
        ]},
    ]};
    println!("Total size of '{}': {} KB", photos.name, total_size(&photos));
    print_tree(&photos, 0);
}

// A recursive function calls itself on a smaller problem.
// It always needs a base case that stops the calls.
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1 // base case
    } else {
        n * factorial(n - 1) // recursive case
    }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn digit_sum(n: u32) -> u32 {
    if n < 10 { n } else { n % 10 + digit_sum(n / 10) }
}

// Fast power: halves the problem each time (2^10 = (2^5)^2).
fn power(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        return 1;
    }
    let half = power(base, exp / 2);
    if exp % 2 == 0 { half * half } else { half * half * base }
}

struct Folder {
    name: &'static str,
    size: u32,
    children: Vec<Folder>,
}

// Trees are naturally recursive: a folder's size is its own size
// plus the size of everything inside it.
fn total_size(folder: &Folder) -> u32 {
    let mut total = folder.size;
    for child in &folder.children {
        total += total_size(child);
    }
    total
}

fn print_tree(folder: &Folder, depth: usize) {
    println!("{}{}", "  ".repeat(depth), folder.name);
    for child in &folder.children {
        print_tree(child, depth + 1);
    }
}
