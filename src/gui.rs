use colored::Colorize;
use eframe::egui;
use egui::{CentralPanel, Layout};
use serde_json::Value;

use crate::function;

pub fn run_weather_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); //Window setting

    eframe::run_native(
        //Starts the GUI
        "Weather App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp::default()))
        }),
    )
}

// Pages
#[derive(PartialEq)]
enum Page {
    Home,
    Weather,
    History,

    Time,
    Hours,
    Current,

    DetailCurrent,
    DetailHours,

}

// Variables Stored in MyApp
struct MyApp {
    page: Page,
    location: String,
    output: String,
    lat: f64,
    long: f64,
    history: Vec<String>,
    response: Value,
    first_hour: i64,
    specific_first: i64,
    specific_last: i64,
    last_hour: i64,
    time: String,
    save_name: String,

    detail_page: Option<function::info::WeatherInfo>,
    detail_info: String,
}

//Default Constructor
impl Default for MyApp {
    fn default() -> Self {
        Self {
            page: Page::Home,
            location: String::new(),
            output: String::new(),
            lat: 0.0,
            long: 0.0,
            history: Vec::new(),
            response: Value::Null,
            first_hour: 0,
            specific_first: 0,
            specific_last: 0,
            last_hour: 0,
            time: String::new(),
            save_name: String::new(),

            detail_page: None,
            detail_info: String::new(),
        }
    }
}

//Main Update Frame for GUI
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Nav Bar
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.page = Page::Home;
                }

                if ui.button("History").clicked() {
                    self.page = Page::History;
                }

                if ui.button("Weather").clicked() {
                    self.page = Page::Weather;
                }
            });

            ui.separator();

            // Pages
            match self.page {
                Page::Home => {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("About Page").size(30.0).strong());
                    });
                }

                Page::History => {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.save_name);
                        if ui.button("Save").clicked() {
                            //[Save?] ←→ [Back]
                            // let mut filename = String::new();
                            // print!("Enter a filename with .txt: ");
                            // io::stdout().flush().unwrap();
                            // io::stdin().read_line(&mut filename).unwrap();
                            // let name = filename.trim();

                            if !&self.history.is_empty() {
                                function::info::save_json(&self.save_name, &self.history);
                                println!("{}", "Saved");
                            } else {
                                println!("{}", "There nothing to save");
                            }
                        }

                        if ui.button("Clear").clicked() {
                            self.history.clear();
                        }
                    });

                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .max_width(250.0)
                        .show(ui, |ui| {
                            for item in &self.history {
                                ui.label(egui::RichText::new(item).size(15.0));
                            }
                        });
                }

                // --- [Weather] ---
                Page::Weather => {
                    // Weather Menu
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
                                egui::include_image!("../assets/weather-icon.png")
                            )
                            .fit_to_exact_size(egui::vec2(150.0, 150.0))
                        );                    });
                }
                // --- [Choose Time Range] ---
                Page::Time => {
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
                            let weather_url = function::info::hours_weather_url(self.lat, self.long);
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();

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
                            let weather_url = function::info::hours_weather_url(self.lat, self.long);
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                            self.time = "Next 24 Hours".to_string();
                            self.first_hour = 0;
                            self.last_hour = 23;
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("SixHour").clicked() {
                            //Hour URL
                            let weather_url = function::info::hours_weather_url(self.lat, self.long);
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                            self.time = "Next Six Hours".to_string();
                            self.first_hour = 0;
                            self.last_hour = 5;
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("NextHour").clicked() {
                            //Hour URL
                            let weather_url = function::info::hours_weather_url(self.lat, self.long);
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
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
                    });
                }

                Page::Current => {
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

                Page::Hours => {
                    if ui.button("Back").clicked() {
                        self.page = Page::Time
                    }
                    // Hours-Time Menu
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(format!(
                            "{}",
                            self.time.clone())).size(25.0).strong()
                        );

                        ui.add_space(10.0);

                        // Hours-Time Choices
                        if ui.button("WindDirection").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::WindDirection);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("WeatherCode").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::WeatherCode);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("WindSpeed").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::WindSpeed);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("Temperature").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::Temperature);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("Time").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::Time);
                            self.page = Page::DetailHours
                        }
                        ui.add_space(10.0);

                        if ui.button("Rain").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::Rain);
                            self.page = Page::DetailHours
                        }
                        ui.add_space(10.0);

                        if ui.button("Humidity").clicked() {
                            self.detail_info.clear();
                            self.detail_page = Some(function::info::WeatherInfo::Humidity);
                            self.page = Page::DetailHours
                        }
                    });
                }

                Page::DetailCurrent => {
                    // Detail Current Page
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.page = Page::Current
                        }

                        if ui.button("Result").clicked() {
                            if let Some(page) = &self.detail_page {
                                let detail = function::info::current_weather_info (
                                    page,
                                    &self.response,
                                    &mut self.history,
                                    &self.location
                                );
                                self.detail_info = detail;
                            }
                        }
                    });
                    ui.label(format!("{}", &self.detail_info));
                }
                Page::DetailHours => {
                    //Detail Hours Page
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.page = Page::Hours
                        }
                        if ui.button("Result").clicked() {
                            if let Some(page) = &self.detail_page {
                                let detail = function::info::hours_weather_info (
                                    page,
                                    &self.response,
                                    self.first_hour,
                                    self.last_hour,
                                    &mut self.history,
                                    &self.location
                                );
                                self.detail_info = detail
                            }
                        }
                    });
                    ui.label(&self.detail_info);
                }
            }
        });
    }
}
