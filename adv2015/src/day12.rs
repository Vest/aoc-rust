pub fn get_answer(input: &str) -> i32 {
    let vec = extract_numbers(input);
    let answer = sum_numbers(&vec);

    answer
}

pub fn get_answer_without_red(input: &str) -> i32 {
    let answer = scan_deep(input);

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

fn find_first_json_object(input: &str) -> &str {
    let mut level = 0usize;
    let mut range = (0usize, 0usize);
    for (i, c) in input.chars().enumerate() {
        if c == '{' {
            if level == 0 {
                range.0 = i;
            }
            level += 1;
        } else if c == '}' {
            level -= 1;

            if level == 0 {
                range.1 = i;
                break;
            }
        }
    }
    if range.0 == range.1 {
        return "";
    }

    &input[range.0..range.1 + 1]
}

fn scan_deep(input: &str) -> i32 {
    let mut str = String::new();
    let res = find_first_json_object(input);

    if res == "" {
        return if input.contains(r#":"red""#) { 0 } else { get_answer(input) };
    }

    if let Some(i) = input.find(res) {
        str.push_str(&input[0..i]);
        str.push_str(format!("{}", scan_deep(&res[1..res.len() - 1])).as_str());
        str.push_str(&input[i + res.len()..]);

        return scan_deep(str.as_str());
    }

    0
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

    #[test]
    fn test_find_json_object() {
        assert_eq!(find_first_json_object(r#"[1,{"c":"red","b":2},3]"#), r#"{"c":"red","b":2}"#);
        assert_eq!(find_first_json_object(r#"[1,{"c":"red",{"a":1},"b":2},3]"#), r#"{"c":"red",{"a":1},"b":2}"#);
        assert_eq!(find_first_json_object(r#"[1,3]"#), r#""#);
        assert_eq!(find_first_json_object(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), r#"{"d":"red","e":[1,2,3,4],"f":5}"#);
    }

    #[test]
    fn test_scan_deep() {
        assert_eq!(scan_deep(r#"[1,{"c":"red",{"a":1},"b":2},3]"#), 4);
        assert_eq!(scan_deep(r#"[1,2,3]"#), 6);
        assert_eq!(scan_deep(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(scan_deep(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(scan_deep(r#"[1,"red",5]"#), 6);
    }
}
