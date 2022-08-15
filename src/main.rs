use dotenv::dotenv;
use std::env::args;
extern crate openweathermap;
use openweathermap::blocking::weather;

fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");

    let city: String;

    match args().len() {
        2 => city = args().nth(1).unwrap().to_string(),
        3 => city = args().nth(1).unwrap().to_string() + " " + &args().nth(2).unwrap().to_string(),
        4 => {
            city = args().nth(1).unwrap().to_string()
                + " "
                + &args().nth(2).unwrap().to_string()
                + " "
                + &args().nth(3).unwrap().to_string()
        }
        _ => city = "Atlanta".to_string(),
    }

    // You can get an API key from https://openweathermap.org
    match &weather(&city, "imperial", "en", &api_key) {
        Ok(current) => {
            println!(
                "It is currently {} degrees Fahrenheit in {}!",
                current.main.temp.round(),
                current.name.as_str()
            );
        }

        Err(..) => println!("Error fetching weather. Please pass a city name as an argument."),
    }
}
