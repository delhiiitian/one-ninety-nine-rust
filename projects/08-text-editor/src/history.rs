// Undo and redo as two stacks. Edits MOVE between them: undo pops an edit
// off the undo stack (taking ownership), applies its inverse, and pushes
// the inverse onto the redo stack. Nothing is cloned and nothing can be in
// two places at once.

use crate::buffer::Buffer;
use crate::edit::Edit;

#[derive(Default)]
pub struct History {
    undo: Vec<Edit>,
    redo: Vec<Edit>,
}

impl History {
    pub fn new() -> History {
        History::default()
    }

    /// Applies a brand-new edit and remembers it. A new edit makes the
    /// redo stack meaningless, so it is cleared (dropping those edits).
    pub fn perform(&mut self, edit: Edit, buffer: &mut Buffer) {
        edit.apply(buffer);
        self.undo.push(edit);
        self.redo.clear();
    }

    /// Returns a description of what was undone, or None if nothing to undo.
    pub fn undo(&mut self, buffer: &mut Buffer) -> Option<String> {
        let edit = self.undo.pop()?; // we now own the edit
        let description = edit.describe();
        let inverse = edit.inverse(); // ownership moves into the inverse
        inverse.apply(buffer);
        self.redo.push(inverse); // and on into the redo stack
        Some(description)
    }

    pub fn redo(&mut self, buffer: &mut Buffer) -> Option<String> {
        let inverse = self.redo.pop()?;
        let edit = inverse.inverse(); // turn it back into the original edit
        edit.apply(buffer);
        let description = edit.describe();
        self.undo.push(edit);
        Some(description)
    }

    pub fn counts(&self) -> (usize, usize) {
        (self.undo.len(), self.redo.len())
    }

    /// Borrow the undo stack to list recent edits, newest first.
    pub fn recent(&self, n: usize) -> impl Iterator<Item = &Edit> {
        self.undo.iter().rev().take(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_and_redo_restore_exact_text() {
        let mut buf = Buffer::from_text("one\ntwo");
        let mut h = History::new();
        h.perform(Edit::Insert { at: 1, text: "middle".into() }, &mut buf);
        h.perform(Edit::Replace { at: 0, old: "one".into(), new: "ONE".into() }, &mut buf);
        h.perform(Edit::Delete { at: 2, text: "two".into() }, &mut buf);
        assert_eq!(buf.text(), "ONE\nmiddle\n");

        assert!(h.undo(&mut buf).is_some());
        assert!(h.undo(&mut buf).is_some());
        assert_eq!(buf.text(), "one\nmiddle\ntwo\n");
        assert_eq!(h.counts(), (1, 2));

        assert!(h.redo(&mut buf).is_some());
        assert_eq!(buf.text(), "ONE\nmiddle\ntwo\n");

        h.undo(&mut buf);
        h.undo(&mut buf);
        assert_eq!(buf.text(), "one\ntwo\n");
        assert_eq!(h.undo(&mut buf), None); // nothing left
    }

    #[test]
    fn new_edit_clears_redo() {
        let mut buf = Buffer::new();
        let mut h = History::new();
        h.perform(Edit::Insert { at: 0, text: "a".into() }, &mut buf);
        h.undo(&mut buf);
        assert_eq!(h.counts(), (0, 1));
        h.perform(Edit::Insert { at: 0, text: "b".into() }, &mut buf);
        assert_eq!(h.counts(), (1, 0));
        assert_eq!(h.redo(&mut buf), None);
    }

    #[test]
    fn find_borrows_lines() {
        let buf = Buffer::from_text("Rust is fast\nPython is easy\nRust is safe");
        let hits = buf.find("Rust");
        assert_eq!(hits, vec![(0, "Rust is fast"), (2, "Rust is safe")]);
    }
}
