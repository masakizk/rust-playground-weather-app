use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CurrentWeatherResponse {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u32,
    pub sys: Sys,
    pub timezone: u32,
    pub id: u32,
    pub name: String,
    pub cod: u32,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u32,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    r#type: u32,
    id: u32,
    country: String,
    sunrise: u32,
    sunset: u32,
}
