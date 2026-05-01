use std::fs::File;
use std::io::Write;

use crate::Value;
use crate::function::detail;
use crate::function::menu;

//Hour Weather Info
pub fn hours_weather_info(
    page: menu::WeatherInfo,
    response: &Value,
    firsthour: i64,
    lasthour: i64,
    last_response: &mut Vec<String>,
    location: &String,
) -> bool {
    // indspeed_10m,weathercode,relativehumidity_2m,rain
    match page {
        menu::WeatherInfo::Back => false,
        menu::WeatherInfo::WindDirection => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["winddirection_10m"][i as usize]
                    .as_f64()
                    .unwrap();
                let detail = detail::wind_direction_weather(info);
                let wind_direction = format!(
                    "{}-Hour {} : WindDirection:{}, {}",
                    i,
                    location.trim(),
                    info,
                    detail
                );

                println!("{}", wind_direction);
                last_response.push(wind_direction);
            }
            true
        }
        menu::WeatherInfo::TimeZone => false,
        menu::WeatherInfo::WeatherCode => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["weathercode"][i as usize]
                    .as_f64()
                    .unwrap();
                let weather_code = format!("{}-Hour {} : WeatherCode:{}", i, location.trim(), info);

                println!("{}", weather_code);
                last_response.push(weather_code);
            }
            true
        }
        menu::WeatherInfo::WindSpeed => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["windspeed_10m"][i as usize]
                    .as_f64()
                    .unwrap();
                let detail = detail::wind_speed_weather(info);
                let wind_speed = format!(
                    "{}-Hour {} : WindSpeed:{}, {}",
                    i,
                    location.trim(),
                    info,
                    detail
                );

                println!("{}", wind_speed);
                last_response.push(wind_speed);
            }
            true
        }
        menu::WeatherInfo::Temperature => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["temperature_2m"][i as usize]
                    .as_f64()
                    .unwrap();
                let detail = detail::temp_weather(info);
                let temp = format!(
                    "{}-Hour {} : Temperature:{}, {}",
                    i,
                    location.trim(),
                    info,
                    detail
                );

                println!("{}", temp);
                last_response.push(temp);
            }
            true
        }
        menu::WeatherInfo::Time => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["time"][i as usize].as_str().unwrap();
                let detail = detail::time_weather(info);
                let time = format!("{}-Hour {} : Time:{}, {}", i, location.trim(), info, detail);

                println!("{}", time);
                last_response.push(time);
            }
            true
        }
        menu::WeatherInfo::Humidity => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["relativehumidity_2m"][i as usize]
                    .as_f64()
                    .unwrap();
                let detail = detail::humidity_weather(info);
                let humidity = format!(
                    "{}-Hour {} : Humidity:{}, {}",
                    i,
                    location.trim(),
                    info,
                    detail
                );

                println!("{}", humidity);
                last_response.push(humidity);
            }
            true
        }
        menu::WeatherInfo::Rain => {
            for i in firsthour..lasthour {
                let info = response["hourly"]["rain"][i as usize].as_f64().unwrap();
                let detail = detail::rain_weather(info);
                let rain = format!("{}-Hour {} : Rain:{}, {}", i, location.trim(), info, detail);

                println!("{}", rain);
                last_response.push(rain);
            }
            true
        }
        menu::WeatherInfo::AllInfo => false,
    }
}

//Current Weather info
pub fn current_weather_info(
    page: menu::WeatherInfo,
    response: &Value,
    last_response: &mut Vec<String>,
    location: &String,
) -> bool {
    match page {
        menu::WeatherInfo::Back => false,
        menu::WeatherInfo::WindDirection => {
            let info = response["current_weather"]["winddirection"]
                .as_f64()
                .unwrap();
            let detail = detail::wind_direction_weather(info);
            let wind_direction =
                format!("{} : WindDirection:{}, {}", location.trim(), info, detail);

            println!("{}", wind_direction);
            last_response.push(wind_direction);
            true
        }
        menu::WeatherInfo::TimeZone => {
            let info = response["timezone"].as_str().unwrap();
            let time_zone = format!("{} : TimeZone:{}", location.trim(), info);

            println!("{}", time_zone);
            last_response.push(time_zone);
            true
        }
        menu::WeatherInfo::WeatherCode => {
            let info = response["current_weather"]["weathercode"].as_f64().unwrap();
            let weather_code = format!("{} : WeatherCode:{}", location.trim(), info);

            println!("{}", weather_code);
            last_response.push(weather_code);
            true
        }
        menu::WeatherInfo::WindSpeed => {
            let info = response["current_weather"]["windspeed"].as_f64().unwrap();
            let detail = detail::wind_speed_weather(info);
            let wind_speed = format!("{} : WindSpeed:{}, {}", location.trim(), info, detail);

            println!("{}", wind_speed);
            last_response.push(wind_speed);
            true
        }
        menu::WeatherInfo::Temperature => {
            let info = response["current_weather"]["temperature"].as_f64().unwrap();
            let detail = detail::temp_weather(info);
            let temp = format!("{} : Temperature:{}, {}", location.trim(), info, detail);

            println!("{}", temp);
            last_response.push(temp);
            true
        }
        menu::WeatherInfo::Time => {
            let info = response["current_weather"]["time"].as_str().unwrap();
            let detail = detail::time_weather(info);
            let time = format!("{} : Time:{}, {}", location.trim(), info, detail);

            println!("{}", time);
            last_response.push(time);
            true
        }
        menu::WeatherInfo::Humidity => false,
        menu::WeatherInfo::Rain => false,
        menu::WeatherInfo::AllInfo => {
            let wind_d = response["current_weather"]["winddirection"]
                .as_f64()
                .unwrap();
            let detail_wind_d = detail::wind_direction_weather(wind_d);

            let time = response["current_weather"]["time"].as_str().unwrap();
            let detail_time = detail::time_weather(time);

            let temp = response["current_weather"]["temperature"].as_f64().unwrap();
            let detail_temp = detail::temp_weather(temp);

            let wind_s = response["current_weather"]["windspeed"].as_f64().unwrap();
            let detail_wind_s = detail::wind_speed_weather(wind_s);

            let code = response["current_weather"]["weathercode"].as_f64().unwrap();
            let timezone = response["timezone"].as_str().unwrap();

            let info = format!(
                "{} - TimeZone {} - The time right is {}, {}. {} - {}, The temp is {} - {}. Weather Code is {}",
                location.trim(),
                timezone,
                time,
                detail_time,
                detail_wind_s,
                detail_wind_d,
                temp,
                detail_temp,
                code
            );

            println!("{}", info);
            last_response.push(info);
            true
        }
    }
}

//URL for Hours
pub fn hours_weather_url(lat: f64, long: f64) -> String {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,windspeed_10m,weathercode,winddirection_10m,relativehumidity_2m,rain",
        lat, long
    );
    return url;
}

//Saving Data
pub fn save_json(filename: &str, response: &Vec<String>) {
    let mut file = File::create(filename).unwrap();
    let encoded = serde_json::to_string(response).unwrap();
    file.write_all(encoded.as_bytes()).unwrap();
}
