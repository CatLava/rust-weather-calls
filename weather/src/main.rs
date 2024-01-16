use struct_iterable::Iterable;
use dotenv::dotenv;
use reqwest;

#[tokio::main]
async fn main()  {
    dotenv().ok();
    println!("Hello, world!");
    let test_loc = Location::new(None, Some("Oakland".to_string()), None, None, None, None);

    let query_string = create_query_string(test_loc);
    let mut lnl = get_lat_long(&query_string).await;
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

async fn get_lat_long(location: &str) -> Result<String, reqwest::Error>{
    let geocode_api_token = std::env::var("GEOCODING_API_KEY").expect("GEOCODING_API_KEY must be set.");
    let mut url = "https://geocode.maps.co/search?".to_string();
    url.push_str(location);
    url.push_str(&format!("&api_key={}",geocode_api_token));
    println!("{:?}", url);
    // After string constructed need to make API call 
    let body = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await;
    println!("body = {:?}", body);
    return body
}



// From lat and long, get a weather call 
