# Project 4: Library management system

Books, members, loans, due dates and fines, managed from a small command
shell. Built in Chapter 4 of *Rust in 199 Pages*.

```bash
cargo run -p library-system
library> list
library> borrow 1 1
library> day 20
library> return 1
Returned late. Fine: Rs 30
cargo test -p library-system
```

## Layout

```text
04-library-system/src/
├── main.rs      the command shell: reads a command, calls the library, prints
├── book.rs      data types: Genre, Status, Book, Member, Display for Book
└── library.rs   Library: collections and rules (limits, due dates, fines), tests
```

## Ideas to extend

- Reservations: a `VecDeque` queue of members waiting for a book.
- Save and load the library to a file (Chapter 6).
- Show each member's total unpaid fines.
