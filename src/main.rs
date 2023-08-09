mod models;

use std::collections::HashMap;
use std::env;

use crate::models::open_weather_map::current_weather::CurrentWeatherResponse;

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
