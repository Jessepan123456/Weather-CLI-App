//Temp Detail
pub fn temp_weather(temp: f64, image: &mut String) -> String {
    if temp < 15.0 && temp > 4.0 {
        *image = "cold".to_string();
        "Feels a bit Chilly".to_string()
    } else if temp < 4.0 && temp >= -7.0 {
        if temp == 0.0 {
            *image = "cold".to_string();
            "Feels Cold-Freezing Point".to_string();
        }
        *image = "cold".to_string();
        "Feels Cold".to_string()
    } else if temp < -7.0 {
        *image = "cold".to_string();
        "It Freezing".to_string()
    } else if temp >= 21.0 && temp <= 26.0 {
        *image = "hot".to_string();
        "Nice Warm".to_string()
    } else if temp >= 27.0 && temp <= 32.0 {
        *image = "hot".to_string();
        "Kinda Hot".to_string()
    } else if temp > 32.0 {
        *image = "hot".to_string();
        "Very Hot".to_string()
    } else {
        "Feels Normal".to_string()
    }
}

//Rain Detail
pub fn rain_weather(rain: f64, image: &mut String) -> String {
    if rain == 0.0 {
        *image = "no_rain".to_string();
        "No Rain".to_string()
    } else if rain >= 0.1 && rain <= 1.0 {
        *image = "rain".to_string();
        "Light Rain".to_string()
    } else if rain > 1.0 && rain <= 4.0 {
        *image = "rain".to_string();
        "Steady Rain".to_string()
    } else if rain >= 5.0 && rain <= 10.0 {
        *image = "heavy_rain".to_string();
        "Heavy Rain".to_string()
    } else {
        *image = "heavy_rain".to_string();
        "Very Heavy Rain".to_string()
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
pub fn wind_speed_weather(wind: f64, image: &mut String) -> String {
    if wind >= 0.0 && wind < 1.0 {
        *image = "light_wind".to_string();
        "Calm Wind".to_string()
    } else if wind >= 1.0 && wind < 5.0 {
        *image = "light_wind".to_string();
        "Light breeze".to_string()
    } else if wind >= 5.0 && wind < 10.0 {
        *image = "wind".to_string();
        "It a bit windy".to_string()
    } else if wind >= 10.0 && wind < 15.0 {
        *image = "strong_wind".to_string();
        "Strong wind".to_string()
    } else if wind >= 15.0 && wind < 20.0 {
        *image = "strong_wind".to_string();
        "Very Strong wind".to_string()
    } else {
        "Dangerous wind".to_string()
    }
}

//Humidity Detail
pub fn humidity_weather(hum: f64) -> String {
    if hum >= 0.0 && hum < 30.0 {
        "Very dry air".to_string()
    } else if hum >= 30.0 && hum < 50.0 {
        "Comfortable air".to_string()
    } else if hum >= 50.0 && hum < 70.0 {
        "Humid air".to_string()
    } else if hum >= 70.0 && hum < 90.0 {
        "Very Humid air".to_string()
    } else {
        "Extremely humid air".to_string()
    }
}

//Time Detail
pub fn time_weather(time: &str, image: &mut String) -> String {
    let hour = &time[11..13];
    let hours: u32 = hour.parse().unwrap();
    if hours >= 5 && hours <= 11 {
        *image = "morning".to_string();
        "It Morning".to_string()
    } else if hours >= 12 && hours <= 17 {
        *image = "afternoon".to_string();
        "It Afternoon".to_string()
    } else if hours >= 18 && hours <= 21 {
        *image = "evening".to_string();
        "It Evening".to_string()
    } else {
        *image = "night".to_string();
        "It Night".to_string()
    }
}

//Weather Code
pub fn weather_code(code: f64) -> String {
    match code {
        0.0 => "Clear Sky".to_string(),
        1.0 | 2.0 => "Partly Cloudy".to_string(),
        3.0 => "Overcast".to_string(),
        45.0 | 48.0 => "Foggy".to_string(),
        51.0 | 53.0 | 55.0 => "Drizzle".to_string(),
        61.0 | 63.0 | 65.0 => "Rain".to_string(),
        71.0 | 73.0 | 75.0 | 77.0 => "Snow".to_string(),
        80.0 | 81.0 | 82.0 => "Rain Showers".to_string(),
        85.0 | 86.0 => "Snow Showers".to_string(),
        95.0 | 96.0 | 99.0 => "Thunderstorm".to_string(),
        _ => "Unknown Weather".to_string(),
    }
}
