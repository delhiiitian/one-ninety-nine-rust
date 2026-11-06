// Chapter 6, Lesson 3: Writing files
// Run: cargo run --example ch06-03

use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir();

    // The quick way: write a whole string at once (creates or replaces).
    let note = dir.join("rust199_note.txt");
    fs::write(&note, "Buy milk\nCall Amma\n")?;
    println!("Wrote {}", note.display());

    // The flexible way: create a File and write to it piece by piece.
    let receipt = dir.join("rust199_receipt.txt");
    let mut file = File::create(&receipt)?;
    writeln!(file, "CHAI POINT RECEIPT")?;
    writeln!(file, "{:<12}{:>8}", "Item", "Price")?;
    let items = [("Masala chai", 30), ("Samosa", 20), ("Bun maska", 40)];
    let mut total = 0;
    for (item, price) in items {
        writeln!(file, "{item:<12}{price:>8}")?;
        total += price;
    }
    writeln!(file, "{:<12}{:>8}", "TOTAL", total)?;
    println!("Wrote {}", receipt.display());

    // Show what is in the file now.
    print!("{}", fs::read_to_string(&receipt)?);

    // Bytes work too: images, zip files and other binary data.
    let bin = dir.join("rust199_bytes.bin");
    fs::write(&bin, [0x52u8, 0x75, 0x73, 0x74])?; // "Rust" in ASCII
    println!("Bytes file says: {}", fs::read_to_string(&bin)?);

    for path in [note, receipt, bin] {
        fs::remove_file(path)?;
    }
    Ok(())
}
