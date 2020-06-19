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

pub fn count_position(s: &str) -> i16 {
    let mut sum: i16 = 0;
    let mut counter: i16 = 1;

    for c in s.chars() {
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

#[cfg(test)]
mod tests {
    use crate::day1::{count_brackets, count_position};

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
        assert_eq!(count_position(")"), 1);
        assert_eq!(count_position("()())"), 5);
    }
}

