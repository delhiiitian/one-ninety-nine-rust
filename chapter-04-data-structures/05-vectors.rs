// Chapter 4, Lesson 5: Vec, a growable list
// Run: cargo run --example ch04-05

fn main() {
    // Create
    let mut playlist: Vec<String> = Vec::new();
    let mut ratings = vec![4, 5, 3];

    // Add to the end
    playlist.push("Kesariya".to_string());
    playlist.push("Naatu Naatu".to_string());
    playlist.push("Jai Ho".to_string());
    ratings.push(5);

    // Read
    println!("First song: {}", playlist[0]);
    println!("Song count: {}", playlist.len());
    match playlist.get(5) {
        Some(song) => println!("Song 6: {song}"),
        None => println!("There is no 6th song"),
    }

    // Insert, remove and pop
    playlist.insert(1, "Chaiyya Chaiyya".to_string());
    let removed = playlist.remove(0);
    println!("Removed {removed}");
    if let Some(last) = playlist.pop() {
        println!("Popped {last}");
    }
    println!("Playlist now: {playlist:?}");

    // Change items in place with iter_mut
    for r in ratings.iter_mut() {
        *r *= 2; // ratings out of 10 now
    }
    println!("Ratings out of 10: {ratings:?}");

    // Sorting, searching and other common operations
    ratings.sort();
    ratings.dedup(); // remove repeated neighbours
    println!("Sorted unique: {ratings:?}");
    println!("Contains 10? {}", ratings.contains(&10));
    let total: i32 = ratings.iter().sum();
    println!("Sum {total}, average {:.1}", total as f64 / ratings.len() as f64);

    // retain keeps only what matches
    let mut temps = vec![31, 45, 29, 50, 33];
    temps.retain(|&t| t < 40);
    println!("Plausible temperatures: {temps:?}");

    // A Vec of structs-like tuples, sorted by a field
    let mut players = vec![("Kohli", 82), ("Rohit", 64), ("Gill", 91)];
    players.sort_by(|a, b| b.1.cmp(&a.1)); // highest score first
    println!("Leaderboard: {players:?}");
}
