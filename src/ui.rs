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
    dirty: bool,                       // kaydedilmemiş değişiklik var mı
}

impl Default for LibraryUI {
    fn default() -> Self {
        Self::new()
    }
}

impl LibraryUI {
    pub fn new() -> Self {
        Self {
            library: Library::new(),
            new_title: String::new(),
            new_author: String::new(),
            new_category: String::from(Book::CATEGORIES[0]),
            category_filter: String::from("Tümü"),
            search_query: String::new(),
            sort_by_title: false,
            expanded_index: None,
            dirty: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // "Tümü" filtresi + tanımlı kategoriler.
        let filter_categories =
            std::iter::once("Tümü").chain(Book::CATEGORIES.iter().copied());

        // --- Filtre satırı ---
        ui.horizontal(|ui| {
            ui.label("Kategori Filtre:");
            egui::ComboBox::from_id_source("category_filter")
                .selected_text(&self.category_filter)
                .show_ui(ui, |ui| {
                    for cat in filter_categories {
                        ui.selectable_value(&mut self.category_filter, cat.to_string(), cat);
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
        let progress = if total > 0 {
            (read_count as f32 / total as f32 * 100.0).round() as u32
        } else {
            0
        };
        ui.horizontal(|ui| {
            ui.label(format!("📚 Toplam: {}", total));
            ui.separator();
            ui.label(format!("✅ Okundu: {}", read_count));
            ui.separator();
            ui.label(format!("⭐ Favori: {}", fav_count));
            ui.separator();
            ui.label(format!("📈 İlerleme: %{}", progress));
        });

        ui.separator();

        // --- Favori Kitaplar ---
        ui.heading("Favori Kitaplar");
        if fav_count == 0 {
            ui.weak("(henüz favori yok)");
        } else {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for book in self.library.books.iter().filter(|b| b.favorite) {
                        ui.colored_label(Book::category_color(&book.category), &book.title);
                    }
                });
            });
        }

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

        ui.weak(format!("{} kitap gösteriliyor", display_indices.len()));

        let mut to_remove: Option<usize> = None;
        let mut toggle_expand: Option<usize> = None;

        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            if display_indices.is_empty() {
                ui.label("Eşleşen kitap yok.");
            }
            for &idx in &display_indices {
                let book = &self.library.books[idx];
                let title = book.title.clone();
                let author = book.author.clone();
                let category = book.category.clone();
                let rating = book.rating;
                let is_expanded = self.expanded_index == Some(idx);

                let is_read = self.library.books[idx].read;

                ui.horizontal(|ui| {
                    // Başlık - Yazar (okunmuşsa üstü çizili)
                    let mut text = egui::RichText::new(format!("{} - {}", title, author))
                        .color(Book::category_color(&category));
                    if is_read {
                        text = text.strikethrough();
                    }
                    ui.label(text);

                    // Okundu checkbox
                    let mut read = self.library.books[idx].read;
                    if ui
                        .checkbox(&mut read, "")
                        .on_hover_text("Okundu olarak işaretle")
                        .changed()
                    {
                        self.library.books[idx].read = read;
                        self.dirty = true;
                    }

                    // Favori checkbox
                    let mut fav = self.library.books[idx].favorite;
                    if ui
                        .checkbox(&mut fav, "⭐")
                        .on_hover_text("Favori")
                        .changed()
                    {
                        self.library.books[idx].favorite = fav;
                        self.dirty = true;
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
                        ).sense(egui::Sense::click()))
                            .on_hover_text(format!("{} yıldız ver", star))
                            .clicked() {
                            // Aynı yıldıza tıklanırsa sıfırla
                            self.library.books[idx].rating = if self.library.books[idx].rating == star { 0 } else { star };
                            self.dirty = true;
                        }
                    }

                    // Not butonu
                    if ui.button("📝").on_hover_text("Not ekle/düzenle").clicked() {
                        toggle_expand = Some(idx);
                    }

                    // Sil
                    if ui.button("Sil").on_hover_text("Kitabı sil").clicked() {
                        to_remove = Some(idx);
                    }
                });

                // Not alanı (açıksa)
                if is_expanded {
                    ui.horizontal(|ui| {
                        ui.label("Not:");
                        // Not düzenlemesi odak kaybında kaydedilir; her tuş
                        // vuruşunda diske yazmamak için.
                        if ui
                            .text_edit_multiline(&mut self.library.books[idx].notes)
                            .lost_focus()
                        {
                            self.dirty = true;
                        }
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
            self.dirty = true;
        }

        // --- Yeni kitap ekleme ---
        ui.separator();
        // Başlık ya da yazar alanında Enter'a basılınca da kitap eklenir.
        let mut submit = false;
        ui.horizontal(|ui| {
            ui.label("Başlık:");
            submit |= ui.text_edit_singleline(&mut self.new_title).lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));
            ui.label("Yazar:");
            submit |= ui.text_edit_singleline(&mut self.new_author).lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));
        });
        ui.horizontal(|ui| {
            ui.label("Kategori:");
            egui::ComboBox::from_id_source("new_category")
                .selected_text(&self.new_category)
                .show_ui(ui, |ui| {
                    for cat in Book::CATEGORIES {
                        ui.selectable_value(&mut self.new_category, cat.to_string(), cat);
                    }
                });
        });
        ui.horizontal(|ui| {
            submit |= ui.button("Yeni Kitap Ekle").clicked();
            if ui.button("Kaydet").clicked() {
                self.dirty = true;
            }
        });

        if submit {
            let title = self.new_title.trim();
            if !title.is_empty() {
                self.library.add_book(Book {
                    title: title.to_string(),
                    author: self.new_author.trim().to_string(),
                    category: self.new_category.clone(),
                    read: false,
                    favorite: false,
                    rating: 0,
                    notes: String::new(),
                });
                self.new_title.clear();
                self.new_author.clear();
                self.dirty = true;
            }
        }

        // Değişiklik varsa kare sonunda otomatik kaydet.
        if self.dirty {
            self.library.save();
            self.dirty = false;
        }
    }
}
