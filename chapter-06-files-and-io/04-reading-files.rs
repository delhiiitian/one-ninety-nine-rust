// Chapter 6, Lesson 4: Reading files
// Run: cargo run --example ch06-04

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rust199_poem.txt");
    fs::write(&path, "Where the mind is without fear\nand the head is held high\n\nWhere knowledge is free\n")?;

    // Read the whole file into one String.
    let text = fs::read_to_string(&path)?;
    println!("The file has {} bytes", text.len());

    // Then work with it: lines, words, characters.
    let lines: Vec<&str> = text.lines().collect();
    let non_empty = lines.iter().filter(|l| !l.trim().is_empty()).count();
    let words = text.split_whitespace().count();
    println!("{} lines ({} not empty), {} words", lines.len(), non_empty, words);
    println!("First line: {}", lines[0]);

    // Files that might not exist: handle the error instead of crashing.
    match fs::read_to_string("missing-file.txt") {
        Ok(t) => println!("Read {} bytes", t.len()),
        Err(e) => println!("Could not read missing-file.txt: {e}"),
    }

    // Raw bytes, for files that are not text.
    let bytes = fs::read(&path)?;
    println!("First 5 bytes: {:?}", &bytes[..5]);

    // Metadata: size and whether it is a file or folder.
    let meta = fs::metadata(&path)?;
    println!("Is a file? {}, size {} bytes", meta.is_file(), meta.len());

    fs::remove_file(&path)?;
    Ok(())
}
