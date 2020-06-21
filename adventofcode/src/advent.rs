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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input(1, 2015, "");
        assert!(input.is_ok(), "The input response is not successful");

        let response = input.unwrap_or(String::from("fail"));
        assert!(response.contains("Please log in"), "The response is unexpected: {}", response);
    }
}

