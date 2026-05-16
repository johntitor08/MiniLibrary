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
    pub expanded_index: Option<usize>, // hangi kitabın notu açık
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
            expanded_index: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let categories = vec!["Tümü", "Programlama", "Roman", "Bilim", "Tarih", "Diğer"];

        // --- Filtre satırı ---
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

        // --- İstatistik ---
        let total = self.library.books.len();
        let read_count = self.library.books.iter().filter(|b| b.read).count();
        let fav_count = self.library.books.iter().filter(|b| b.favorite).count();
        ui.horizontal(|ui| {
            ui.label(format!("📚 Toplam: {}", total));
            ui.separator();
            ui.label(format!("✅ Okundu: {}", read_count));
            ui.separator();
            ui.label(format!("⭐ Favori: {}", fav_count));
        });

        ui.separator();

        // --- Favori Kitaplar ---
        ui.heading("Favori Kitaplar");
        egui::ScrollArea::horizontal().show(ui, |ui| {
            for book in self.library.books.iter().filter(|b| b.favorite) {
                ui.colored_label(Book::category_color(&book.category), &book.title);
            }
        });

        ui.separator();

        // --- Filtrelenmiş liste ---
        let filtered_indices: Vec<usize> = self
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
            .map(|(i, _)| i)
            .collect();

        let display_indices = if self.sort_by_title {
            let mut sorted = filtered_indices.clone();
            sorted.sort_by(|&a, &b| {
                self.library.books[a].title.cmp(&self.library.books[b].title)
            });
            sorted
        } else {
            filtered_indices
        };

        let mut to_remove: Option<usize> = None;
        let mut toggle_expand: Option<usize> = None;

        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for &idx in &display_indices {
                let book = &self.library.books[idx];
                let title = book.title.clone();
                let author = book.author.clone();
                let category = book.category.clone();
                let rating = book.rating;
                let is_expanded = self.expanded_index == Some(idx);

                ui.horizontal(|ui| {
                    // Başlık - Yazar
                    ui.colored_label(
                        Book::category_color(&category),
                        format!("{} - {}", title, author),
                    );

                    // Okundu checkbox
                    let mut read = self.library.books[idx].read;
                    if ui.checkbox(&mut read, "").changed() {
                        self.library.books[idx].read = read;
                    }

                    // Favori checkbox
                    let mut fav = self.library.books[idx].favorite;
                    if ui.checkbox(&mut fav, "⭐").changed() {
                        self.library.books[idx].favorite = fav;
                    }

                    // Puan (1-5 yıldız)
                    for star in 1u8..=5 {
                        let filled = star <= rating;
                        let label = if filled { "★" } else { "☆" };
                        let color = if filled {
                            egui::Color32::GOLD
                        } else {
                            egui::Color32::DARK_GRAY
                        };
                        if ui.add(egui::Label::new(
                            egui::RichText::new(label).color(color).size(16.0)
                        ).sense(egui::Sense::click())).clicked() {
                            // Aynı yıldıza tıklanırsa sıfırla
                            self.library.books[idx].rating = if self.library.books[idx].rating == star { 0 } else { star };
                        }
                    }

                    // Not butonu
                    if ui.button("📝").clicked() {
                        toggle_expand = Some(idx);
                    }

                    // Sil
                    if ui.button("Sil").clicked() {
                        to_remove = Some(idx);
                    }
                });

                // Not alanı (açıksa)
                if is_expanded {
                    ui.horizontal(|ui| {
                        ui.label("Not:");
                        ui.text_edit_multiline(&mut self.library.books[idx].notes);
                    });
                }
            }
        });

        if let Some(idx) = toggle_expand {
            if self.expanded_index == Some(idx) {
                self.expanded_index = None;
            } else {
                self.expanded_index = Some(idx);
            }
        }

        if let Some(idx) = to_remove {
            self.library.remove_book(idx);
            if self.expanded_index == Some(idx) {
                self.expanded_index = None;
            }
        }

        // --- Yeni kitap ekleme ---
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
                        rating: 0,
                        notes: String::new(),
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
