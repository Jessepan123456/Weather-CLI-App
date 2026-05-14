use eframe::egui;
use crate::{function};
use crate::gui::MyApp;
use crate::gui::Page;

impl MyApp{
    pub fn current_page(&mut self, ui: &mut egui::Ui) {
        if ui.button("Back").clicked() {
            self.page = Page::Time
        }
        // Current-Time Menu
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(format!(
                "{}",
                self.time.clone())).size(25.0).strong()
            );

            ui.add_space(10.0);

            // Current-Time Choices
            if ui.button("WindDirection").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::WindDirection);
                self.page = Page::DetailCurrent
            }

            ui.add_space(10.0);

            if ui.button("TimeZone").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::TimeZone);
                self.page = Page::DetailCurrent
            }

            ui.add_space(10.0);

            if ui.button("WeatherCode").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::WeatherCode);
                self.page = Page::DetailCurrent
            }

            ui.add_space(10.0);

            if ui.button("WindSpeed").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::WindSpeed);
                self.page = Page::DetailCurrent
            }

            ui.add_space(10.0);

            if ui.button("Temperature").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::Temperature);
                self.page = Page::DetailCurrent
            }
            ui.add_space(10.0);

            if ui.button("Time").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::Time);
                self.page = Page::DetailCurrent
            }
            ui.add_space(10.0);

            if ui.button("All Info").clicked() {
                self.detail_info.clear();
                self.detail_page = Some(function::info::WeatherInfo::AllInfo);
                self.page = Page::DetailCurrent
            }
        });
    }
}