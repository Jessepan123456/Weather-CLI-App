# Weather Application
A weather App built with Rust that retrieves real-time weather data from an external API and displays it.

## Features
- Real-time Data
- Forecast Display with weekly data
- GUI built with egui/eframe
- Multiple weather detail pages, for current and hourly weather information
- Error handling safety for invalid requests and missing data
- History/Save for previous searches

## How to Run
1. Clone the repository/download
2. Navigate into the project folder
Example:
  git bash:
    cd Weather-app
4. Run application
   cargo run

## What I Learned
- Learn to structure large Rust Projects into multiple modules
- Working with external APIs
- Parsing JSON data
- GUI using egui/eframe
- Error handling without using unwrap()
- Managing states and user interaction

## Future Improvement
- Better images icon
- Add hourly forecast graphs
- Improve UI
- Implement async API requests/reduce the API calls
