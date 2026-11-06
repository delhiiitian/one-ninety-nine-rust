// Runs the finished program on the sample log, like a user would.
use std::process::Command;

fn run(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_log-analyzer"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("program should run");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn full_report() {
    let (ok, out, _) = run(&["sample.log"]);
    assert!(ok);
    assert!(out.contains("LOG REPORT"));
    assert!(out.contains("122 total"));
    assert!(out.contains("line 38: missing timestamp"));
}

#[test]
fn filter_errors_only() {
    let (ok, out, _) = run(&["sample.log", "--level", "error"]);
    assert!(ok);
    assert!(!out.contains("  INFO"));
    assert!(out.contains("ERROR"));
}

#[test]
fn missing_file_fails_cleanly() {
    let (ok, _, err) = run(&["nope.log"]);
    assert!(!ok);
    assert!(err.contains("cannot open 'nope.log'"));
}

#[test]
fn writes_report_file() {
    let path = std::env::temp_dir().join("rust199_log_report.txt");
    let (ok, out, _) = run(&["sample.log", "--output", path.to_str().unwrap()]);
    assert!(ok, "{out}");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.starts_with("LOG REPORT"));
    std::fs::remove_file(path).unwrap();
}
