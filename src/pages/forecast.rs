use eframe::egui;
use crate::gui::MyApp;
use crate::Value;
use chrono::{NaiveDate, Datelike};

impl MyApp {
    pub fn forecast_page(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
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

                    let weather_url = format!(
                        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,rain_sum,windspeed_10m_max&timezone=auto",
                        self.lat,
                        self.long
                    );
                    self.response = reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                    self.display_forecast = true;

                } else {
                    self.output = "Location not found".to_string();
                }
            };

            ui.label(&self.output);
        });

        ui.heading("7 Day Forecast");
        if self.display_forecast == true {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..7 {
                    //Days
                    let day =
                        self.response["daily"]["time"][i]
                            .as_str()
                            .unwrap();

                    let actual_day = 
                        NaiveDate::parse_from_str(day, "%Y-%m-%d")
                        .unwrap();

                    let week = actual_day.weekday();

                    //Temps
                    let max_temp =
                        self.response["daily"]["temperature_2m_max"][i]
                            .as_f64()
                            .unwrap();

                    let min_temp =
                        self.response["daily"]["temperature_2m_min"][i]
                            .as_f64()
                            .unwrap();

                    let rain = self.response["daily"]["rain_sum"][i]
                        .as_f64()
                        .unwrap();

                    let wind = self.response["daily"]["windspeed_10m_max"][i]
                        .as_f64()
                        .unwrap();

                    ui.group(|ui| {
                        ui.label(format!("{} {}", day, week));

                        ui.label(format!(
                            "High: {}°C  Low: {}°C",
                            max_temp,
                            min_temp
                        ));

                        ui.label(format!("Rain: {} mm  Wind: {} km/h", rain, wind))
                    });

                    ui.add_space(10.0);
                }
            });
        }
    }
}