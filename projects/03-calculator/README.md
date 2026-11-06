# Project 3: Command-line calculator

Type maths the way you write it, with precedence, brackets, powers,
functions and the previous answer. Built in Chapter 3 of *Rust in 199 Pages*.

```bash
cargo run -p calculator -- "2 * (3 + 4)"      # 14
cargo run -p calculator                       # interactive
> sqrt(16) + 2^3
= 12
> ans / 4
= 3
> history
cargo test -p calculator
```

Supports `+ - * / % ^`, brackets, unary minus, `pi`, `ans`, and the
functions `sqrt abs round sin cos` (degrees).

## Layout

```text
03-calculator/src/
├── main.rs     evaluate(), the interactive loop, number formatting, tests
├── lexer.rs    text -> tokens
└── parser.rs   tokens -> answer (recursive descent, one function per rule)
```

## Ideas to extend

- Variables: `let rate = 18` then `100 * rate / 100`.
- More functions: `log`, `max(a, b)` with two arguments.
- Build a tree of the expression first, then print it before evaluating.
