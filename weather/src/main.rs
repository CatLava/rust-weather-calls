use std::{error::Error, vec};
use std::collections::HashMap;

use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};

mod location_map;


#[tokio::main]
async fn main()  {
    dotenv().ok();
    println!("Hello, world!");
    let test_loc = location_map::Location::new(None, Some("Oakland".to_string()), None, None, None, None);

    let query_string = location_map::create_query_string(test_loc);
    // TODO need to unwrap safely here
    let first_option = get_lat_long(&query_string).await
                                    .unwrap()
                                    .into_iter()
                                    .nth(0).
                                    unwrap();
    
    println!("{:?}", first_option.display_name);
    println!("{:?}", first_option.lat);
    println!("{:?}", first_option.lon);

    let weather  = get_weather_from_lat_long(&first_option.lat, &first_option.lon).await.unwrap();
    println!("{:?}", weather);
    println!("{:?}", weather.main);
    println!("{:?}", weather.main.temp_min);


    return
}



async fn get_lat_long(location: &str) -> Option<Vec<location_map::GeoApiFields>>{
    let geocode_api_token = std::env::var("GEOCODING_API_KEY").expect("GEOCODING_API_KEY must be set.");
    let mut url = "https://geocode.maps.co/search?".to_string();
    url.push_str(location);
    url.push_str(&format!("&api_key={}",geocode_api_token));
    println!("{:?}", url);
    // After string constructed need to make API call 
    let client = reqwest::Client::new();

    let body = client.get(url)
        .send()
        .await
        .unwrap();
        // .text()
        // .await;
    // Use below for troubleshooting the object return 
    //println!("{:?}", body.text().await);
    let resp: Option<Vec<location_map::GeoApiFields>> = match body.status() {
        reqwest::StatusCode::OK =>{
            match body.json::<Vec<location_map::GeoApiFields>>().await {
                Ok(body) => {
                    // println!("Body success {:?}", body);
                    Some(body)
                }
                _ => {
                    println!("unable to move forward");
                    None
                }
            }
        },
        // TODO need to move this into the success response 

        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };
    //let status = body.json::<Vec<location_map::GeoApiFields>>().await;
    return resp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    coord: HashMap<String, f32>,
    weather: Vec<WeatherBasic>,
    base: String, 
    main: WeatherMain,
    visibility: i32,
    wind: WeatherWind,
    rain: Option<String>,
    clouds: HashMap<String, i32>,
    dt: i64, // This is datetime, can convert
    sys: WeatherSys,
    timezone: i64,
    id: i64,
    name: String,
    code: Option<i32>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherBasic {
    id: i32,
    main: String,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherMain {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherWind {
    speed: f32,
    deg: i32,
    gust: Option<i8>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherSys {
    r#type: i8,
    id: i64,
    country: String,
    sunrise: i64, // Date string can convert
    sunset: i64
}
async fn get_weather_from_lat_long(lat: &str, lon: &str) -> Option<WeatherResponse> {
    let weather_api_token = std::env::var("WEATHER_API_KEY").expect("GEOCODING_API_KEY must be set.");
    // https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    let mut url = "https://api.openweathermap.org/data/2.5/weather?".to_string();
    // Todo can set temperate type into imperial 
    url.push_str(&format!("lat={}&lon={}&appid={}&units=imperial",lat, lon, weather_api_token));
    let client = reqwest::Client::new();

    let body = client.get(url)
        .send()
        .await
        .unwrap();
        // .text()
        // .await;
    println!("Weather");
    println!("{:?}", body);
    let resp =  match body.status() {
        reqwest::StatusCode::OK =>{
            match body.json::<WeatherResponse>().await {
                Ok(body) => {
                    println!("Body success {:?}", body);
                    Some(body)
                }
                _ => {
                    println!("unable to move forward");
                    None
                }
            }
        },
        // TODO need to move this into the success response 

        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };
    println!("{:?}", resp);
    return resp

}
