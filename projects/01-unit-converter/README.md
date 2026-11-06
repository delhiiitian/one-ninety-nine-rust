# Project 1: Unit converter

A command-line tool that converts length, mass, temperature and volume.
Built in Chapter 1 of *Rust in 199 Pages*.

```bash
cargo run -p unit-converter -- 10 km mi      # 10 kilometre = 6.2137 mile
cargo run -p unit-converter -- 98.6 f c      # body temperature in Celsius
cargo run -p unit-converter -- list          # every unit it knows
cargo run -p unit-converter                  # interactive mode, type `quit` to exit
cargo test -p unit-converter                 # run the tests
```

## Layout

```text
01-unit-converter/
├── Cargo.toml
└── src/
    ├── main.rs    reads input, prints answers, tests
    └── units.rs   the table of units and the conversion formula
```

Data (`units.rs`) is kept apart from input and output (`main.rs`), so adding
a unit is one new line in the table and touches no other code.

## Ideas to extend

- Add area units (square metre, acre, hectare).
- Add a `--precision 2` option.
- Print every unit of the same kind at once: `10 km all`.
