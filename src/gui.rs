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

#[derive(PartialEq)]
enum Page {
    Home,
    Weather,
    History,
    Time,
    Hours,
    Current,
    Detail,
}

struct MyApp {
    page: Page,
    location: String,
    output: String,
    lat: f64,
    long: f64,
    history: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            page: Page::Home,
            location: String::new(),
            output: String::new(),
            lat: 0.0,
            long: 0.0,
            history: Vec::new(),
        }
    }
}

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
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("Day").clicked() {
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("SixHour").clicked() {
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("NextHour").clicked() {
                            self.page = Page::Hours
                        }

                        ui.add_space(10.0);

                        if ui.button("CurrentTime").clicked() {
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
                        // ui.label(egui::RichText::new(format!(
                        //     "Lat: {}, Long: {}",
                        //     self.lat.clone(),
                        //     self.long.clone())).size(25.0).strong()
                        // );

                        ui.add_space(10.0);

                        // Current-Time Choices
                        if ui.button("WindDirection").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("TimeZone").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("WeatherCode").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("WindSpeed").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("Temperature").clicked() {
                            self.page = Page::Detail
                        }
                        ui.add_space(10.0);

                        if ui.button("Time").clicked() {
                            self.page = Page::Detail
                        }
                        ui.add_space(10.0);

                        if ui.button("All Info").clicked() {
                            self.page = Page::Detail
                        }
                    });
                }

                Page::Hours => {
                    if ui.button("Back").clicked() {
                        self.page = Page::Time
                    }
                    // Hours-Time Menu
                    ui.vertical_centered(|ui| {
                        // ui.label(egui::RichText::new(format!(
                        //     "Lat: {}, Long: {}",
                        //     self.lat.clone(),
                        //     self.long.clone())).size(25.0).strong()
                        // );

                        ui.add_space(10.0);

                        // Hours-Time Choices
                        if ui.button("WindDirection").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("WeatherCode").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("WindSpeed").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("Temperature").clicked() {
                            self.page = Page::Detail
                        }

                        ui.add_space(10.0);

                        if ui.button("Time").clicked() {
                            self.page = Page::Detail
                        }
                        ui.add_space(10.0);

                        if ui.button("Rain").clicked() {
                            self.page = Page::Detail
                        }
                        ui.add_space(10.0);

                        if ui.button("Humidity").clicked() {
                            self.page = Page::Detail
                        }
                    });
                }

                Page::Detail => {
                    // Detail Page
                    if ui.button("Back").clicked() {
                        self.page = Page::Time
                    }
                }
            }
        });
    }
}

//         function::menu::MainPage::Show => {
//             println!("{}", "History:".cyan());
//             for i in 0..last_response.len() {
//                 println!("{}", last_response[i]);
//             }
//         }
//         function::menu::MainPage::Save => {
//             // [Save?] ←→ [Back]
//             let mut filename = String::new();
//             print!("Enter a filename with .txt: ");
//             io::stdout().flush().unwrap();
//             io::stdin().read_line(&mut filename).unwrap();
//             let name = filename.trim();

//             if !last_response.is_empty() {
//                 function::info::save_json(name, &last_response);
//                 println!("{}", "Saved".green());
//             } else {
//                 println!("{}", "There nothing to save".red());
//             }

//             // --- [Choose Time Range] ---
//             // Time-Range Menu
//             loop {
//                 let page = function::menu::time_range_menu(&mut choice);

//                 // Output based on the choice
//                 match page {
//                     function::menu::TimeRange::Back => {
//                         break;
//                     }
//                     function::menu::TimeRange::SpecificHour => {
//                         //URL
//                         let weather_url = function::info::hours_weather_url(lat, long);
//                         let response: Value =
//                             reqwest::blocking::get(weather_url).unwrap().json().unwrap();

//                         let mut first_index: i64;
//                         let mut last_index: i64;

//                         loop {
//                             let mut first = String::new();
//                             let mut last = String::new();
//                             print!("Enter your first range: ");
//                             io::stdout().flush().unwrap();
//                             io::stdin().read_line(&mut first).unwrap();
//                             print!("Enter your last range: ");
//                             io::stdout().flush().unwrap();
//                             io::stdin().read_line(&mut last).unwrap();

