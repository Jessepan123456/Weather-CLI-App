use serde_json::Value;

mod function;
mod gui;

fn main() -> eframe::Result<()> {
    gui::run_weather_app()
}
