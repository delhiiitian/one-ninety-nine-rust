// Chapter 6, Lesson 5: Line by line with BufReader and BufWriter
// Run: cargo run --example ch06-05

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rust199_numbers.txt");

    // BufWriter collects small writes in memory and sends them to the
    // file in big chunks: much faster than 100,000 tiny writes.
    {
        let file = File::create(&path)?;
        let mut out = BufWriter::new(file);
        for n in 1..=100_000 {
            writeln!(out, "{n}")?;
        }
        out.flush()?; // also happens automatically when `out` is dropped
    }

    // BufReader reads big chunks and hands them out line by line, so
    // even a 10 GB file never has to fit in memory.
    let reader = BufReader::new(File::open(&path)?);
    let mut sum: u64 = 0;
    let mut count = 0;
    let mut multiples_of_7 = 0;
    for line in reader.lines() {
        let n: u64 = line?.trim().parse().unwrap_or(0);
        sum += n;
        count += 1;
        if n % 7 == 0 {
            multiples_of_7 += 1;
        }
    }
    println!("Read {count} lines, sum {sum}, {multiples_of_7} multiples of 7");

    // Find the first line that matches, then stop reading.
    let reader = BufReader::new(File::open(&path)?);
    let found = reader
        .lines()
        .map_while(Result::ok)
        .position(|l| l.ends_with("777"));
    println!("First line ending in 777 is line {:?}", found.map(|i| i + 1));

    std::fs::remove_file(&path)?;
    Ok(())
}
