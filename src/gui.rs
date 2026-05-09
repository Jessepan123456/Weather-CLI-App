use colored::Colorize;
use eframe::egui;
use egui::{CentralPanel, Layout};
use serde_json::Value;
use std::io;
use std::io::Write;
use std::process;

use crate::function;

pub fn run_weather_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); //Window setting

    eframe::run_native(
        //Starts the GUI
        "Weather App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
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
    first_hour: i32,
    last_hour: i32,
    time: String,

    detail_page: Option<WeatherInfo>,
}

// WeatherInfo
pub enum WeatherInfo {
    Time,
    Temperature,
    WindSpeed,
    WeatherCode,
    WindDirection,
    TimeZone,
    Rain,
    Humidity,
    AllInfo,
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
            last_hour: 0,
            time: String::new(),

            detail_page: None,
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
                    ui.label("This is the Home Page");
                }

                Page::History => {
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {}

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
                                self.page = Page::Time;
                            } else {
                                self.output = "Location not found".to_string();
                                self.history.push(self.output.clone());
                            }
                        };
                        ui.add_space(10.0);

                        ui.label(&self.output);
                    });
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
                        if ui.button("SpecificHour").clicked() {
                            //Hour URL
                            let weather_url = format!(
                                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                                self.lat, self.long
                            );
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                        }

                        ui.add_space(10.0);
                        
                        //Hours Choices
                        // ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.first_hour).range(0..=23));
                        ui.add(egui::DragValue::new(&mut self.last_hour).range(0..=23));
                        // });

                        if ui.button("Confirm Hours").clicked() {
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

                        ui.add_space(20.0);

                        if ui.button("Day").clicked() {
                            //Hour URL
                            let weather_url = format!(
                                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                                self.lat, self.long
                            );
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                            self.time = "Next 24 Hours".to_string();
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("SixHour").clicked() {
                            //Hour URL
                            let weather_url = format!(
                                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                                self.lat, self.long
                            );
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                            self.time = "Next Six Hours".to_string();
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("NextHour").clicked() {
                            //Hour URL
                            let weather_url = format!(
                                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                                self.lat, self.long
                            );
                            self.response =
                                reqwest::blocking::get(weather_url).unwrap().json().unwrap();
                            self.time = "Next Hour".to_string();
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
                            self.detail_page = Some(WeatherInfo::WindDirection);
                            self.page = Page::DetailCurrent
                        }

                        ui.add_space(10.0);

                        if ui.button("TimeZone").clicked() {
                            self.detail_page = Some(WeatherInfo::TimeZone);
                            self.page = Page::DetailCurrent
                        }

                        ui.add_space(10.0);

                        if ui.button("WeatherCode").clicked() {
                            self.detail_page = Some(WeatherInfo::WeatherCode);
                            self.page = Page::DetailCurrent
                        }

                        ui.add_space(10.0);

                        if ui.button("WindSpeed").clicked() {
                            self.detail_page = Some(WeatherInfo::WindSpeed);
                            self.page = Page::DetailCurrent
                        }

                        ui.add_space(10.0);

                        if ui.button("Temperature").clicked() {
                            self.detail_page = Some(WeatherInfo::Temperature);
                            self.page = Page::DetailCurrent
                        }
                        ui.add_space(10.0);

                        if ui.button("Time").clicked() {
                            self.detail_page = Some(WeatherInfo::Time);
                            self.page = Page::DetailCurrent
                        }
                        ui.add_space(10.0);

                        if ui.button("All Info").clicked() {
                            self.detail_page = Some(WeatherInfo::AllInfo);
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
                            self.detail_page = Some(WeatherInfo::WindDirection);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("WeatherCode").clicked() {
                            self.detail_page = Some(WeatherInfo::WeatherCode);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("WindSpeed").clicked() {
                            self.detail_page = Some(WeatherInfo::WindSpeed);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("Temperature").clicked() {
                            self.detail_page = Some(WeatherInfo::Temperature);
                            self.page = Page::DetailHours
                        }

                        ui.add_space(10.0);

                        if ui.button("Time").clicked() {
                            self.detail_page = Some(WeatherInfo::Time);
                            self.page = Page::DetailHours
                        }
                        ui.add_space(10.0);

                        if ui.button("Rain").clicked() {
                            self.detail_page = Some(WeatherInfo::Rain);
                            self.page = Page::DetailHours
                        }
                        ui.add_space(10.0);

                        if ui.button("Humidity").clicked() {
                            self.detail_page = Some(WeatherInfo::Humidity);
                            self.page = Page::DetailHours
                        }
                    });
                }

                Page::DetailCurrent => {
                    // Detail Current Page
                    if ui.button("Back").clicked() {
                        self.page = Page::Current
                    }
                }
                
                Page::DetailHours => {
                    //Detail Hours Page
                    if ui.button("Back").clicked() {
                        self.page = Page::Hours
                    }
                }
            }
        });
    }
}
