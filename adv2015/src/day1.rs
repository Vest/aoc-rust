use reqwest::header::{HeaderMap, HeaderValue};

pub fn get_answer(session: String) -> i16{
    let body = get_input(session);
    let count = count_brackets(body);

    count
}

fn count_brackets(s: String) -> i16 {
    let chars = s.chars();
    let floor = chars.fold(0, |acc, c| {
        if c == '(' {
            acc + 1
        } else if c == ')' {
            acc - 1
        } else {
            acc
        }
    });

    floor
}

fn get_input(session: String) -> String {
    let cookie = format!("session={}", session);
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::COOKIE,
                   HeaderValue::from_str(cookie.as_str()).unwrap());


    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build();

    if let Err(e) = client {
        eprintln!("Couldn't build the client: {}", e);
        return String::from("");
    }

    let client = client.unwrap();


    let body = client.get("https://adventofcode.com/2015/day/1/input")
        .send()
        .unwrap()
        .text()
        .unwrap();

    String::from(body)
}

#[cfg(test)]
mod tests {
    use crate::day1::count_brackets;
    use crate::day1::get_input;

    #[test]
    fn test_count_brackets() {
        assert_eq!(count_brackets("((("), 3);
        assert_eq!(count_brackets(")))"), -3);
        assert_eq!(count_brackets("()(())"), 0);
        assert_eq!(count_brackets("((())"), 1);
        assert_eq!(count_brackets("(((a)b)c"), 1);
    }

    #[test]
    fn test_get() {
        assert_ne!(get_input().len(), 0, "Couldn't send GET as a HTTP request");
    }
}

