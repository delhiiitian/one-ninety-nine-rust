// Chapter 6, Lesson 6: Appending with OpenOptions
// Run: cargo run --example ch06-06

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn log(path: &std::path::Path, message: &str) -> io::Result<()> {
    // append(true): add to the end instead of replacing the file.
    // create(true): make the file if it does not exist yet.
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{message}")
}

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rust199_diary.log");
    let _ = fs::remove_file(&path); // start fresh; ignore "not found"

    log(&path, "07:00 woke up")?;
    log(&path, "08:30 started learning Rust")?;
    log(&path, "10:00 wrote my first file")?;
    print!("{}", fs::read_to_string(&path)?);

    // create_new(true) refuses to overwrite an existing file: a safety net.
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => println!("Created a new file"),
        Err(e) => println!("Refused to overwrite: {e}"),
    }

    // truncate(true) empties a file when it is opened for writing.
    let mut f = OpenOptions::new().write(true).truncate(true).open(&path)?;
    writeln!(f, "fresh start")?;
    print!("After truncate: {}", fs::read_to_string(&path)?);

    fs::remove_file(&path)?;
    Ok(())
}
