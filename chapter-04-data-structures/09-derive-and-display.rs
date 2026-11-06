// Chapter 4, Lesson 9: Deriving traits and implementing Display
// Run: cargo run --example ch04-09

use std::collections::HashSet;
use std::fmt;

// derive asks the compiler to write common trait impls for you.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// Display is written by hand: it decides how users see the value.
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Default)]
struct Settings {
    volume: u8,
    dark_mode: bool,
    username: String,
}

fn main() {
    let v1 = Version { major: 1, minor: 4, patch: 2 };
    let v2 = Version { major: 1, minor: 10, patch: 0 };

    println!("Display: {v1}");          // uses Display
    println!("Debug:   {v1:?}");        // uses derived Debug
    let copy = v1.clone();              // Clone
    println!("Equal? {}", copy == v1);  // PartialEq
    println!("{v1} < {v2}? {}", v1 < v2); // PartialOrd compares field by field

    let mut versions = vec![v2.clone(), v1.clone(), Version { major: 0, minor: 9, patch: 9 }];
    versions.sort(); // Ord
    let listed: Vec<String> = versions.iter().map(|v| v.to_string()).collect();
    println!("Sorted: {}", listed.join(", "));

    let mut seen = HashSet::new(); // Hash + Eq let it go in a HashSet
    seen.insert(v1.clone());
    seen.insert(copy);
    println!("Unique versions in set: {}", seen.len());

    // Default fills every field with its default value.
    let s = Settings { volume: 7, ..Default::default() };
    println!("{s:#?}");
    println!("Volume {}, dark mode {}, user '{}'", s.volume, s.dark_mode, s.username);
}
