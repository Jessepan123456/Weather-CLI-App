use eframe::egui;
use crate::{function};
use crate::gui::MyApp;

impl MyApp {
    pub fn history_page(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.save_name);

            if ui.button("Save").clicked() {
                if !self.history.is_empty() {
                    function::info::save_json(
                        &self.save_name,
                        &self.history
                    );

                    println!("Saved");
                }
            }

            if ui.button("Clear").clicked() {
                self.history.clear();
            }
        });

        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for item in &self.history {
                    ui.label(item);
                }
            });
    }
}