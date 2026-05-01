use colored::Colorize;

//Temp Detail
pub fn temp_weather(temp: f64) -> String {
    if temp < 15.0 && temp > 4.0 {
        "Feels a bit Chilly".cyan().to_string()
    } else if temp < 4.0 && temp >= -7.0 {
        if temp == 0.0 {
            "Feels Cold-Freezing Point".bright_blue().to_string();
        }
        "Feels Cold".blue().to_string()
    } else if temp < -7.0 {
        "It Freezing".purple().to_string()
    } else if temp >= 21.0 && temp <= 26.0 {
        "Nice Warm".yellow().to_string()
    } else if temp >= 27.0 && temp <= 32.0 {
        "Kinda Hot".bright_red().to_string()
    } else if temp > 32.0 {
        "Very Hot".red().to_string()
    } else {
        "Feels Normal".to_string()
    }
}

//Rain Detail
pub fn rain_weather(rain: f64) -> String {
    if rain == 0.0 {
        "No Rain".to_string()
    } else if rain >= 0.1 && rain <= 1.0 {
        "Light Rain".cyan().to_string()
    } else if rain > 1.0 && rain >= 4.0 {
        "Steady Rain".cyan().to_string()
    } else if rain >= 5.0 && rain <= 10.0 {
        "Heavy Rain".blue().to_string()
    } else {
        "Very Heavy Rain".blue().to_string()
    }
}

//Wind_Direction Detail
pub fn wind_direction_weather(wind: f64) -> String {
    if wind >= 0.0 && wind <= 89.0 {
        "The wind is coming from North".to_string()
    } else if wind >= 90.0 && wind <= 179.0 {
        "The wind is coming from East".to_string()
    } else if wind >= 180.0 && wind <= 269.0 {
        "The wind is coming from South".to_string()
    } else {
        "The wind is coming from West".to_string()
    }
}

//Wind_Speed Detail
pub fn wind_speed_weather(wind: f64) -> String {
    if wind >= 0.0 && wind < 1.0 {
        "Calm Wind".to_string()
    } else if wind >= 1.0 && wind < 5.0 {
        "Light breeze".to_string()
    } else if wind >= 5.0 && wind < 10.0 {
        "It a bit windy".to_string()
    } else if wind >= 10.0 && wind < 15.0 {
        "Strong wind".blue().to_string()
    } else if wind >= 15.0 && wind < 20.0 {
        "Very Strong wind".blue().to_string()
    } else {
        "The wind is coming from West".to_string()
    }
}

//Humidity Detail
pub fn humidity_weather(hum: f64) -> String {
    if hum >= 0.0 && hum < 30.0 {
        "Very dry air".red().to_string()
    } else if hum >= 30.0 && hum < 50.0 {
        "Comfortable air".to_string()
    } else if hum >= 50.0 && hum < 70.0 {
        "Humid air".to_string()
    } else if hum >= 70.0 && hum < 90.0 {
        "Very Humid air".to_string()
    } else {
        "Extremely humid air".red().to_string()
    }
}

//Time Detail
pub fn time_weather(time: &str) -> String {
    let hour = &time[11..13];
    let hours: u32 = hour.parse().unwrap();
    if hours >= 5 && hours <= 11 {
        "It Morning".yellow().to_string()
    } else if hours >= 12 && hours <= 17 {
        "It Afternoon".green().to_string()
    } else if hours >= 18 && hours <= 21 {
        "It Evening".cyan().to_string()
    } else {
        "It Night".blue().to_string()
    }
}
