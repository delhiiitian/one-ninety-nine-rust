# Project 7: Markdown to HTML converter

A library (`mdconv`) plus a command-line tool that turn Markdown into HTML:
headings, paragraphs, **bold**, *italic*, `code`, links, lists, quotes, code
blocks and rules. Built in Chapter 7 of *Rust in 199 Pages* to show modules,
library + binary crates, unit tests, integration tests and doc tests.

```bash
cargo run -p markdown-converter -- projects/07-markdown-converter/sample.md
cargo run -p markdown-converter -- projects/07-markdown-converter/sample.md -o sample.html --page
cargo test -p markdown-converter      # unit, integration and doc tests
cargo doc -p markdown-converter --open
```

## Layout

```text
07-markdown-converter/
├── Cargo.toml        declares a [lib] and a [[bin]]
├── sample.md
├── src/
│   ├── lib.rs        public API: to_html, full_page, parse, Block (+ doc tests)
│   ├── block.rs      lines -> blocks (headings, lists, code...) -> HTML
│   ├── inline.rs     **bold**, *italic*, `code`, [links](url), escaping
│   └── main.rs       command-line wrapper around the library
└── tests/
    └── convert.rs    uses the library from outside, like any other crate
```

## Ideas to extend

- Images: `![alt](src.png)`.
- Nested lists by indentation.
- A table of contents built from the headings.
