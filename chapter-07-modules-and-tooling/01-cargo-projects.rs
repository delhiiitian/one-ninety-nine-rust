// Chapter 7, Lesson 1: Cargo projects and Cargo.toml
// Run: cargo run --example ch07-01
//
// Cargo reads Cargo.toml and passes some of it to your program at compile
// time. env!("NAME") reads such a value while compiling.

fn main() {
    println!("Package: {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Built from folder: {}", env!("CARGO_MANIFEST_DIR"));

    // cfg!(debug_assertions) is true for `cargo run`, false for `--release`.
    if cfg!(debug_assertions) {
        println!("This is a DEBUG build: fast to compile, slower to run, extra checks.");
    } else {
        println!("This is a RELEASE build: optimised for speed.");
    }

    // The everyday Cargo commands (run them in a terminal):
    let commands = [
        ("cargo new hello", "create a new project in ./hello"),
        ("cargo build", "compile (debug)"),
        ("cargo run", "compile and run"),
        ("cargo build --release", "compile with optimisations"),
        ("cargo check", "type-check quickly without building"),
        ("cargo test", "run all tests"),
        ("cargo doc --open", "build and open documentation"),
        ("cargo add serde", "add a dependency to Cargo.toml"),
        ("cargo fmt", "format all code"),
        ("cargo clippy", "lint: find likely mistakes"),
    ];
    for (cmd, what) in commands {
        println!("  {cmd:<24} {what}");
    }
}
