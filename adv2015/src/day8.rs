use std::char;

pub fn calc_difference(input: &str) -> usize {
    let literals = input
        .lines()
        .fold(0, |acc, line| acc + count_literals(line));

    let chars = input.lines().fold(0, |acc, line| acc + count_chars(line));

    literals - chars
}

pub fn calc_new_difference(input: &str) -> usize {
    let literals = input
        .lines()
        .fold(0, |acc, line| acc + count_literals(line));

    let chars = input
        .lines()
        .fold(0, |acc, line| acc + count_escaped_chars(line));

    chars - literals
}

fn count_literals(input: &str) -> usize {
    input.chars().count()
}

fn count_chars(input: &str) -> usize {
    unescape(input).chars().count()
}

fn count_escaped_chars(input: &str) -> usize {
    escape(input).chars().count()
}

// Info: this code was taken from the crate "snailquote". It wasn't stolen for sale, but for
// education purposes only. Just to solve Advent Calendar 2015
fn unescape(input: &str) -> String {
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    let mut chars = input.chars();

    let mut res = String::with_capacity(input.len());

    while let Some(c) = chars.next() {
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
                continue;
            }
        } else if in_double_quote {
            if c == '"' {
                in_double_quote = false;
                continue;
            }

            if c == '\\' {
                match chars.next() {
                    None => return String::new(),
                    Some(c2) => {
                        res.push(match c2 {
                            '\\' => '\\',
                            '\'' => '\'',
                            '"' => '"',
                            'x' => parse_unicode(&mut chars),
                            _ => {
                                return String::new();
                            }
                        });
                        continue;
                    }
                }
            }
        } else if c == '\'' {
            in_single_quote = true;
            continue;
        } else if c == '"' {
            in_double_quote = true;
            continue;
        }

        res.push(c);
    }

    res
}

fn escape(input: &str) -> String {
    let mut chars = input.chars();

    let mut res = String::with_capacity(input.len());
    res.push('"');

    while let Some(c) = chars.next() {
        match c {
            '\'' | '"' | '\\' => {
                res.push('\\');
                res.push(c)
            }
            _ => res.push(c),
        }
    }
    res.push('"');
    res
}

fn parse_unicode<I>(chars: &mut I) -> char
where
    I: Iterator<Item = char>,
{
    let c1 = chars.next().unwrap();
    let c2 = chars.next().unwrap();

    let d1 = c1.to_digit(16).unwrap();
    let d2 = c2.to_digit(16).unwrap();

    let hex = d1 * 16 + d2;

    char::from_u32(hex).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_literals() {
        assert_eq!(count_literals("\"\""), 2);
        assert_eq!(count_literals("\"abc\""), 5);
        assert_eq!(count_literals("\"aaa\\\"aaa\""), 10);
        assert_eq!(count_literals("\"\\x27\""), 6);
    }

    #[test]
    fn test_unescape() {
        assert_eq!(unescape("\"\""), "");
        assert_eq!(unescape("\"abc\""), "abc");
        assert_eq!(unescape("\"aaa\\\"aaa\""), "aaa\"aaa");
        assert_eq!(unescape("\"\\x27\""), "'");
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("\"\""), 0);
        assert_eq!(count_chars("\"abc\""), 3);
        assert_eq!(count_chars("\"aaa\\\"aaa\""), 7);
        assert_eq!(count_chars("\"\\x27\""), 1);
    }

    #[test]
    fn test_escape() {
        println!(
            "Debug: input={} result={} escape(input)={}",
            r#""""#,
            r#""\"\"""#,
            escape(r#""""#)
        );
        assert_eq!(escape(r#""""#), r#""\"\"""#);
        assert_eq!(escape(r#""abc""#), r#""\"abc\"""#);

        println!(
            "Debug: input={} result={} escape(input)={}",
            r#""aaa\"aaa""#,
            r#""\"aaa\\\"aaa\"""#,
            escape(r#""aaa\"aaa""#)
        );
        assert_eq!(escape(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(escape(r#""\x27""#), r#""\"\\x27\"""#);
    }

    #[test]
    fn test_count_escaped_chars() {
        assert_eq!(count_escaped_chars("\"\""), 6);
        assert_eq!(count_escaped_chars("\"abc\""), 9);
        assert_eq!(count_escaped_chars("\"aaa\\\"aaa\""), 16);
        assert_eq!(count_escaped_chars("\"\\x27\""), 11);
    }

    #[test]
    fn test_calc_difference() {
        assert_eq!(calc_difference("\"aaa\\\"aaa\""), 3);
        assert_eq!(calc_difference("\'aaa\\\'aaa\'"), 3);
    }

    #[test]
    fn test_calc_new_difference() {
        assert_eq!(calc_new_difference("\"aaa\\\"aaa\""), 6);
        assert_eq!(calc_new_difference("\'aaa\\\'aaa\'"), 6);
    }
}
