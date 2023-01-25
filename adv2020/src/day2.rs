pub fn count_simple_passwords(input: &str) -> usize {
    parse_input(input).filter(is_password_simple).count()
}

pub fn count_complex_passwords(input: &str) -> usize {
    parse_input(input).filter(is_password_complex).count()
}

fn is_password_simple(rule: &Rule) -> bool {
    let actual_count = rule.password.chars().filter(|c| *c == rule.letter).count();

    actual_count >= rule.from && actual_count <= rule.to
}

fn is_password_complex(rule: &Rule) -> bool {
    let actual_count: Vec<char> = rule.password.chars().collect();

    if rule.from > actual_count.len() || rule.to > actual_count.len() {
        return false;
    }

    let letter_from = actual_count[rule.from - 1];
    let letter_to = actual_count[rule.to - 1];

    if letter_from == rule.letter || letter_to == rule.letter {
        letter_from != letter_to
    } else {
        false
    }
}

struct Rule {
    from: usize,
    to: usize,
    letter: char,
    password: String,
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Rule> + 'a {
    input.lines().filter_map(|line| {
        let v: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|&s| !s.is_empty())
            .collect();

        if v.len() != 4 {
            return None;
        }

        let from = v[0].parse::<usize>().ok();
        let to = v[1].parse::<usize>().ok();
        let letter = v[2].parse::<char>().ok();

        Some(Rule {
            from: from?,
            to: to?,
            letter: letter?,
            password: String::from(v[3]),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"1-3 a: abcde
                                   1-3 b: cdefg
                                   2-9 c: ccccccccc"#;

    #[test]
    fn test_parse_input() {
        let result: Vec<Rule> = parse_input(
            r#"17-18 f: fffffffffffffffffff
                                    1 abc"#,
        )
        .collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, 17);
        assert_eq!(result[0].to, 18);
        assert_eq!(result[0].letter, 'f');
        assert_eq!(result[0].password, "fffffffffffffffffff");
    }

    #[test]
    fn test_is_password_simple() {
        let ref mut rule = Rule {
            from: 1,
            to: 3,
            letter: 'a',
            password: String::from("abcabab"),
        };

        assert!(is_password_simple(rule));

        rule.to = 2;
        assert!(!is_password_simple(rule));
    }

    #[test]
    fn test_is_password_complex() {
        let ref mut rule = Rule {
            from: 1,
            to: 3,
            letter: 'a',
            password: String::from("abcabab"),
        };

        assert!(is_password_complex(rule));

        rule.to = 4;
        assert!(!is_password_complex(rule));

        rule.from = 100;
        assert!(!is_password_complex(rule));
    }

    #[test]
    fn test_count_simple_passwords() {
        assert_eq!(count_simple_passwords(INPUT), 2);
    }

    #[test]
    fn test_count_complex_passwords() {
        assert_eq!(count_complex_passwords(INPUT), 1);
    }
}
