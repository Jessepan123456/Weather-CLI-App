use serde_json::Value;

mod function;
mod gui;
mod pages;

fn main() -> eframe::Result<()> {
    gui::run_weather_app()
}
