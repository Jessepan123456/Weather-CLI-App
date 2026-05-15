use crate::Value;
use crate::gui::MyApp;
use chrono::{Datelike, NaiveDate};
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

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

                self.error_handled_forecast(&location_url);

                //Longitude and Latitude of the Location
                self.long = self.response["results"][0]["longitude"].as_f64().unwrap_or(0.0);
                self.lat = self.response["results"][0]["latitude"].as_f64().unwrap_or(0.0);

                let weather_url = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,rain_sum,windspeed_10m_max&timezone=auto",
                    self.lat,
                    self.long
                );
                self.error_handled_forecast(&weather_url);

                self.display_forecast = true;
            };

            ui.label(&self.output);
        });

        //Graph Section
        ui.heading("7 Day Forecast");
        ui.horizontal(|ui| {
            if self.display_forecast == true {
                self.build_forecast_info(ui);
            }
        });

        ui.add_space(20.0);

        self.plot_graph(ui);
    }

    //Info Forecast
    fn build_forecast_info(&mut self, ui: &mut egui::Ui) {
        self.temp_max_points.clear();
        self.temp_min_points.clear();
        self.rain_data_points.clear();
        self.wind_data_points.clear();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..7 {
                //Days
                let day = self.response["daily"]["time"][i].as_str().unwrap();

                let actual_day = NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap();

                let week = actual_day.weekday();

                //Max Temp Check
                let max_temp = self.response["daily"]["temperature_2m_max"][i].as_f64();

                let max_temp = match max_temp {
                    Some(v) => v,
                    None => {
                        self.error = Some("Missing temperature data".to_string());
                        continue;
                    }
                };

                //Min Temp Check
                let min_temp = self.response["daily"]["temperature_2m_min"][i].as_f64();

                let min_temp = match min_temp {
                    Some(v) => v,
                    None => {
                        self.error = Some("Missing temperature data".to_string());
                        continue;
                    }
                };

                //Rain Check
                let rain = self.response["daily"]["rain_sum"][i].as_f64();

                let rain = match rain {
                    Some(v) => v,
                    None => {
                        self.error = Some("Missing temperature data".to_string());
                        continue;
                    }
                };

                //Wind Check
                let wind = self.response["daily"]["windspeed_10m_max"][i].as_f64();

                let wind = match wind {
                    Some(v) => v,
                    None => {
                        self.error = Some("Missing temperature data".to_string());
                        continue;
                    }
                };

                // Vec For Graph
                self.temp_max_points.push([i as f64, max_temp]);
                self.temp_min_points.push([i as f64, min_temp]);
                self.rain_data_points.push([i as f64, rain]);
                self.wind_data_points.push([i as f64, wind]);

                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(150.0);
                        ui.set_max_width(150.0);

                        ui.label(format!("{} {}", day, week));

                        ui.label(format!("High: {}°C  Low: {}°C", max_temp, min_temp));

                        ui.label(format!("Rain: {} mm  Wind: {} km/h", rain, wind))
                    });
                });
            }
        });
    }

    //Plot Graph
    fn plot_graph(&mut self, ui: &mut egui::Ui) {
        let max_points: PlotPoints = PlotPoints::from(self.temp_max_points.clone());
        let min_points: PlotPoints = PlotPoints::from(self.temp_min_points.clone());
        let rain_points: PlotPoints = PlotPoints::from(self.rain_data_points.clone());
        let wind_points: PlotPoints = PlotPoints::from(self.wind_data_points.clone());

        let max_line = Line::new(max_points).name("Max");
        let min_line = Line::new(min_points).name("Min");
        let rain_line = Line::new(rain_points).name("rain");
        let wind_line = Line::new(wind_points).name("wind");

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("Temperature Graph");

                Plot::new("temp_plot")
                    .height(120.0)
                    .width(250.0)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show_axes([false, false])
                    .show_grid(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(max_line);
                        plot_ui.line(min_line);
                    });

                ui.label("Rain Graph");

                Plot::new("rain_plot")
                    .height(120.0)
                    .width(250.0)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show_axes([false, false])
                    .show_grid(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(rain_line);
                    });

                ui.label("Wind Graph");

                Plot::new("wind_plot")
                    .height(120.0)
                    .width(250.0)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show_axes([false, false])
                    .show_grid(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(wind_line);
                    });
            });
        });
    }

    fn error_handled_forecast(&mut self, url: &String) {
        //Error Handled for Response
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
