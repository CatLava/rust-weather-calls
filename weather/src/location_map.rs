use struct_iterable::Iterable;
use std::error::Error;

use dotenv::dotenv;
use reqwest;
use serde_json;
use serde::{Deserialize, Serialize};

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

// TODO this needs testing and tests written 
pub fn create_query_string(location: Location) -> String {
    // make this a list and join with & for the appropriate query 
    let mut qs: String = "".to_string();
    let mut query_string = Vec::new()
    match location.city {
        Some(city) => query_string.push(&format!("city={}", city)),
        None => (),
    }
    match location.street {
        Some(street) => query_string.push("hello"),
        None => (),
    }
    println!("{:?}", qs);
    return qs
    // TODO this will be a matching and string creation
}