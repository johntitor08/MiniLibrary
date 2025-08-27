use serde::{Deserialize, Serialize};
use eframe::egui;

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub category: String,
    pub read: bool,
    pub favorite: bool,
}

impl Book {
    pub fn category_color(category: &str) -> egui::Color32 {
        match category {
            "Programlama" => egui::Color32::LIGHT_BLUE,
            "Roman" => egui::Color32::LIGHT_RED,
            "Bilim" => egui::Color32::LIGHT_GREEN,
            "Tarih" => egui::Color32::LIGHT_YELLOW,
            _ => egui::Color32::GRAY,
        }
    }
}
