// Drives the real editor with typed commands and checks the saved file.
use std::io::Write;
use std::process::{Command, Stdio};

fn session(args: &[&str], input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_text-editor"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("editor should start");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()
}

#[test]
fn edit_undo_redo_and_save() {
    let path = std::env::temp_dir().join("rust199_editor_test.txt");
    let _ = std::fs::remove_file(&path);
    let p = path.to_str().unwrap();
    let out = session(
        &[p],
        "a Shopping list\na rice\na dal\ni 2 mangoes\nr 3 brown rice\nd 4\nu\nu\nredo\nc 1\nv\nf rice\nw\nq\n",
    );
    assert!(out.contains("Undid: delete line 4"));
    assert!(out.contains("Redid: replace line 3"));
    assert!(out.contains("   3: brown rice"));
    let saved = std::fs::read_to_string(&path).unwrap();
    assert_eq!(saved, "Shopping list\nmangoes\nbrown rice\ndal\nShopping list\n");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn friendly_errors() {
    let out = session(&[], "d 1\nu\nr x hi\nzap\nv\nq\n");
    assert!(out.contains("Error: line 1 is out of range (1 to 0)"));
    assert!(out.contains("Error: nothing to undo"));
    assert!(out.contains("Error: 'x' is not a line number"));
    assert!(out.contains("Error: unknown command 'zap'"));
    assert!(out.contains("Error: clipboard is empty"));
}
