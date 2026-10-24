// Plays a whole game by piping keystrokes into the real program.
use std::io::Write;
use std::process::{Command, Stdio};

fn play(seed: &str, input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_guessing-game"))
        .args(["--seed", seed])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("game should start");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn binary_search_always_wins_medium() {
    // Find the secret for seed 7 by halving the range, like a person would.
    let mut low = 1;
    let mut high = 100;
    let mut input = String::from("2\n");
    for _ in 0..7 {
        let mid = (low + high) / 2;
        input.push_str(&format!("{mid}\n"));
        let out = play("7", &input);
        if out.contains("Correct!") {
            assert!(out.contains("Thanks for playing!"));
            return;
        }
        // The newest hint is the last line that mentions higher or lower.
        let hint = out
            .lines()
            .rev()
            .find(|l| l.contains("higher") || l.contains("lower") || l.contains("Too "))
            .expect("a hint");
        if hint.contains("higher") || hint.contains("Too low") {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    panic!("binary search should find the number within 7 tries");
}

#[test]
fn rejects_bad_input() {
    let out = play("1", "9\n2\nabc\n500\n");
    assert!(out.contains("Please type 1, 2 or 3."));
    assert!(out.contains("That's not a whole number."));
    assert!(out.contains("Stay between 1 and 100."));
}
