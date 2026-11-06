// Project 4: Library management system
//
//   cargo run -p library-system
// Then type `help` to see the commands.

mod book;
mod library;

use book::Genre;
use library::Library;
use std::io::{self, BufRead, Write};

fn main() {
    let mut lib = Library::new();
    seed(&mut lib);
    println!("City Library. Type `help` for commands.");

    let stdin = io::stdin();
    loop {
        print!("library> ");
        io::stdout().flush().expect("could not flush stdout");
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).expect("could not read input") == 0 {
            break; // end of input
        }
        let line = line.trim();
        let (command, rest) = line.split_once(' ').unwrap_or((line, ""));
        match command {
            "" => {}
            "help" => help(),
            "list" => lib.books().for_each(|b| println!("{b}")),
            "search" => {
                let found = lib.search(rest);
                if found.is_empty() {
                    println!("No books match '{rest}'");
                }
                found.iter().for_each(|b| println!("{b}"));
            }
            "add" => add(&mut lib, rest),
            "join" => {
                let id = lib.add_member(rest);
                println!("Welcome, {rest}! Your member id is {id}.");
            }
            "borrow" => match two_numbers(rest) {
                Some((book, member)) => match lib.borrow_book(book, member) {
                    Ok(due) => println!("Borrowed. Please return by day {due}."),
                    Err(e) => println!("Cannot borrow: {e}"),
                },
                None => println!("Usage: borrow <book id> <member id>"),
            },
            "return" => match rest.parse() {
                Ok(book) => match lib.return_book(book) {
                    Ok(0) => println!("Returned on time. Thank you!"),
                    Ok(fine) => println!("Returned late. Fine: Rs {fine}"),
                    Err(e) => println!("Cannot return: {e}"),
                },
                Err(_) => println!("Usage: return <book id>"),
            },
            "member" => match rest.parse().ok().and_then(|id| lib.member(id)) {
                Some(m) => println!("{} (#{}) holds books {:?}", m.name, m.id, m.borrowed),
                None => println!("No such member"),
            },
            "overdue" => lib.overdue().iter().for_each(|b| println!("{b}")),
            "stats" => {
                for (genre, n) in lib.genre_counts() {
                    println!("{genre:?}: {n}");
                }
            }
            "day" => {
                lib.today += rest.parse::<u32>().unwrap_or(1);
                println!("It is now day {}", lib.today);
            }
            "quit" => break,
            other => println!("Unknown command '{other}'. Type `help`."),
        }
    }
    println!("Library closed.");
}

fn help() {
    println!("  list                          all books");
    println!("  search <text>                 find by title or author");
    println!("  add <title> | <author> | <genre>   genres: fiction science history children");
    println!("  join <name>                   new member");
    println!("  borrow <book id> <member id>");
    println!("  return <book id>");
    println!("  member <id>                   who holds what");
    println!("  overdue                       late books");
    println!("  stats                         books per genre");
    println!("  day [n]                       move the clock forward n days");
    println!("  quit");
}

fn add(lib: &mut Library, rest: &str) {
    let parts: Vec<&str> = rest.split('|').map(|p| p.trim()).collect();
    let [title, author, genre] = parts[..] else {
        println!("Usage: add <title> | <author> | <genre>");
        return;
    };
    match Genre::parse(genre) {
        Some(g) => println!("Added book #{}", lib.add_book(title, author, g)),
        None => println!("Unknown genre '{genre}'"),
    }
}

fn two_numbers(text: &str) -> Option<(u32, u32)> {
    let mut parts = text.split_whitespace().map(|p| p.parse::<u32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => Some((a, b)),
        _ => None,
    }
}

fn seed(lib: &mut Library) {
    lib.add_book("The Guide", "R. K. Narayan", Genre::Fiction);
    lib.add_book("Wings of Fire", "A. P. J. Abdul Kalam", Genre::Science);
    lib.add_book("The Discovery of India", "Jawaharlal Nehru", Genre::History);
    lib.add_book("Panchatantra Tales", "Vishnu Sharma", Genre::Children);
    lib.add_book("A Brief History of Time", "Stephen Hawking", Genre::Science);
    lib.add_member("Asha");
    lib.add_member("Dev");
}
