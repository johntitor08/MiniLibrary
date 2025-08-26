use eframe::egui;
use eframe::CreationContext;
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
}

impl LibraryApp {
    fn new(_cc: &CreationContext) -> Self {
        let books: Vec<Book> = fs::read_to_string("library.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { books }
    }

    fn save_to_file(&self) {
        let _ = fs::write("library.json", serde_json::to_string_pretty(&self.books).unwrap());
    }
}

impl eframe::App for LibraryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mini Library");

            for book in &mut self.books {
                ui.horizontal(|ui| {
                    ui.label(format!("{} - {}", book.title, book.author));
                    ui.checkbox(&mut book.read, "Okundu");
                    ui.checkbox(&mut book.favorite, "Favori");
                });
            }

            if ui.button("Kaydet").clicked() {
                self.save_to_file();
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Mini Library",
        options,
        Box::new(|cc: &CreationContext| Box::new(LibraryApp::new(cc))),
    );
}
