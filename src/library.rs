use crate::book::Book;
use std::fs;

pub struct Library {
    pub books: Vec<Book>,
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
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
        match serde_json::to_string_pretty(&self.books) {
            Ok(json) => {
                if let Err(e) = fs::write("library.json", json) {
                    eprintln!("library.json kaydedilemedi: {e}");
                }
            }
            Err(e) => eprintln!("Kitaplar serileştirilemedi: {e}"),
        }
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
