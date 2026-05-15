use eframe::egui;
use crate::{function};
use crate::gui::MyApp;
use crate::gui::Page;

impl MyApp{
    pub fn time_page(&mut self, ui: &mut egui::Ui) {
        if ui.button("Back").clicked() {
            self.page = Page::Time
        }
        // Time-Range Menu
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(format!(
                    "Lat: {}, Long: {}",
                    self.lat.clone(),
                    self.long.clone()
                ))
                .size(25.0)
                .strong(),
            );

            ui.add_space(10.0);

            // Time-Range Choices
            ui.label("SpecificHour");

            ui.add_space(10.0);
            
            //Hours Choices
            // ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.specific_first).range(0..=23));
            ui.add(egui::DragValue::new(&mut self.specific_last).range(0..=23));
            // });

            if ui.button("Confirm Hours").clicked() {
                //Hour URL
                self.url_response();

                //Specfic Time
                self.first_hour = self.specific_first;
                self.last_hour = self.specific_last;
                if &self.first_hour <= &self.last_hour {
                    self.history.push(self.first_hour.clone().to_string());
                    self.history.push(self.last_hour.clone().to_string());

                    self.time = format!("Next {} Hour to {} Hour", self.first_hour, self.last_hour);
                    self.page = Page::Hours
                }
                else{
                self.output = "Invalid Hours".to_string();
                }
            }
            ui.label(&self.output);

            ui.add_space(15.0);

            if ui.button("Day").clicked() {
                //Hour URL
                self.url_response();
                self.time = "Next 24 Hours".to_string();
                self.first_hour = 0;
                self.last_hour = 23;
                self.page = Page::Hours
            }

            ui.add_space(10.0);

            if ui.button("SixHour").clicked() {
                //Hour URL
                self.url_response();
                self.time = "Next Six Hours".to_string();
                self.first_hour = 0;
                self.last_hour = 5;
                self.page = Page::Hours
            }

            ui.add_space(10.0);

            if ui.button("NextHour").clicked() {
                //Hour URL
                self.url_response();
                self.time = "Next Hour".to_string();
                self.first_hour = 0;
                self.last_hour = 1;
                self.page = Page::Hours
            }

            ui.add_space(10.0);

            if ui.button("CurrentTime").clicked() {
                //Current URL
                let weather_url = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                    self.lat, self.long
                );
                self.response =
                    reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                self.time = "Current Time".to_string();
                self.page = Page::Current
            }
            ui.add(
                egui::Image::new(
                    egui::include_image!("../../assets/time.png")
                )
                .fit_to_exact_size(egui::vec2(150.0, 150.0))
            );  
        });
    }

    //Helper Method
    fn url_response(&mut self) {
        let weather_url = function::info::hours_weather_url(self.lat, self.long);
        self.response =
            reqwest::blocking::get(weather_url).unwrap().json().unwrap();
    }
}
