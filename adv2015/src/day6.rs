struct NextToken(Token, usize);

enum Token {
    // toggle
    Toggle,

    // turn on
    TurnOn,

    // turn off
    TurnOff,

    // through
    Through,

    // 768,548
    Coord(u16, u16),

    EOF,
}

enum Operation {
    Call(Token, Token, Token),
    EOF,
}

struct Lexer {
    input: String,
    current_pos: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let input = input.to_lowercase();

        Lexer {
            input,
            current_pos: 0usize,
        }
    }

    fn next_token(&mut self) -> Token {
        let next_token = get_token(self.input.as_str(), self.current_pos);
        self.current_pos = next_token.1;

        next_token.0
    }
}

fn get_token(input: &str, from: usize) -> NextToken {
    let mut word = String::with_capacity(7);
    let chars = input.chars();

    for (i, c) in chars.enumerate() {
        if i < from {
            continue;
        }

        match c {
            // whitespace, but we have a word in the buffer
            _ if c.is_whitespace() && !word.is_empty() => {
                match word.as_str() {
                    "turn" => word.push(' '),
                    _ => return NextToken(parse_token(word.as_str()), i),
                }
            }

            // letter
            _ if c.is_alphabetic() => {
                word.push(c);
            }

            // number
            _ if c.is_numeric() || c == ',' => {
                word.push(c);
            }

            // something else
            _ => continue,
        }
    }

    return NextToken(parse_token(word.as_str()), input.chars().count());
}

fn parse_token(input: &str) -> Token {
    match input {
        "turn on" => return Token::TurnOn,
        "turn off" => return Token::TurnOff,
        "toggle" => return Token::Toggle,
        "through" => return Token::Through,
        _ if input.contains(',') => return {
            let coord_pair: Vec<&str> = input.split(',').collect();
            let res_x = coord_pair[0].parse::<u16>();
            let res_y = coord_pair[1].parse::<u16>();

            Token::Coord(
                res_x.unwrap_or(0u16),
                res_y.unwrap_or(0u16),
            )
        },
        _ => Token::EOF,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{cmp, fmt};

    impl fmt::Debug for NextToken {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("NextToken")
                .field(&self.0)
                .field(&self.1)
                .finish()
        }
    }

    impl fmt::Debug for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Token::Toggle => f.write_str("Toggle"),
                Token::TurnOn => f.write_str("TurnOn"),
                Token::TurnOff => f.write_str("TurnOff"),
                Token::Through => f.write_str("Through"),
                Token::Coord(x, y) => f.write_fmt(format_args!("Coord({}, {})", x, y)),
                Token::EOF => f.write_str("EOF"),
            }
        }
    }

    impl cmp::PartialEq for NextToken {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    impl cmp::PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Token::Toggle, Token::Toggle) => true,
                (Token::TurnOn, Token::TurnOn) => true,
                (Token::TurnOff, Token::TurnOff) => true,
                (Token::Through, Token::Through) => true,
                (Token::EOF, Token::EOF) => true,
                (Token::Coord(x1, y1), Token::Coord(x2, y2)) if x1 == x2 && y1 == y2 => true,
                _ => false
            }
        }
    }

    #[test]
    fn test_get_token_toggle() {
        let input = "toggle 461,550 through 564,900";
        assert_eq!(get_token(input, 0), NextToken(Token::Toggle, 6), "Unexpected Token");
        assert_eq!(get_token(input, 6), NextToken(Token::Coord(461, 550), 14), "Unexpected Token");
        assert_eq!(get_token(input, 14), NextToken(Token::Through, 22), "Unexpected Token");
        assert_eq!(get_token(input, 23), NextToken(Token::Coord(564, 900), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_turn_off() {
        let input = "turn off 812,389 through 865,874";
        assert_eq!(get_token(input, 0), NextToken(Token::TurnOff, 8), "Unexpected Token");
        assert_eq!(get_token(input, 8), NextToken(Token::Coord(812, 389), 16), "Unexpected Token");
        assert_eq!(get_token(input, 16), NextToken(Token::Through, 24), "Unexpected Token");
        assert_eq!(get_token(input, 24), NextToken(Token::Coord(865, 874), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_turn_on() {
        let input = "turn on 599,989 through 806,993";
        assert_eq!(get_token(input, 0), NextToken(Token::TurnOn, 7), "Unexpected Token");
        assert_eq!(get_token(input, 8), NextToken(Token::Coord(599, 989), 15), "Unexpected Token");
        assert_eq!(get_token(input, 15), NextToken(Token::Through, 23), "Unexpected Token");
        assert_eq!(get_token(input, 23), NextToken(Token::Coord(806, 993), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(String::from("turn on 599,989 through 806,993"));
        assert_eq!(lexer.next_token(), Token::TurnOn, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Coord(599, 989), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Through, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Coord(806, 993), "Unexpected Token");
    }
}
