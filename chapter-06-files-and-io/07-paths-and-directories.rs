// Chapter 6, Lesson 7: Paths and directories
// Run: cargo run --example ch06-07

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // PathBuf builds paths safely: the right separator on every OS.
    let base: PathBuf = std::env::temp_dir().join("rust199_photos");
    let trip = base.join("2026").join("goa");
    fs::create_dir_all(&trip)?; // like mkdir -p: makes every missing folder

    for name in ["beach.jpg", "sunset.jpg", "notes.txt"] {
        fs::write(trip.join(name), name)?;
    }
    fs::write(base.join("readme.md"), "# Photos")?;

    // Pull a path apart.
    let photo = trip.join("beach.jpg");
    println!("File name: {:?}", photo.file_name());
    println!("Stem:      {:?}", photo.file_stem());
    println!("Extension: {:?}", photo.extension());
    println!("Parent:    {}", photo.parent().unwrap().display());
    println!("Exists?    {}", photo.exists());

    // List one folder.
    let mut names: Vec<String> = fs::read_dir(&trip)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    println!("In goa/: {names:?}");

    // Walk a whole tree recursively and count .jpg files.
    println!("All .jpg files: {}", count_with_extension(&base, "jpg")?);

    // Rename (move) and copy
    fs::rename(trip.join("notes.txt"), base.join("trip-notes.txt"))?;
    fs::copy(base.join("readme.md"), base.join("readme-copy.md"))?;

    fs::remove_dir_all(&base)?; // careful: deletes everything inside
    println!("Cleaned up. Exists now? {}", base.exists());
    Ok(())
}

fn count_with_extension(dir: &Path, ext: &str) -> io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            count += count_with_extension(&path, ext)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            count += 1;
        }
    }
    Ok(count)
}
