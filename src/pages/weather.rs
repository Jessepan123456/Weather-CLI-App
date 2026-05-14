 use eframe::egui;
use crate::gui::MyApp;
use crate::Value;
use crate::gui::Page;
 
impl MyApp{
    pub fn weather_page(&mut self, ui : &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("Weather App").size(30.0).strong());

            ui.add_space(10.0);

            ui.text_edit_singleline(&mut self.location);

            ui.add_space(10.0);

            if ui.button("Enter Location").clicked() {
                //Url For the location
                self.history.push(self.location.clone());
                let location_url = format!(
                    "https://geocoding-api.open-meteo.com/v1/search?name={}",
                    self.location
                );

                let response: Value = reqwest::blocking::get(location_url)
                    .unwrap()
                    .json()
                    .unwrap();

                let is_valid = response["results"]
                    .as_array()
                    .map(|arr| !arr.is_empty())
                    .unwrap_or(false);

                if is_valid {
                    //Longitude and Latitude of the Location
                    self.long = response["results"][0]["longitude"].as_f64().unwrap();
                    self.lat = response["results"][0]["latitude"].as_f64().unwrap();

                    self.history.push(self.long.clone().to_string());
                    self.history.push(self.lat.clone().to_string());
                    self.output = String::new();
                    self.page = Page::Time;
                } else {
                    self.output = "Location not found".to_string();
                    self.history.push(self.output.clone());
                }
            };
            ui.add_space(10.0);

            ui.label(&self.output);

            ui.add(
                egui::Image::new(
                    egui::include_image!("../../assets/weather-icon.png")
                )
                .fit_to_exact_size(egui::vec2(150.0, 150.0))
            );        
        });
    }  
}