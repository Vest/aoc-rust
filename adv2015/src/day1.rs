use reqwest::header::{HeaderMap, HeaderValue};

pub fn get_answer1(session: String) -> i16 {
    let body = get_input(session);
    let count = count_brackets(body);

    count
}

pub fn get_answer2(session: String) -> i16 {
    let body = get_input(session);
    let position = count_position(body);
    position
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

fn count_position(s: String) -> i16 {
    let mut sum: i16 = 0;
    let mut counter: i16 = 1;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        sum += if c == '(' {
            1
        } else if c == ')' {
            -1
        } else {
            0
        };

        if sum < 0 {
            return counter;
        } else {
            counter += 1;
        }
    }

    return 0;
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
    use crate::day1::{count_brackets, count_position};

    #[test]
    fn test_count_brackets() {
        assert_eq!(count_brackets(String::from("(((")), 3);
        assert_eq!(count_brackets(String::from(")))")), -3);
        assert_eq!(count_brackets(String::from("()(())")), 0);
        assert_eq!(count_brackets(String::from("((())")), 1);
        assert_eq!(count_brackets(String::from("(((a)b)c")), 1);
    }

    #[test]
    fn test_count_position() {
        assert_eq!(count_position(String::from(")")), 1);
        assert_eq!(count_position(String::from("()())")), 5);
    }
}

