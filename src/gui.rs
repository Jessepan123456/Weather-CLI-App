use eframe::egui;
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
pub enum Page {
    Home,
    Weather,
    History,
    Forecast,

    Time,
    Hours,
    Current,

    DetailCurrent,
    DetailHours,
}

// Variables Stored in MyApp
pub struct MyApp {
    pub page: Page,
    pub location: String,
    pub output: String,
    pub lat: f64,
    pub long: f64,
    pub history: Vec<String>,
    pub response: Value,
    pub first_hour: i64,
    pub specific_first: i64,
    pub specific_last: i64,
    pub last_hour: i64,
    pub time: String,
    pub save_name: String,

    pub image: String,

    pub display_forecast: bool,
    pub temp_max_points: Vec<[f64; 2]>,
    pub temp_min_points: Vec<[f64; 2]>,
    pub wind_data_points: Vec<[f64; 2]>,
    pub rain_data_points: Vec<[f64; 2]>,

    pub detail_page: Option<function::info::WeatherInfo>,
    pub detail_info: String,
    pub error: Option<String>,
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

            image: String::new(),

            display_forecast: false,
            temp_max_points: Vec::new(),
            temp_min_points: Vec::new(),
            wind_data_points: Vec::new(),
            rain_data_points: Vec::new(),

            detail_page: None,
            detail_info: String::new(),
            error: None,
        }
    }
}

//Main Update Frame for GUI
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
                if ui.button("Retry").clicked() {
                    self.error = None
                }
                return;
            }

            // Nav Bar
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.page = Page::Home;
                }

                if ui.button("History").clicked() {
                    self.page = Page::History;
                }

                if ui.button("Forecast").clicked() {
                    self.page = Page::Forecast;
                }

                if ui.button("Weather").clicked() {
                    self.page = Page::Weather;
                }
            });

            ui.separator();

            // Pages
            match self.page {
                // --- [Home] ---
                Page::Home => {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("About Page").size(30.0).strong());
                    });
                }

                // --- [History] ---
                Page::History => self.history_page(ui),

                // --- [7 Day Weather Forcast] ---
                Page::Forecast => self.forecast_page(ui),

                // --- [Weather] ---
                Page::Weather => {
                    // Weather Menu
                    self.weather_page(ui)
                }
                // --- [Choose Time Range] ---
                Page::Time => self.time_page(ui),

                Page::Current => self.current_page(ui),

                Page::Hours => self.hours_page(ui),

                Page::DetailCurrent => {
                    // Detail Current Page
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.image.clear();
                            self.page = Page::Current
                        }

                        if ui.button("Result").clicked() {
                            if let Some(page) = &self.detail_page {
                                let detail = function::info::current_weather_info(
                                    page,
                                    &self.response,
                                    &mut self.history,
                                    &self.location,
                                    &mut self.image,
                                );
                                self.detail_info = detail;
                            }
                        }
                    });

                    self.images_display(ui);
                }
                Page::DetailHours => {
                    //Detail Hours Page
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.page = Page::Hours
                        }
                        if ui.button("Result").clicked() {
                            if let Some(page) = &self.detail_page {
                                let detail = function::info::hours_weather_info(
                                    page,
                                    &self.response,
                                    self.first_hour,
                                    self.last_hour,
                                    &mut self.history,
                                    &self.location,
                                    &mut self.image,
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

//Helper Methods
impl MyApp {
    // Images Display Section
    fn images_display(&mut self, ui: &mut egui::Ui) {
        // Display Detail
        ui.label(format!("{}", &self.detail_info));
        let image_texture = match self.image.as_str() {
            "cold" => egui::include_image!("../assets/cold.png"),
            "hot" => egui::include_image!("../assets/hot.png"),
            "rain" => egui::include_image!("../assets/rain.png"),
            "no_rain" => egui::include_image!("../assets/no_rain.png"),
            "heavy_rain" => egui::include_image!("../assets/heavy_rain.png"),
            "light_wind" => egui::include_image!("../assets/light_wind.png"),
            "wind" => egui::include_image!("../assets/wind.png"),
            "strong_wind" => egui::include_image!("../assets/strong_wind.png"),
            "morning" => egui::include_image!("../assets/morning.png"),
            "afternoon" => egui::include_image!("../assets/afternoon.png"),
            "evening" => egui::include_image!("../assets/evening.png"),
            "night" => egui::include_image!("../assets/night.png"),
            _ => egui::include_image!("../assets/nothing.png"),
        };
        ui.add(egui::Image::new(image_texture).fit_to_exact_size(egui::vec2(150.0, 150.0)));
    }
}
