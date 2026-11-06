# Project 8: Line editor with undo and redo

A small text editor that works one line at a time, like the classic `ed`,
with unlimited undo and redo, a clipboard, search and saving. Built in
Chapter 8 of *Rust in 199 Pages* to show ownership and borrowing in a real
design: edits are values that MOVE between the undo and redo stacks.

```bash
cargo run -p text-editor -- notes.txt
edit> a Shopping list
edit> a rice
edit> r 2 brown rice
edit> u
Undid: replace line 2 with "brown rice"
edit> redo
edit> w
cargo test -p text-editor
```

## Layout

```text
08-text-editor/
├── src/
│   ├── main.rs      the command loop and Editor state
│   ├── buffer.rs    Buffer: owns the lines; borrow-based find()
│   ├── edit.rs      Edit enum: insert, delete, replace, and inverse()
│   └── history.rs   undo/redo stacks; edits move between them; tests
└── tests/
    └── session.rs   types a whole editing session into the real program
```

## Ideas to extend

- Group several edits into one undo step (e.g. a `paste 3 lines` command).
- Character-level editing within a line.
- Keep the history when saving, so undo survives a restart (serde, Chapter 7).
