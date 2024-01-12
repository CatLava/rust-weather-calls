use struct_iterable::Iterable;

fn main()  {
    println!("Hello, world!");
    let test_loc = Location::new(None, Some("Oakland".to_string()), None, None, None, None);

    create_query_string(test_loc);
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

fn get_lat_long(location: &str) {
    let url = "https://geocode.maps.co/search?";
    // Need to get API keys and what not 

}



// From lat and long, get a weather call 
