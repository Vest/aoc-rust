const MAX_LENGTH: usize = 8;

struct Password {
    state: String,
}

impl Password {
    fn new_str(input: &str) -> Password {
        Password {
            state: String::from(input)
        }
    }

    fn new() -> Password {
        let mut pwd = Password {
            state: String::with_capacity(MAX_LENGTH)
        };

        for _ in 0..MAX_LENGTH {
            pwd.state.push('a');
        }

        pwd
    }

    fn get_password(&self) -> &str {
        self.state.as_str()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_new() {
        let pwd = Password::new();
        let pwd_str = pwd.get_password();

        assert_eq!(pwd_str, "aaaaaaaa", "Expected a string with eight 'a' chars");
    }

    #[test]
    fn test_password_inc() {
        let mut pwd = Password::new();
        pwd.inc();

        {
            let pwd_str = pwd.get_password();
            assert_eq!(pwd_str, "aaaaaaab", "Expected a string with seven 'a' chars and one 'b'");
        }
        pwd.inc();
        pwd.inc();
        pwd.inc();
        pwd.inc();

        {
            let pwd_str = pwd.get_password();
            assert_eq!(pwd_str, "aaaaaaaf", "Expected a string with seven 'a' chars and one 'f'");
        }
    }

    #[test]
    fn test_password_inc_close_to_max() {
        let mut pwd = Password::new_str("zzzzzzzy");
        pwd.inc();
        let pwd_str = pwd.get_password();

        assert_eq!(pwd_str, "zzzzzzzz", "Expected a string with eight 'z' chars");
    }

    #[test]
    fn test_password_inc_max() {
        let mut pwd = Password::new_str("zzzzzzzz");
        pwd.inc();
        let pwd_str = pwd.get_password();

        assert_eq!(pwd_str, "aaaaaaaa", "Expected a string with eight 'a' chars");
    }

    #[test]
    fn test_password_iter() {
        let mut pwd = Password::new();
        while let Some(pass) = pwd.next() {
            assert_eq!(pass, "aaaaaaab", "Expected a string with seven 'a' chars and one 'b'");
            break;
        }
    }
}
