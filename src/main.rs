mod book;
mod library;
mod ui;

use eframe::egui;
use ui::LibraryUI;

impl eframe::App for LibraryUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📚 Mini Library");
            ui.separator();
            self.ui(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 640.0])
            .with_min_inner_size([480.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mini Library",
        options,
        Box::new(|_cc| Box::new(LibraryUI::new())),
    )
}
