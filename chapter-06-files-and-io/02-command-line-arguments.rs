// Chapter 6, Lesson 2: Command-line arguments
// Run: cargo run --example ch06-02 -- --name Asha --times 3 loud

use std::env;

fn main() {
    // args() yields the program path first, then each argument.
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);

    // A tiny hand-written parser for --flag value pairs and plain words.
    let mut name = String::from("world");
    let mut times = 1;
    let mut loud = false;
    let mut rest = args.iter().skip(1);
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--name" => name = rest.next().cloned().unwrap_or(name),
            "--times" => {
                times = rest.next().and_then(|t| t.parse().ok()).unwrap_or(1);
            }
            "loud" => loud = true,
            "--help" => {
                println!("usage: ch06-02 [--name NAME] [--times N] [loud]");
                return;
            }
            other => println!("(ignoring unknown argument '{other}')"),
        }
    }

    for _ in 0..times {
        let greeting = format!("Hello, {name}!");
        if loud {
            println!("{}", greeting.to_uppercase());
        } else {
            println!("{greeting}");
        }
    }
    // Real tools use the `clap` crate for this; it writes the help text,
    // checks types and reports mistakes for you.
}
