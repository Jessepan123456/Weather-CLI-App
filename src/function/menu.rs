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
