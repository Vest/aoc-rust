use std::env;
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

pub enum HttpError {
    WrongHeader(InvalidHeaderValue),
    WrongClient(reqwest::Error),
}

impl From<InvalidHeaderValue> for HttpError {
    fn from(err: InvalidHeaderValue) -> HttpError {
        HttpError::WrongHeader(err)
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> HttpError {
        HttpError::WrongClient(err)
    }
}

pub fn get_input(day: u8, year: u16, session: &str) -> Result<String, HttpError> {
    let cookie = format!("session={}", session);
    let header_value = HeaderValue::from_str(cookie.as_str())?;
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::COOKIE, header_value);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let body = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day).as_str())
        .send()?
        .text()?;

    Ok(body)
}

fn main() {
    println!("Advent 2015!");
    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            let session = session_value.as_str();

            for day in 1..26 {
                match get_input(day, 2015, session) {
                    Ok(input) => adv2015::print_answers(day, input),
                    Err(_) => eprintln!("Couldn't get Input value for day {} / 2015.", day),
                }
            }
        }
        Err(e) => println!("Couldn't get {} key from environment variable. Description: {}", key, e),
    }

    println!("Done");
}
