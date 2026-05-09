use colored::Colorize;
use std::io;
use std::io::Write;

//Main
pub enum MainPage {
    WeatherLocation,
    Save,
    Show,
    Clear,
    Exit,
}

//Time
pub enum TimeRange {
    CurrentTime,
    NextHour,
    SixHour,
    Day,
    SpecificHour,
    Back,
}

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
    Back,
}

// Main Menu
pub fn main_menu(choice: &mut String) -> MainPage {
    loop {
        // Main Menu
        println!("4) Weather Location");
        println!("3) Save Data History");
        println!("2) Show History");
        println!("1) Clear History");
        println!("0) Exit");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(choice).unwrap();

        //Number choice
        let page = match choice.trim().parse::<u32>() {
            Ok(0) => MainPage::Exit,
            Ok(1) => MainPage::Clear,
            Ok(2) => MainPage::Show,
            Ok(3) => MainPage::Save,
            Ok(4) => MainPage::WeatherLocation,
            _ => {
                println!("{}", "Try again invalid input".red());
                continue;
            }
        };
        return page;
    }
}

// Time-Range Menu
pub fn time_range_menu(choice: &mut String) -> TimeRange {
    loop {
        println!("5) Current Weather");
        println!("4) Next Hour Weather");
        println!("3) Next 6 Hour Weather");
        println!("2) Next 24 Hour Weather");
        println!("1) Choose Your Own Hour");
        println!("0) Back");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(choice).unwrap();

        //Number choice
        let page = match choice.trim().parse::<u32>() {
            Ok(0) => TimeRange::Back,
            Ok(1) => TimeRange::SpecificHour,
            Ok(2) => TimeRange::Day,
            Ok(3) => TimeRange::SixHour,
            Ok(4) => TimeRange::NextHour,
            Ok(5) => TimeRange::CurrentTime,
            _ => {
                println!("{}", "Try again invalid input".red());
                continue;
            }
        };
        return page;
    }
}

// Hours Weather Menu
pub fn hours_weather_menu(choice: &mut String) -> WeatherInfo {
    loop {
        println!("7) Humidity");
        println!("6) Rain");
        println!("5) Time");
        println!("4) Temperature");
        println!("3) WindSpeed");
        println!("2) WeatherCode");
        println!("1) WindDirection");
        println!("0) Back");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(choice).unwrap();

        //Number choice
        let page = match choice.trim().parse::<u32>() {
            Ok(0) => WeatherInfo::Back,
            Ok(1) => WeatherInfo::WindDirection,
            Ok(2) => WeatherInfo::WeatherCode,
            Ok(3) => WeatherInfo::WindSpeed,
            Ok(4) => WeatherInfo::Temperature,
            Ok(5) => WeatherInfo::Time,
            Ok(6) => WeatherInfo::Rain,
            Ok(7) => WeatherInfo::Humidity,
            _ => {
                println!("{}", "Try again invalid input".red());
                continue;
            }
        };
        return page;
    }
}

//Current Weather Menu
pub fn current_weather_menu(choice: &mut String) -> WeatherInfo {
    loop {
        println!("7) All Info");
        println!("6) Time");
        println!("5) Temperature");
        println!("4) WindSpeed");
        println!("3) WeatherCode");
        println!("2) TimeZone");
        println!("1) WindDirection");
        println!("0) Back");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(choice).unwrap();

        //Number choice
        let page = match choice.trim().parse::<u32>() {
            Ok(0) => WeatherInfo::Back,
            Ok(1) => WeatherInfo::WindDirection,
            Ok(2) => WeatherInfo::TimeZone,
            Ok(3) => WeatherInfo::WeatherCode,
            Ok(4) => WeatherInfo::WindSpeed,
            Ok(5) => WeatherInfo::Temperature,
            Ok(6) => WeatherInfo::Time,
            Ok(7) => WeatherInfo::AllInfo,
            _ => {
                println!("{}", "Try again invalid input".red());
                continue;
            }
        };
        return page;
    }
}

// // --- [Choose City] ---
// println!("Welcome to the Weather info");
// let mut choice = String::new();
// let mut last_response: Vec<String> = Vec::new();

// loop {
//     let page: function::menu::MainPage = function::menu::main_menu(&mut choice);

//     //Output of that page
//     match page {
//         function::menu::MainPage::Exit => {
//             println!("{}", "Exiting...".red());
//             process::exit(0);
//         }
//         function::menu::MainPage::Clear => {
//             last_response.clear();
//             println!("{}", "History was cleared".cyan());
//         }
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
//         }
//         function::menu::MainPage::WeatherLocation => {
//             // Input Location
//             let mut location = String::new();
//             print!("Enter a location(EX: Salt Lake City): ");
//             io::stdout().flush().unwrap();
//             io::stdin().read_line(&mut location).unwrap();

//             //Url For the location
//             let location_url = format!(
//                 "https://geocoding-api.open-meteo.com/v1/search?name={}",
//                 location
//             );
//             let response: Value = reqwest::blocking::get(location_url)
//                 .unwrap()
//                 .json()
//                 .unwrap();

//             //Longitude and Latitude of the Location
//             let long = response["results"][0]["longitude"].as_f64().unwrap();
//             let lat = response["results"][0]["latitude"].as_f64().unwrap();

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