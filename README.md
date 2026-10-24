# Rust in 199 Pages: the code

Every program from the book *Rust in 199 Pages* (the 199 series): 81 short
lessons in 9 chapters, plus 9 complete projects. Each lesson is one file,
and the book links straight to it.

## Quick start

1. Install Rust from <https://rustup.rs> (Rust 1.82 or newer).
2. Clone this repository and run any lesson:

```bash
git clone https://github.com/delhiiitian/one-ninety-nine-rust.git
cd one-ninety-nine-rust
cargo run --example ch01-01        # chapter 1, lesson 1
cargo run -p unit-converter -- 10 km mi
```

Run everything (all lessons, all project tests) with `./scripts/run-all.sh`.
