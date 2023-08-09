use std::collections::HashMap;
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CurrentWeatherResponse {
    coord: Coordinates,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u32,
    sys: Sys,
    timezone: u32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize)]
struct Coordinates {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: u32,
}

#[derive(Debug, Deserialize)]
struct Sys {
    r#type: u32,
    id: u32,
    country: String,
    sunrise: u32,
    sunset: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No API key provided".into());
    }

    let api_key = &args[1];
    println!("Using Open Weather Map API key: {}", api_key);

    let latitude = 35.689501375244;
    let longitude = 139.69173371705;

    let mut query = HashMap::new();
    query.insert("lat", latitude.to_string());
    query.insert("lon", longitude.to_string());
    query.insert("appid", api_key.to_string());

    let client = reqwest::Client::new();

    let resp = client.get("https://api.openweathermap.org/data/2.5/weather")
        .query(&query)
        .send()
        .await?;

    let resp: CurrentWeatherResponse = resp.json().await?;

    println!("{:#?}", resp);

    return Ok(());
}
