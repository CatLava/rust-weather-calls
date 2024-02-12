# rust-weather-calls


## Purpose:
This is a command line tool written in rust that allows the user to make get weather data from entering a geographical 
location such as a city. 
In order to configure this, the below websites API keys are used, along with the response syntax that was converted to 
structs
Using this for the geocoding API : https://geocode.maps.co/

Used for weather data : https://openweathermap.org/current#one

## Usage
cargo run -- -city Denver -state Colorado
used to help with reqwest syntax : https://blog.logrocket.com/making-http-requests-rust-reqwest/
