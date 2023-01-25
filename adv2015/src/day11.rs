use std::collections::HashSet;

const MAX_LENGTH: usize = 8;

pub fn get_expired_once(input: &str) -> String {
    find_next(input)
}

pub fn get_expired_twice(input: &str) -> String {
    let expired = find_next(input);
    find_next(expired.as_str())
}

struct Password {
    state: String,
}

impl Password {
    fn new_str(input: &str) -> Password {
        Password {
            state: String::from(input),
        }
    }

    #[allow(dead_code)]
    fn new() -> Password {
        let mut pwd = Password {
            state: String::with_capacity(MAX_LENGTH),
        };

        for _ in 0..MAX_LENGTH {
            pwd.state.push('a');
        }

        pwd
    }

    fn get_password(&self) -> String {
        self.state.clone()
    }

    fn inc(&mut self) {
        let mut chars = self.state.chars().rev();
        let mut inc = 1u32;
        let mut result = String::with_capacity(MAX_LENGTH);
        while let Some(c) = chars.next() {
            let mut next_c = std::char::from_u32(c as u32 + inc).unwrap();

            if c == 'z' && inc == 1 {
                next_c = 'a';
            } else {
                inc = 0;
            }

            result.push(next_c);
        }

        self.state = result.chars().rev().collect::<String>()
    }
}

impl Iterator for Password {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.chars().filter(|c| *c == 'z').count() == self.state.len() {
            return None;
        }

        self.inc();
        Some(self.state.clone())
    }
}

fn is_increased(input: &str) -> bool {
    let mut count = 0u8;
    let mut prev_char: Option<char> = None;

    for c in input.chars() {
        if prev_char.is_none() {
            prev_char = Some(c);
            count = 1;
            continue;
        }

        let prev_ascii = prev_char.unwrap() as u8;
        let c_ascii = c as u8;

        count = if c_ascii.checked_sub(prev_ascii) == Some(1) {
            count + 1
        } else {
            1
        };

        if count == 3 {
            return true;
        }

        prev_char = Some(c);
    }

    false
}

fn is_not_confusing(input: &str) -> bool {
    !input.contains(|c| c == 'i' || c == 'o' || c == 'l')
}

fn has_pairs(input: &str) -> bool {
    let mut set: HashSet<String> = HashSet::new();
    let mut prev_char: Option<char> = None;
    let mut overlap_char: Option<char> = None;

    for c in input.chars() {
        if prev_char == None {
            prev_char = Some(c);
            continue;
        }

        if prev_char.unwrap() == c {
            // A potential pair
            if let Some(overlap) = overlap_char {
                if overlap == c {
                    continue;
                }
            }

            let mut pair = String::new();
            pair.push(c);
            pair.push(c);
            set.insert(pair);
        }

        overlap_char = prev_char;
        prev_char = Some(c);
    }

    set.len() > 1
}

fn find_next(input: &str) -> String {
    let mut pass = Password::new_str(input);

    while let Some(guessed_pass) = pass.next() {
        let pass_str = guessed_pass.as_str();

        if is_increased(pass_str) && is_not_confusing(pass_str) && has_pairs(pass_str) {
            break;
        }
    }

    pass.get_password()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_new() {
        let pwd = Password::new();
        let pwd_str = pwd.get_password();

        assert_eq!(
            pwd_str, "aaaaaaaa",
            "Expected a string with eight 'a' chars"
        );
    }

    #[test]
    fn test_password_inc() {
        let mut pwd = Password::new();
        pwd.inc();

        {
            let pwd_str = pwd.get_password();
            assert_eq!(
                pwd_str, "aaaaaaab",
                "Expected a string with seven 'a' chars and one 'b'"
            );
        }
        pwd.inc();
        pwd.inc();
        pwd.inc();
        pwd.inc();

        {
            let pwd_str = pwd.get_password();
            assert_eq!(
                pwd_str, "aaaaaaaf",
                "Expected a string with seven 'a' chars and one 'f'"
            );
        }
    }

    #[test]
    fn test_password_inc_close_to_max() {
        let mut pwd = Password::new_str("zzzzzzzy");
        pwd.inc();
        let pwd_str = pwd.get_password();

        assert_eq!(
            pwd_str, "zzzzzzzz",
            "Expected a string with eight 'z' chars"
        );
    }

    #[test]
    fn test_password_inc_max() {
        let mut pwd = Password::new_str("zzzzzzzz");
        pwd.inc();
        let pwd_str = pwd.get_password();

        assert_eq!(
            pwd_str, "aaaaaaaa",
            "Expected a string with eight 'a' chars"
        );
    }

    #[test]
    fn test_password_iter() {
        let mut pwd = Password::new();
        while let Some(pass) = pwd.next() {
            assert_eq!(
                pass, "aaaaaaab",
                "Expected a string with seven 'a' chars and one 'b'"
            );
            break;
        }
    }

    #[test]
    fn test_is_increased() {
        assert!(!is_increased("aaaaaaaa"));
        assert!(is_increased("abcaaaaa"));
        assert!(!is_increased("abbceffg"));
        assert!(!is_increased("abbcegjk"));
        assert!(is_increased("abcdffaa"));
        assert!(is_increased("ghjaabcc"));
        assert!(is_increased("hijklmmn"));
    }

    #[test]
    fn test_is_not_confusing() {
        assert!(!is_not_confusing("hijklmmn"));
        assert!(is_not_confusing("abbceffg"));
        assert!(is_not_confusing("abbcegjk"));
        assert!(is_not_confusing("abcdffaa"));
        assert!(is_not_confusing("ghjaabcc"));
    }

    #[test]
    fn test_has_pairs() {
        assert!(!has_pairs("hijklmmn"));
        assert!(has_pairs("abbceffg"));
        assert!(!has_pairs("abbcegjk"));
        assert!(has_pairs("abcdffaa"));
        assert!(has_pairs("ghjaabcc"));
        assert!(has_pairs("ghjaaabcc"));
        assert!(has_pairs("ghjaaaabcc"));
    }

    #[test]
    fn test_find_next_fast() {
        let expired_pass = "abcdefgh";
        let new_pass = find_next(expired_pass);

        assert_eq!(new_pass, "abcdffaa", "I expected another password");
    }

    #[test]
    #[ignore]
    fn test_find_next_slow() {
        let expired_pass = "ghijklmn";
        let new_pass = find_next(expired_pass);

        assert_eq!(new_pass, "ghjaabcc", "I expected another password");
    }
}
