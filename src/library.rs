use crate::book::Book;
use std::fs;

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        let books: Vec<Book> = fs::read_to_string("library.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { books }
    }

    pub fn save(&self) {
        let _ = fs::write(
            "library.json",
            serde_json::to_string_pretty(&self.books).unwrap(),
        );
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self, index: usize) {
        if index < self.books.len() {
            self.books.remove(index);
        }
    }
}
