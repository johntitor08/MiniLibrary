use eframe::egui::{self, RichText};
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("+ Kitap Ekle").clicked() {
                    // TODO: Yeni kitap ekleme dialogu
                    println!("Kitap ekleye tıklandı!");
                }
                if ui.button("🔎 Filtrele").clicked() {
                    // TODO: Filtreleme menüsü
                    println!("Filtrelemeye tıklandı!");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mini Library");

            // Tablo başlıkları
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Kapak").strong());
                ui.label(RichText::new("Kitap Başlığı").strong());
                ui.label(RichText::new("Yazar").strong());
                ui.label(RichText::new("Kategori").strong());
                ui.label(RichText::new("Okundu").strong());
                ui.label(RichText::new("Favori").strong());
            });
            ui.separator();

            for book in &mut self.books {
                ui.horizontal(|ui| {
                    // Kapak
                    if !book.cover_path.is_empty() {
                        let img = egui::Image::new(
                            egui::TextureId::User(0), // placeholder, gerçek load edilirse texture atanır
                            [20.0, 20.0],
                        );
                        ui.add(img); 
                    } else {
                        ui.label("📕");
                    }

                    ui.label(&book.title);
                    ui.label(&book.author);
                    ui.label(&book.category);
                    ui.checkbox(&mut book.read, "");
                    ui.checkbox(&mut book.favorite, "");
                });
            }

            ui.separator();

            // İstatistikler
            let total = self.books.len();
            let read_count = self.books.iter().filter(|b| b.read).count();
            let fav_count = self.books.iter().filter(|b| b.favorite).count();

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.label(format!("Toplam Kitap: {}", total));
                    ui.label(format!("Okunan: {}", read_count));
                    ui.label(format!("Favori: {}", fav_count));
                });

                if ui.button("Kaydet").clicked() {
                    self.save_to_file();
                }
            });
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
