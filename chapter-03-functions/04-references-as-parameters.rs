// Chapter 3, Lesson 4: Passing references to functions
// Run: cargo run --example ch03-04

fn main() {
    let shopping = vec![String::from("rice"), String::from("dal"), String::from("ghee")];

    // &shopping lends the list to the function. We still own it after.
    print_list(&shopping);
    let n = count_long_items(&shopping, 3);
    println!("{n} items have more than 3 letters");

    // &mut lends it with permission to change it.
    let mut basket = shopping; // basket now owns the list
    add_item(&mut basket, "jaggery");
    print_list(&basket);

    let mut score = 10;
    double(&mut score);
    println!("Score after doubling: {score}");
}

// &[String] = "a borrowed view of some Strings". It can't change them.
fn print_list(items: &[String]) {
    for (i, item) in items.iter().enumerate() {
        println!("{}. {item}", i + 1);
    }
}

fn count_long_items(items: &[String], min_len: usize) -> usize {
    let mut count = 0;
    for item in items {
        if item.len() > min_len {
            count += 1;
        }
    }
    count
}

// &mut Vec<String> = "a borrowed list I'm allowed to change".
fn add_item(items: &mut Vec<String>, item: &str) {
    items.push(item.to_string());
}

fn double(value: &mut i32) {
    *value *= 2; // * follows the reference to the real value
}
