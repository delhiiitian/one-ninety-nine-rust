// Chapter 5, Lesson 9: Box<dyn Error> and a main that returns Result
// Run: cargo run --example ch05-09

use std::error::Error;
use std::fs;

// Box<dyn Error> means "any kind of error". Perfect for applications
// where you just want to report the error, not handle each kind.
fn read_total(path: &str) -> Result<f64, Box<dyn Error>> {
    let text = fs::read_to_string(path)?; // io::Error
    let mut total = 0.0;
    for (line_no, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value: f64 = line.parse()?; // ParseFloatError
        if value < 0.0 {
            // A plain message can become a Box<dyn Error> too.
            return Err(format!("line {}: negative amount {value}", line_no + 1).into());
        }
        total += value;
    }
    Ok(total)
}

// main can return Result. If it returns Err, Rust prints the error
// and the program exits with a failure code (1).
fn main() -> Result<(), Box<dyn Error>> {
    let dir = std::env::temp_dir();
    let good = dir.join("rust199_expenses.txt");
    let bad = dir.join("rust199_bad.txt");
    fs::write(&good, "120.50\n80\n\n49.50\n")?;
    fs::write(&bad, "100\nten\n")?;

    println!("Expenses total: {}", read_total(good.to_str().unwrap())?);

    match read_total(bad.to_str().unwrap()) {
        Ok(t) => println!("Total: {t}"),
        Err(e) => println!("Could not total the bad file: {e}"),
    }
    match read_total("no/such/file.txt") {
        Ok(t) => println!("Total: {t}"),
        Err(e) => println!("Could not total the missing file: {e}"),
    }

    fs::remove_file(good)?;
    fs::remove_file(bad)?;
    Ok(()) // success
}