//                             first_index = match first.trim().parse() {
//                                 Ok(n) => n,
//                                 Err(_) => {
//                                     println!(
//                                         "{}",
//                                         "Invalid number for first hour. Try again.".red()
//                                     );
//                                     continue; // go back to the loop
//                                 }
//                             };

//                             last_index = match last.trim().parse() {
//                                 Ok(n) => n,
//                                 Err(_) => {
//                                     println!(
//                                         "{}",
//                                         "Invalid number for last hour. Try again.".red()
//                                     );
//                                     continue;
//                                 }
//                             };
//                             if first_index > last_index {
//                                 println!(
//                                     "{}",
//                                     "first hour is bigger than the last hour".yellow()
//                                 );
//                                 continue;
//                             } else {
//                                 break;
//                             }
//                         }

//                         // --- [Choose Weather Field] ---
//                         // Weather Field Menu
//                         loop {
//                             let page = function::menu::hours_weather_menu(&mut choice);

//                             if function::info::hours_weather_info(
//                                 page,
//                                 &response,
//                                 first_index + 1,
//                                 last_index + 1,
//                                 &mut last_response,
//                                 &location,
//                             ) == false
//                             {
//                                 break;
//                             }
//                         }
//                     }
//                     function::menu::TimeRange::Day => {
//                         //URL
//                         let weather_url = function::info::hours_weather_url(lat, long);
//                         let response: Value =
//                             reqwest::blocking::get(weather_url).unwrap().json().unwrap();

//                         // --- [Choose Weather Field] ---
//                         // Weather Field Menu
//                         loop {
//                             let page = function::menu::hours_weather_menu(&mut choice);

//                             if function::info::hours_weather_info(
//                                 page,
//                                 &response,
//                                 1,
//                                 25,
//                                 &mut last_response,
//                                 &location,
//                             ) == false
//                             {
//                                 break;
//                             }
//                         }
//                     }
//                     function::menu::TimeRange::SixHour => {
//                         //URL
//                         let weather_url = function::info::hours_weather_url(lat, long);
//                         let response: Value =
//                             reqwest::blocking::get(weather_url).unwrap().json().unwrap();

//                         // --- [Choose Weather Field] ---
//                         // Weather Field Menu
//                         loop {
//                             let page = function::menu::hours_weather_menu(&mut choice);

//                             if function::info::hours_weather_info(
//                                 page,
//                                 &response,
//                                 1,
//                                 7,
//                                 &mut last_response,
//                                 &location,
//                             ) == false
//                             {
//                                 break;
//                             }
//                         }
//                     }
//                     function::menu::TimeRange::NextHour => {
//                         //URL
//                         let weather_url = function::info::hours_weather_url(lat, long);
//                         let response: Value =
//                             reqwest::blocking::get(weather_url).unwrap().json().unwrap();

//                         // --- [Choose Weather Field] ---
//                         // Weather Field Menu
//                         loop {
//                             let page = function::menu::hours_weather_menu(&mut choice);

//                             if function::info::hours_weather_info(
//                                 page,
//                                 &response,
//                                 1,
//                                 2,
//                                 &mut last_response,
//                                 &location,
//                             ) == false
//                             {
//                                 break;
//                             }
//                         }
//                     }

//                     function::menu::TimeRange::CurrentTime => {
//                         //URL
//                         let weather_url = format!(
//                             "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
//                             lat, long
//                         );
//                         let response: Value =
//                             reqwest::blocking::get(weather_url).unwrap().json().unwrap();

//                         // --- [Choose Weather Field] ---
//                         // Weather Field Menu
//                         loop {
//                             let page = function::menu::current_weather_menu(&mut choice);

//                             // --- [Show Rsesult] ---
//                             // Output the info
//                             if function::info::current_weather_info(
//                                 page,
//                                 &response,
//                                 &mut last_response,
//                                 &location,
//                             ) == false
//                             {
//                                 break;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
