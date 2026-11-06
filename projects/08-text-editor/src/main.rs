// Project 8: Line editor with undo and redo
//
//   cargo run -p text-editor                 start with an empty document
//   cargo run -p text-editor -- notes.txt    open a file
// Type `help` for commands.

mod buffer;
mod edit;
mod history;

use buffer::Buffer;
use edit::Edit;
use history::History;
use std::fs;
use std::io::{self, BufRead, Write};

struct Editor {
    buffer: Buffer,
    history: History,
    clipboard: Option<String>,
    path: Option<String>,
    dirty: bool, // unsaved changes?
}

fn main() {
    let path = std::env::args().nth(1);
    let buffer = match &path {
        Some(p) => match fs::read_to_string(p) {
            Ok(text) => Buffer::from_text(&text),
            Err(_) => {
                println!("New file: {p}");
                Buffer::new()
            }
        },
        None => Buffer::new(),
    };
    let mut editor = Editor { buffer, history: History::new(), clipboard: None, path, dirty: false };
    println!("Rust line editor. {} lines loaded. Type `help`.", editor.buffer.len());

    let stdin = io::stdin();
    loop {
        print!("edit> ");
        io::stdout().flush().expect("could not flush stdout");
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).expect("could not read input") == 0 {
            break;
        }
        let line = line.trim_end_matches(['\n', '\r']);
        if line.trim() == "q" {
            if editor.dirty {
                println!("(unsaved changes were discarded)");
            }
            break;
        }
        match run(&mut editor, line) {
            Ok(message) if !message.is_empty() => println!("{message}"),
            Ok(_) => {}
            Err(e) => println!("Error: {e}"),
        }
    }
    println!("Bye.");
}

/// Line numbers are typed from 1; the buffer counts from 0.
fn line_index(text: &str, len: usize, allow_end: bool) -> Result<usize, String> {
    let n: usize = text.parse().map_err(|_| format!("'{text}' is not a line number"))?;
    let max = if allow_end { len + 1 } else { len };
    if n == 0 || n > max {
        return Err(format!("line {n} is out of range (1 to {max})"));
    }
    Ok(n - 1)
}

fn run(ed: &mut Editor, input: &str) -> Result<String, String> {
    let (command, rest) = input.trim_start().split_once(' ').unwrap_or((input.trim(), ""));
    let len = ed.buffer.len();
    let edit = match command {
        "" => return Ok(String::new()),
        "help" => return Ok(HELP.trim_end().to_string()),
        "p" => return Ok(print(&ed.buffer)),
        "a" => Edit::Insert { at: len, text: rest.to_string() },
        "i" => {
            let (n, text) = rest.split_once(' ').unwrap_or((rest, ""));
            Edit::Insert { at: line_index(n, len, true)?, text: text.to_string() }
        }
        "d" => {
            let at = line_index(rest, len, false)?;
            // The Edit must own the deleted text for undo, so copy it out
            // of the buffer before the delete happens.
            Edit::Delete { at, text: ed.buffer.line(at).unwrap_or_default().to_string() }
        }
        "r" => {
            let (n, text) = rest.split_once(' ').unwrap_or((rest, ""));
            let at = line_index(n, len, false)?;
            let old = ed.buffer.line(at).unwrap_or_default().to_string();
            Edit::Replace { at, old, new: text.to_string() }
        }
        "c" => {
            let at = line_index(rest, len, false)?;
            // clone: the clipboard needs its own String; the line stays put
            ed.clipboard = ed.buffer.line(at).map(str::to_string);
            return Ok(format!("Copied line {}", at + 1));
        }
        "v" => {
            let text = ed.clipboard.clone().ok_or("clipboard is empty")?;
            let at = if rest.is_empty() { len } else { line_index(rest, len, true)? };
            Edit::Insert { at, text }
        }
        "u" => {
            let done = ed.history.undo(&mut ed.buffer).ok_or("nothing to undo")?;
            ed.dirty = true;
            return Ok(format!("Undid: {done}"));
        }
        "redo" => {
            let done = ed.history.redo(&mut ed.buffer).ok_or("nothing to redo")?;
            ed.dirty = true;
            return Ok(format!("Redid: {done}"));
        }
        "f" => {
            if rest.is_empty() {
                return Err("usage: f <word>".into());
            }
            let hits = ed.buffer.find(rest); // borrows the buffer...
            let report = if hits.is_empty() {
                format!("'{rest}' not found")
            } else {
                hits.iter().map(|(i, l)| format!("{:>4}: {l}", i + 1)).collect::<Vec<_>>().join("\n")
            };
            return Ok(report); // ...and the borrow ends here
        }
        "log" => {
            let (u, r) = ed.history.counts();
            let mut out = format!("{u} undo steps, {r} redo steps");
            for e in ed.history.recent(5) {
                out.push_str(&format!("\n  {}", e.describe()));
            }
            return Ok(out);
        }
        "stats" => {
            return Ok(format!("{} lines, {} words, {} bytes", ed.buffer.len(), ed.buffer.word_count(), ed.buffer.text().len()));
        }
        "w" => {
            if !rest.is_empty() {
                ed.path = Some(rest.to_string());
            }
            let path = ed.path.as_deref().ok_or("usage: w <file>")?;
            fs::write(path, ed.buffer.text()).map_err(|e| format!("cannot save {path}: {e}"))?;
            ed.dirty = false;
            return Ok(format!("Saved {} lines to {path}", ed.buffer.len()));
        }
        other => return Err(format!("unknown command '{other}', type `help`")),
    };
    let description = edit.describe();
    ed.history.perform(edit, &mut ed.buffer); // the edit moves into history
    ed.dirty = true;
    Ok(format!("OK: {description}"))
}

fn print(buffer: &Buffer) -> String {
    if buffer.is_empty() {
        return "(empty)".to_string();
    }
    (0..buffer.len()).map(|i| format!("{:>4}| {}", i + 1, buffer.line(i).unwrap_or(""))).collect::<Vec<_>>().join("\n")
}

const HELP: &str = "
  p               print the document with line numbers
  a <text>        append a line at the end
  i <n> <text>    insert a line before line n
  d <n>           delete line n
  r <n> <text>    replace line n
  c <n>           copy line n to the clipboard
  v [n]           paste the clipboard (at the end, or before line n)
  u / redo        undo / redo the last change
  f <word>        find lines containing a word
  log             show recent changes
  stats           count lines, words and bytes
  w [file]        save
  q               quit
";
