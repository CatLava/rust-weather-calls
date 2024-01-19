use struct_iterable::Iterable;
use std::error::Error;

use dotenv::dotenv;
use reqwest;
use serde_json;
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main()  {
    dotenv().ok();
    println!("Hello, world!");
    let test_loc = Location::new(None, Some("Oakland".to_string()), None, None, None, None);

    let query_string = create_query_string(test_loc);
    // TODO need to unwrap safely here
    let mut lnl = get_lat_long(&query_string).await.unwrap();
    let first_loc = lnl.into_iter().nth(0).unwrap();
    
    println!("{:?}", first_loc.display_name);
    println!("{:?}", first_loc.lat);
    println!("{:?}", first_loc.lon);


    return
}

// This is as defined in the geocode API 
#[derive(Iterable)]
pub struct Location {
    street: Option<String>,
    city: Option<String>,
    county: Option<String>,
    state: Option<String>,
    country: Option<String>,
    postalcode: Option<String>,
}

impl Location {
    fn new(street: Option<String>,
        city: Option<String>,
        county: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postalcode: Option<String>) -> Location {
            Location {
                street: street,
                city: city,
                county: county,
                state: state,
                country: country,
                postalcode: postalcode
            }

    }

}

#[derive(Serialize, Deserialize, Debug)]
struct GeoApiFields {
    place_id: i32,
    licence: String,
    osm_type: String,
    osm_id: i32,
    boundingbox: Vec<String>,
    lat: String,
    lon: String,
    display_name: String,
    class: String,
    r#type: String,
    importance: f32
}

pub fn create_query_string(location: Location) -> String {
    // make this a list and join with & for the appropriate query 
    let mut qs: String = "".to_string();
    match location.city {
        Some(city) => qs.push_str(&format!("city={}", city)),
        None => (),
    }
    match location.street {
        Some(street) => qs.push_str("hello"),
        None => (),
    }
    println!("{:?}", qs);
    return qs
    // TODO this will be a matching and string creation
}

async fn get_lat_long(location: &str) -> Result<Vec<GeoApiFields>, reqwest::Error>{
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
    // Use below for troubleshooting th eobject return 
    //println!("{:?}", body.text().await);
    match body.status() {
        reqwest::StatusCode::OK => {println!("Success!")},
        // TODO need to move this into the success response 
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    }
    let b2 = body.json::<Vec<GeoApiFields>>().await;
    // TODO match on status code
    // https://blog.logrocket.com/making-http-requests-rust-reqwest/
    // println!("body = {:?}", b2);
    return b2
}

async fn get_weather_from_lat_long(lat: f32, lon: f32){
    let weather_api_token = std::env::var("WEATHER_API_KEY").expect("GEOCODING_API_KEY must be set.");
    // https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    let mut url = "https://api.openweathermap.org/data/2.5/weather?".to_string();
}



// From lat and long, get a weather call 
