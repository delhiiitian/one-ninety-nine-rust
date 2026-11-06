// The data types: what a book and a member ARE.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Genre {
    Fiction,
    Science,
    History,
    Children,
}

impl Genre {
    pub fn parse(text: &str) -> Option<Genre> {
        match text.trim().to_lowercase().as_str() {
            "fiction" => Some(Genre::Fiction),
            "science" => Some(Genre::Science),
            "history" => Some(Genre::History),
            "children" => Some(Genre::Children),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Available,
    Borrowed { member_id: u32, due_day: u32 },
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: Genre,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub borrowed: Vec<u32>, // ids of books this member holds
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match &self.status {
            Status::Available => "available".to_string(),
            Status::Borrowed { member_id, due_day } => {
                format!("borrowed by member {member_id}, due day {due_day}")
            }
        };
        write!(
            f,
            "#{:<3} {:<28} {:<20} {:<9} {}",
            self.id,
            self.title,
            self.author,
            format!("{:?}", self.genre),
            status
        )
    }
}
