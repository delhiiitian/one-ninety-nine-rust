// The rules: what a library DOES with books and members.

use crate::book::{Book, Genre, Member, Status};
use std::collections::{BTreeMap, HashMap};

pub const MAX_LOANS: usize = 3;
pub const LOAN_DAYS: u32 = 14;
pub const FINE_PER_DAY: u32 = 5; // rupees

pub struct Library {
    books: BTreeMap<u32, Book>, // sorted by id, so listings are stable
    members: HashMap<u32, Member>,
    next_book_id: u32,
    next_member_id: u32,
    pub today: u32, // a simple day counter instead of real dates
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: BTreeMap::new(),
            members: HashMap::new(),
            next_book_id: 1,
            next_member_id: 1,
            today: 1,
        }
    }

    pub fn add_book(&mut self, title: &str, author: &str, genre: Genre) -> u32 {
        let id = self.next_book_id;
        self.next_book_id += 1;
        let book = Book {
            id,
            title: title.to_string(),
            author: author.to_string(),
            genre,
            status: Status::Available,
        };
        self.books.insert(id, book);
        id
    }

    pub fn add_member(&mut self, name: &str) -> u32 {
        let id = self.next_member_id;
        self.next_member_id += 1;
        self.members.insert(id, Member { id, name: name.to_string(), borrowed: Vec::new() });
        id
    }

    pub fn books(&self) -> impl Iterator<Item = &Book> {
        self.books.values()
    }

    pub fn member(&self, id: u32) -> Option<&Member> {
        self.members.get(&id)
    }

    /// Case-insensitive search in titles and authors.
    pub fn search(&self, query: &str) -> Vec<&Book> {
        let q = query.to_lowercase();
        self.books
            .values()
            .filter(|b| b.title.to_lowercase().contains(&q) || b.author.to_lowercase().contains(&q))
            .collect()
    }

    pub fn borrow_book(&mut self, book_id: u32, member_id: u32) -> Result<u32, String> {
        let member = self.members.get_mut(&member_id).ok_or(format!("no member {member_id}"))?;
        let book = self.books.get_mut(&book_id).ok_or(format!("no book {book_id}"))?;
        if book.status != Status::Available {
            return Err(format!("'{}' is already borrowed", book.title));
        }
        if member.borrowed.len() >= MAX_LOANS {
            return Err(format!("{} already has {MAX_LOANS} books", member.name));
        }
        let due_day = self.today + LOAN_DAYS;
        book.status = Status::Borrowed { member_id, due_day };
        member.borrowed.push(book_id);
        Ok(due_day)
    }

    /// Returns the book and gives back the fine in rupees (0 if on time).
    pub fn return_book(&mut self, book_id: u32) -> Result<u32, String> {
        let book = self.books.get_mut(&book_id).ok_or(format!("no book {book_id}"))?;
        let Status::Borrowed { member_id, due_day } = book.status else {
            return Err(format!("'{}' was not borrowed", book.title));
        };
        book.status = Status::Available;
        if let Some(member) = self.members.get_mut(&member_id) {
            member.borrowed.retain(|&id| id != book_id);
        }
        let days_late = self.today.saturating_sub(due_day);
        Ok(days_late * FINE_PER_DAY)
    }

    pub fn overdue(&self) -> Vec<&Book> {
        self.books
            .values()
            .filter(|b| matches!(b.status, Status::Borrowed { due_day, .. } if due_day < self.today))
            .collect()
    }

    /// Number of books per genre, in genre order.
    pub fn genre_counts(&self) -> BTreeMap<Genre, usize> {
        let mut counts = BTreeMap::new();
        for book in self.books.values() {
            *counts.entry(book.genre).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> (Library, u32, u32) {
        let mut lib = Library::new();
        let book = lib.add_book("The Guide", "R. K. Narayan", Genre::Fiction);
        let member = lib.add_member("Asha");
        (lib, book, member)
    }

    #[test]
    fn borrow_and_return_on_time() {
        let (mut lib, book, member) = sample();
        assert_eq!(lib.borrow_book(book, member), Ok(15));
        assert!(lib.borrow_book(book, member).is_err()); // already out
        assert_eq!(lib.member(member).unwrap().borrowed, vec![book]);
        lib.today = 10;
        assert_eq!(lib.return_book(book), Ok(0));
        assert!(lib.member(member).unwrap().borrowed.is_empty());
    }

    #[test]
    fn late_return_has_fine() {
        let (mut lib, book, member) = sample();
        lib.borrow_book(book, member).unwrap();
        lib.today = 20; // due on 15, so 5 days late
        assert_eq!(lib.overdue().len(), 1);
        assert_eq!(lib.return_book(book), Ok(25));
    }

    #[test]
    fn loan_limit() {
        let (mut lib, _, member) = sample();
        for i in 0..MAX_LOANS {
            let id = lib.add_book(&format!("Book {i}"), "Anon", Genre::Science);
            lib.borrow_book(id, member).unwrap();
        }
        let extra = lib.add_book("One too many", "Anon", Genre::History);
        assert!(lib.borrow_book(extra, member).unwrap_err().contains("already has"));
    }

    #[test]
    fn search_ignores_case() {
        let (lib, _, _) = sample();
        assert_eq!(lib.search("narayan").len(), 1);
        assert_eq!(lib.search("tolkien").len(), 0);
    }
}
