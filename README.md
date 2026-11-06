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

## Chapters

| # | Folder | Topics |
|---|--------|--------|
| 1 | `chapter-01-basics` | hello world, variables, types, strings, printing |
| 2 | `chapter-02-control-flow` | if, loops, match, ranges |
| 3 | `chapter-03-functions` | parameters, closures, recursion |
| 4 | `chapter-04-data-structures` | structs, enums, Option, Vec, HashMap |
| 5 | `chapter-05-error-handling` | panic, Result, `?`, custom errors |
| 6 | `chapter-06-files-and-io` | input, files, paths, buffered I/O |
| 7 | `chapter-07-modules-and-tooling` | modules, crates, tests, docs, Cargo |
| 8 | `chapter-08-ownership-and-borrowing` | moves, borrows, slices, lifetimes, Box, Rc |
| 9 | `chapter-09-real-world` | threads, Arc/Mutex, channels, TCP, HTTP, JSON, processes, timing, C FFI |

## Projects

| # | Run it | What it is |
|---|--------|------------|
| 1 | `cargo run -p unit-converter -- 10 km mi` | unit converter |
| 2 | `cargo run -p guessing-game` | number guessing game |
| 3 | `cargo run -p calculator` | expression calculator |
| 4 | `cargo run -p library-system` | library lending system |
| 5 | `cargo run -p bank-system` | bank accounts with errors |
| 6 | `cargo run -p log-analyzer -- projects/06-log-analyzer/sample.log` | log file analyzer |
| 7 | `cargo run -p markdown-converter -- projects/07-markdown-converter/sample.md` | Markdown to HTML |
| 8 | `cargo run -p text-editor -- notes.txt` | line editor with undo and redo |
| 9 | `cargo run -p kv-server` | multi-threaded key-value web server |

Each project folder has its own README and tests: `cargo test -p <name>`.
