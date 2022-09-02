use dotenv::dotenv;
use std::env::args;
extern crate openweathermap;
use openweathermap::blocking::weather;

fn main() {
    // Default city used if no args passed
    let default = String::from("Atlanta");

    let mut city = String::new();

    let mut arguments: Vec<String> = args().collect();
    arguments.remove(0);

    if arguments.len() == 0 {
        city = default;
    }

    for x in arguments {
        city.push_str(" ");
        city.push_str(&x);
    }

    fetch_weather(&city);
}

fn fetch_weather(city: &str) {
    // You can get an API key from https://openweathermap.org
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");

    match &weather(city, "imperial", "en", &api_key) {
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
