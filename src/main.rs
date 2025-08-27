use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    title: String,
    author: String,
    category: String,
    read: bool,
    favorite: bool,
    cover_path: String,
}

struct LibraryApp {
    books: Vec<Book>,
    new_title: String,
    new_author: String,
    new_category: String,
    new_cover_path: String,
}

impl LibraryApp {
    fn new() -> Self {
        let books: Vec<Book> = fs::read_to_string("library.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self {
            books,
            new_title: String::new(),
            new_author: String::new(),
            new_category: String::new(),
            new_cover_path: String::from("covers/default.png"),
        }
    }

    fn save_to_file(&self) {
        let _ = fs::write(
            "library.json",
            serde_json::to_string_pretty(&self.books).unwrap(),
        );
    }

    fn add_book(&mut self, title: String, author: String, category: String, cover_path: String) {
        self.books.push(Book {
            title,
            author,
            category,
            read: false,
            favorite: false,
            cover_path,
        });
    }

    fn remove_book(&mut self, index: usize) {
        if index < self.books.len() {
            self.books.remove(index);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut to_remove: Vec<usize> = vec![];
        for (i, book) in self.books.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label("[cover]");
                ui.vertical(|ui| {
                    ui.label(format!("{} - {}", book.title, book.author));
                    ui.label(format!("Kategori: {}", book.category));
                    ui.checkbox(&mut book.read, "Okundu");
                    ui.checkbox(&mut book.favorite, "Favori");
                });
                if ui.button("Sil").clicked() {
                    to_remove.push(i);
                }
            });
            ui.separator();
        }
        for &i in to_remove.iter().rev() {
            self.remove_book(i);
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Başlık:");
            ui.text_edit_singleline(&mut self.new_title);
            ui.label("Yazar:");
            ui.text_edit_singleline(&mut self.new_author);
        });
        ui.horizontal(|ui| {
            ui.label("Kategori:");
            ui.text_edit_singleline(&mut self.new_category);
            ui.label("Kapak yolu:");
            ui.text_edit_singleline(&mut self.new_cover_path);
        });

        ui.horizontal(|ui| {
            if ui.button("Yeni Kitap Ekle").clicked() {
                if !self.new_title.is_empty() {
                    self.add_book(
                        self.new_title.clone(),
                        self.new_author.clone(),
                        self.new_category.clone(),
                        self.new_cover_path.clone(),
                    );
                    self.new_title.clear();
                    self.new_author.clear();
                    self.new_category.clear();
                    self.new_cover_path = String::from("covers/default.png");
                }
            }
            if ui.button("Kaydet").clicked() {
                self.save_to_file();
            }
        });
    }
}

impl eframe::App for LibraryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📚 Mini Library");
            ui.separator();
            self.ui(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mini Library",
        options,
        Box::new(|_cc| Box::new(LibraryApp::new())),
    )
}
