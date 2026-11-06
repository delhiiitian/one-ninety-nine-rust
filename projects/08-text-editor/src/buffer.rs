// The text being edited: a list of lines the Buffer OWNS.
// Methods borrow it (&self to read, &mut self to change) and hand
// removed text back to the caller by value, so nothing is ever copied
// or lost behind the scenes.

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { lines: Vec::new() }
    }

    pub fn from_text(text: &str) -> Buffer {
        Buffer { lines: text.lines().map(String::from).collect() }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Borrow one line. The &str lives as long as the borrow of the buffer.
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    /// Takes ownership of `text` and stores it at `index`.
    pub fn insert(&mut self, index: usize, text: String) {
        self.lines.insert(index, text);
    }

    /// Removes a line and gives ownership of it back to the caller.
    pub fn remove(&mut self, index: usize) -> String {
        self.lines.remove(index)
    }

    /// Swaps in new text and returns the old text, without cloning either.
    pub fn replace(&mut self, index: usize, text: String) -> String {
        std::mem::replace(&mut self.lines[index], text)
    }

    /// Every line containing `word`, as (line number, borrowed line).
    /// The returned slices point INTO the buffer, so the buffer cannot be
    /// changed while the results are still in use: the borrow checker
    /// guarantees the results never point at stale text.
    pub fn find<'a>(&'a self, word: &str) -> Vec<(usize, &'a str)> {
        self.lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains(word))
            .map(|(i, line)| (i, line.as_str()))
            .collect()
    }

    pub fn text(&self) -> String {
        let mut out = self.lines.join("\n");
        if !out.is_empty() {
            out.push('\n');
        }
        out
    }

    pub fn word_count(&self) -> usize {
        self.lines.iter().map(|l| l.split_whitespace().count()).sum()
    }
}
