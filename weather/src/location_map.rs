use struct_iterable::Iterable;

use serde::{Deserialize, Serialize};

// This is as defined in the geocode API 
#[derive(Iterable, Serialize, Deserialize, Debug)]
pub struct Location {
    street: Option<String>,
    city: Option<String>,
    county: Option<String>,
    state: Option<String>,
    country: Option<String>,
    postalcode: Option<String>,
}

impl Location {
    pub fn new(street: Option<String>,
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
pub struct GeoApiFields {
    pub place_id: i32,
    pub licence: String,
    osm_type: String,
    osm_id: i32,
    boundingbox: Vec<String>,
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    class: String,
    r#type: String,
    importance: f32
}

// TODO this needs testing and tests written 
pub fn create_query_string(location: Location) -> String {
    // make this a list and join with & for the appropriate query 
    let mut query_string: Vec<String> = Vec::new();
    match location.street {
        Some(street) => query_string.push(format!("street={}", street.replace(" ", "+")).to_owned()),
        None => (),
    }
    match location.city {
        Some(city) => query_string.push(format!("city={}", city.replace(" ", "+")).to_owned()),
        None => (),
    }
    match location.county {
        Some(county) => query_string.push(format!("county={}", county.replace(" ", "+")).to_owned()),
        None => (),
    }
    match location.state {
        Some(state) => query_string.push(format!("state={}", state.replace(" ", "+")).to_owned()),
        None => (),
    }
    match location.postalcode {
        Some(postalcode) => query_string.push(format!("postalcode={}", postalcode.replace(" ", "+")).to_owned()),
        None => (),
    }
    let final_string = query_string.join("&");
    return final_string
    // TODO this will be a matching and string creation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_city() {
        assert_eq!(create_query_string(Location{
            street: None,
            city: Some("oakland".to_string()),
            county : None,
            state: None,
            country : None,
            postalcode : None,
        }), "city=oakland");
    }

    #[test]
    fn city_and_street() {
        assert_eq!(create_query_string(Location{
            street: Some("4122 Broadway".to_string()),
            city: Some("oakland".to_string()),
            county : None,
            state: None,
            country : None,
            postalcode : None,
        }), "street=4122+Broadway&city=oakland");
    }
}