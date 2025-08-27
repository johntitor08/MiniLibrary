use crate::book::Book;
use crate::library::Library;
use eframe::egui;

pub struct LibraryUI {
    pub library: Library,
    pub new_title: String,
    pub new_author: String,
    pub new_category: String,
    pub category_filter: String,
    pub search_query: String,
    pub sort_by_title: bool,
}

impl LibraryUI {
    pub fn new() -> Self {
        Self {
            library: Library::new(),
            new_title: String::new(),
            new_author: String::new(),
            new_category: String::new(),
            category_filter: String::from("Tümü"),
            search_query: String::new(),
            sort_by_title: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let categories = vec!["Tümü", "Programlama", "Roman", "Bilim", "Tarih", "Diğer"];

        ui.horizontal(|ui| {
            ui.label("Kategori Filtre:");
            egui::ComboBox::from_label("")
                .selected_text(&self.category_filter)
                .show_ui(ui, |ui| {
                    for cat in &categories {
                        ui.selectable_value(&mut self.category_filter, cat.to_string(), *cat);
                    }
                });
            ui.label("Ara:");
            ui.text_edit_singleline(&mut self.search_query);
            ui.checkbox(&mut self.sort_by_title, "Başlığa Göre Sırala");
        });

        ui.separator();
        ui.heading("Favori Kitaplar");
        egui::ScrollArea::horizontal().show(ui, |ui| {
            for book in self.library.books.iter().filter(|b| b.favorite) {
                ui.colored_label(Book::category_color(&book.category), &book.title);
            }
        });

        let mut to_remove: Vec<usize> = vec![];
        let filtered_books: Vec<(usize, String, String, String, bool, bool)> = self
            .library
            .books
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                (self.category_filter == "Tümü" || b.category == self.category_filter)
                    && (self.search_query.is_empty()
                        || b.title.to_lowercase().contains(&self.search_query.to_lowercase())
                        || b.author.to_lowercase().contains(&self.search_query.to_lowercase()))
            })
            .map(|(i, b)| (i, b.title.clone(), b.author.clone(), b.category.clone(), b.read, b.favorite))
            .collect();

        let display_books = if self.sort_by_title {
            let mut sorted = filtered_books.clone();
            sorted.sort_by(|a, b| a.1.cmp(&b.1));
            sorted
        } else {
            filtered_books
        };

        for (idx, title, author, category, read, _favorite) in display_books.iter() {
            ui.horizontal(|ui| {
                ui.colored_label(Book::category_color(category), format!("{} - {}", title, author));
                ui.label(if *read { "📖" } else { "🆕" });
                ui.checkbox(&mut self.library.books[*idx].favorite, "⭐");
                if ui.button("Sil").clicked() {
                    to_remove.push(*idx);
                }
            });
        }

        for &i in to_remove.iter().rev() {
            self.library.remove_book(i);
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
        });
        ui.horizontal(|ui| {
            if ui.button("Yeni Kitap Ekle").clicked() {
                if !self.new_title.is_empty() {
                    self.library.add_book(Book {
                        title: self.new_title.clone(),
                        author: self.new_author.clone(),
                        category: self.new_category.clone(),
                        read: false,
                        favorite: false,
                    });
                    self.new_title.clear();
                    self.new_author.clear();
                    self.new_category.clear();
                }
            }
            if ui.button("Kaydet").clicked() {
                self.library.save();
            }
        });
    }
}
