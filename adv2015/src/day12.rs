pub fn get_answer(input: &str) -> i32 {
    let vec = extract_numbers(input);
    let answer = sum_numbers(&vec);

    answer
}

fn extract_numbers(input: &str) -> Vec<i32> {
    input.split_terminator(|c| {
        c == '[' || c == ']' || c == ',' || c == '"' || c == ':' || c == '{' || c == '}'
    })
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>())
        .filter(|p| p.is_ok())
        .map(|r| r.unwrap())
        .collect()
}

fn sum_numbers(input: &Vec<i32>) -> i32 {
    input.iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers_1() {
        let res = extract_numbers(r#"[1,2,3]"#);
        assert!(res.contains(&1));
        assert!(res.contains(&2));
        assert!(res.contains(&3));
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_extract_numbers_2() {
        let res = extract_numbers(r#"{"a":2,"b":4}"#);
        assert!(res.contains(&2));
        assert!(res.contains(&4));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_extract_numbers_3() {
        let res = extract_numbers(r#"[[[3]]]"#);
        assert!(res.contains(&3));
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_extract_numbers_4() {
        let res = extract_numbers(r#"{"a":{"b":4},"c":-1}"#);
        assert!(res.contains(&4));
        assert!(res.contains(&-1));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_extract_numbers_5() {
        let res = extract_numbers(r#"{"a":[-1,1]}"#);
        assert!(res.contains(&-1));
        assert!(res.contains(&1));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_extract_numbers_6() {
        let res = extract_numbers(r#"[-1,{"a":1}]"#);
        assert!(res.contains(&-1));
        assert!(res.contains(&1));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_extract_numbers_7() {
        let res = extract_numbers(r#"[]"#);
        assert!(res.is_empty());
    }

    #[test]
    fn test_extract_numbers_8() {
        let res = extract_numbers(r#"{}"#);
        assert!(res.is_empty());
    }

    #[test]
    fn test_sum_numbers_1() {
        let res = extract_numbers(r#"[1,2,3]"#);
        let sum = sum_numbers(&res);
        assert_eq!(sum, 6);
    }


    #[test]
    fn test_sum_numbers_2() {
        let res = extract_numbers(r#"[]"#);
        let sum = sum_numbers(&res);
        assert_eq!(sum, 0);
    }
}
