const ITERATION_COUNT_FIRST: usize = 40;
const ITERATION_COUNT_SECOND: usize = 50;

pub fn calc_first(input: &str) -> usize {
    calc_length(input, ITERATION_COUNT_FIRST)
}

pub fn calc_second(input: &str) -> usize {
    calc_length(input, ITERATION_COUNT_SECOND)
}

fn calc_length(input: &str, iterations: usize) -> usize {
    let mut look_and_say = String::from(input);
    for _ in 0..iterations {
        look_and_say = convert_to_look(&look_and_say);
    }

    look_and_say.chars().count()
}

fn convert_to_look(input: &String) -> String {
    let mut result = String::new();
    let mut current_char: Option<char> = None;
    let mut count = 0;

    for c in input.chars() {
        if current_char.is_none() {
            current_char = Some(c);
            count = 1;
            continue;
        }

        if current_char.unwrap() == c {
            count += 1;
        } else {
            result.push_str(format!("{}{}", count, current_char.unwrap()).as_str());
            count = 1;
            current_char = Some(c);
        }
    }

    if let Some(c) = current_char {
        result.push_str(format!("{}{}", count, c).as_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_look() {
        assert_eq!(convert_to_look(&"1".to_string()), "11");
        assert_eq!(convert_to_look(&"11".to_string()), "21");
        assert_eq!(convert_to_look(&"21".to_string()), "1211");
        assert_eq!(convert_to_look(&"1211".to_string()), "111221");
        assert_eq!(convert_to_look(&"111221".to_string()), "312211");
        assert_eq!(convert_to_look(&"3113322113".to_string()), "132123222113");
    }

    #[test]
    fn test_calc_length() {
        assert_eq!(calc_length(&"1".to_string(), 1), 2, "Length of 11 is 2");
        assert_eq!(calc_length(&"1".to_string(), 2), 2, "Length of 21 is 2");
        assert_eq!(calc_length(&"1".to_string(), 3), 4, "Length of 1211 is 4");
        assert_eq!(calc_length(&"1".to_string(), 4), 6, "Length of 111221 is 6");
        assert_eq!(calc_length(&"1".to_string(), 5), 6, "Length of 312211 is 6");
    }
}

