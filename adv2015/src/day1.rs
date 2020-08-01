pub fn count_brackets(s: &str) -> i16 {
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

pub fn count_position(s: &str) -> Option<i16> {
    let mut sum: i16 = 0;
    let mut counter: i16 = 1;

    for c in s.chars() {
        sum += if c == '(' {
            1
        } else if c == ')' {
            -1
        } else {
            return None;
        };

        if sum < 0 {
            return Some(counter);
        } else {
            counter += 1;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_brackets() {
        assert_eq!(count_brackets("((("), 3);
        assert_eq!(count_brackets(")))"), -3);
        assert_eq!(count_brackets("()(())"), 0);
        assert_eq!(count_brackets("((())"), 1);
        assert_eq!(count_brackets("(((a)b)c"), 1);
    }

    #[test]
    fn test_count_position() {
        assert_eq!(count_position(")"), Some(1));
        assert_eq!(count_position("()())"), Some(5));
    }

    #[test]
    fn test_count_unexpected_char_position() {
        assert_eq!(count_position(")"), Some(1));
        assert_eq!(count_position("()!!!())"), None);
    }

    #[test]
    fn test_count_wrong_position() {
        assert_eq!(count_position("("), None);
        assert_eq!(count_position("()()("), None);
    }
}

