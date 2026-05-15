use crate::Value;
use crate::gui::MyApp;
use crate::gui::Page;
use eframe::egui;

impl MyApp {
    pub fn weather_page(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("Weather App").size(30.0).strong());

            ui.add_space(10.0);

            ui.text_edit_singleline(&mut self.location);

            ui.add_space(10.0);

            if ui.button("Enter Location").clicked() {
                self.weather_button();
            };
            ui.add_space(10.0);

            ui.label(&self.output);

            ui.add(
                egui::Image::new(egui::include_image!("../../assets/weather-icon.png"))
                    .fit_to_exact_size(egui::vec2(150.0, 150.0)),
            );
        });
    }

    fn weather_button(&mut self) {
        //Url For the location
        self.history.push(self.location.clone());
        let location_url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}",
            self.location
        );

        self.error_handled_api(&location_url);

        //Longitude and Latitude of the Location
        self.long = self.response["results"][0]["longitude"]
            .as_f64()
            .unwrap_or(0.0);
        self.lat = self.response["results"][0]["latitude"]
            .as_f64()
            .unwrap_or(0.0);

        self.history.push(self.long.clone().to_string());
        self.history.push(self.lat.clone().to_string());
        self.output = String::new();

        //Clears Forecast
        self.display_forecast = false;
        self.temp_max_points = Vec::new();
        self.temp_min_points = Vec::new();
        self.rain_data_points = Vec::new();
        self.wind_data_points = Vec::new();

        self.page = Page::Time;
    }

    fn error_handled_api(&mut self, url: &String) {
        // Error Handled for Response
        let response = match reqwest::blocking::get(url) {
            Ok(r) => r,
            Err(_) => {
                self.error = Some("Network error".to_string());
                return;
            }
        };

        let json = match response.json::<Value>() {
            Ok(j) => j,
            Err(_) => {
                self.error = Some("Failed to parse data".to_string());
                return;
            }
        };

        self.response = json;
    }
}
