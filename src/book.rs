use serde::{Deserialize, Serialize};
use eframe::egui;

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub category: String,
    pub read: bool,
    pub favorite: bool,
    #[serde(default)]
    pub rating: u8, // 0-5
    #[serde(default)]
    pub notes: String,
}

impl Book {
    /// Seçilebilir kategoriler ("Tümü" filtresi hariç).
    pub const CATEGORIES: [&'static str; 5] =
        ["Programlama", "Roman", "Bilim", "Tarih", "Diğer"];

    pub fn category_color(category: &str) -> egui::Color32 {
        match category {
            "Programlama" => egui::Color32::LIGHT_BLUE,
            "Roman"       => egui::Color32::LIGHT_RED,
            "Bilim"       => egui::Color32::LIGHT_GREEN,
            "Tarih"       => egui::Color32::LIGHT_YELLOW,
            _             => egui::Color32::GRAY,
        }
    }
}
