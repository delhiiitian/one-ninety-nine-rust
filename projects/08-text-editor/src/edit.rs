// One change to the buffer, stored as data so it can be undone.
// Each Edit owns the text it needs: an insert remembers what was inserted,
// a delete remembers what was deleted. Undo is "apply the inverse".

use crate::buffer::Buffer;

#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    Insert { at: usize, text: String },
    Delete { at: usize, text: String },
    Replace { at: usize, old: String, new: String },
}

impl Edit {
    /// Applies the edit. Takes `&self` and clones the text into the buffer,
    /// because the history must keep its own copy for undo/redo.
    pub fn apply(&self, buffer: &mut Buffer) {
        match self {
            Edit::Insert { at, text } => buffer.insert(*at, text.clone()),
            Edit::Delete { at, .. } => {
                buffer.remove(*at);
            }
            Edit::Replace { at, new, .. } => {
                buffer.replace(*at, new.clone());
            }
        }
    }

    /// The edit that cancels this one. Consumes `self`: the inverse takes
    /// over the owned strings, so no text is copied.
    pub fn inverse(self) -> Edit {
        match self {
            Edit::Insert { at, text } => Edit::Delete { at, text },
            Edit::Delete { at, text } => Edit::Insert { at, text },
            Edit::Replace { at, old, new } => Edit::Replace { at, old: new, new: old },
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Edit::Insert { at, text } => format!("insert line {}: \"{text}\"", at + 1),
            Edit::Delete { at, text } => format!("delete line {}: \"{text}\"", at + 1),
            Edit::Replace { at, new, .. } => format!("replace line {} with \"{new}\"", at + 1),
        }
    }
}
