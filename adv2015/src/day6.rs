use std::fmt;

pub fn count_bulbs(input: &str) -> usize {
    let mut santa = SantaInterpreter::new();
    santa.interpret(String::from(input));
    santa.get_state()
}

pub fn count_brightness(input: &str) -> usize {
    let mut santa = SantaBetterInterpreter::new();
    santa.interpret(String::from(input));
    santa.get_state()
}

const LIGHT_MAX_SIZE: usize = 1000;

struct NextToken(Token, usize);

#[derive(Copy, Clone)]
struct Coord(usize, usize);

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
    Coord(usize, usize),

    EOF,
}

#[derive(Copy, Clone)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Copy, Clone)]
enum Call {
    Call(Operation, Coord, Coord),
    EOF,
}

struct Lexer {
    input: String,
    current_pos: usize,
}

struct Parser {
    lexer: Lexer,
    parsing: bool,
}

struct SantaInterpreter {
    state: Vec<Vec<bool>>,
    parser: Parser,
}

struct SantaBetterInterpreter {
    state: Vec<Vec<usize>>,
    parser: Parser,
}

impl Token {
    fn parse_token(input: &str) -> Token {
        match input {
            "turn on" => return Token::TurnOn,
            "turn off" => return Token::TurnOff,
            "toggle" => return Token::Toggle,
            "through" => return Token::Through,
            _ if input.contains(',') => return {
                let coord_pair: Vec<&str> = input.split(',').collect();
                let res_x = coord_pair[0].parse::<usize>();
                let res_y = coord_pair[1].parse::<usize>();

                Token::Coord(
                    res_x.unwrap_or(0usize),
                    res_y.unwrap_or(0usize),
                )
            },
            _ => Token::EOF,
        }
    }
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

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            parsing: true,
        }
    }

    fn next_operation(&mut self) -> Call {
        if !self.parsing {
            return Call::EOF;
        }

        match self.lexer.next_token() {
            Token::EOF => {
                self.parsing = false;
                Call::EOF
            }
            token @ Token::TurnOn | token @ Token::TurnOff | token @ Token::Toggle => {
                let op = match token {
                    Token::Toggle => Operation::Toggle,
                    Token::TurnOn => Operation::TurnOn,
                    _ => Operation::TurnOff, // eventually TurnOff
                };
                let c1 = match self.lexer.next_token() {
                    Token::Coord(x, y) => Coord(x, y),
                    t => {
                        println!("Unexpected token {} after token {}", t, token);
                        return Call::EOF;
                    }
                };
                let th = match self.lexer.next_token() {
                    t @ Token::Through => t,
                    t => {
                        println!("Unexpected token {} after token {}", t, c1);
                        return Call::EOF;
                    }
                };
                let c2 = match self.lexer.next_token() {
                    Token::Coord(x, y) => Coord(x, y),
                    t => {
                        println!("Unexpected token {} after token {}", t, th);
                        return Call::EOF;
                    }
                };

                Call::Call(op, c1, c2)
            }
            token => {
                println!("Unexpected token in the line: {}", token);
                self.parsing = false;
                Call::EOF
            }
        }
    }
}

impl Iterator for Parser {
    type Item = Call;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_operation() {
            Call::EOF => None,
            result => Some(result),
        }
    }
}

impl SantaInterpreter {
    fn new() -> SantaInterpreter {
        SantaInterpreter {
            parser: Parser::new(String::from("")),
            state: vec![vec![false; LIGHT_MAX_SIZE]; LIGHT_MAX_SIZE],
        }
    }

    fn interpret(&mut self, input: String) {
        self.parser = Parser::new(input);

        while let Some(c) = self.parser.next() {
            match c {
                Call::Call(op, c1, c2) => {
                    for x in c1.0..=c2.0 {
                        for y in c1.1..=c2.1 {
                            self.state[x][y] = match op {
                                Operation::TurnOn => true,
                                Operation::TurnOff => false,
                                Operation::Toggle => !self.state[x][y],
                            };
                        }
                    }
                }
                Call::EOF => break,
            };
        }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        for x in 0..LIGHT_MAX_SIZE {
            for y in 0..LIGHT_MAX_SIZE {
                self.state[x][y] = false;
            }
        }
    }

    fn get_state(&self) -> usize {
        self.state.iter()
            .fold(0usize, |acc1, line| {
                acc1 + line.iter()
                    .fold(0usize, |acc2, &bulb| {
                        acc2 + if bulb { 1 } else { 0 }
                    })
            })
    }
}

impl SantaBetterInterpreter {
    fn new() -> SantaBetterInterpreter {
        SantaBetterInterpreter {
            parser: Parser::new(String::from("")),
            state: vec![vec![0; LIGHT_MAX_SIZE]; LIGHT_MAX_SIZE],
        }
    }

    fn interpret(&mut self, input: String) {
        self.parser = Parser::new(input);

        while let Some(c) = self.parser.next() {
            match c {
                Call::Call(op, c1, c2) => {
                    for x in c1.0..=c2.0 {
                        for y in c1.1..=c2.1 {
                            self.state[x][y] = match op {
                                Operation::TurnOn => self.state[x][y].saturating_add(1),
                                Operation::TurnOff => self.state[x][y].saturating_sub(1),
                                Operation::Toggle => self.state[x][y].saturating_add(2),
                            };
                        }
                    }
                }
                Call::EOF => break,
            };
        }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        for x in 0..LIGHT_MAX_SIZE {
            for y in 0..LIGHT_MAX_SIZE {
                self.state[x][y] = 0;
            }
        }
    }

    fn get_state(&self) -> usize {
        self.state.iter()
            .fold(0usize, |acc1, line| {
                acc1.saturating_add(line.iter()
                    .fold(0usize, |acc2, &brightness| acc2.saturating_add(brightness)))
            })
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
                    _ => return NextToken(Token::parse_token(word.as_str()), i),
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

    return NextToken(Token::parse_token(word.as_str()), input.chars().count());
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Toggle => f.write_str("tg"),
            Token::TurnOn => f.write_str("on"),
            Token::TurnOff => f.write_str("off"),
            Token::Through => f.write_str(";"),
            Token::Coord(x, y) => f.write_fmt(format_args!("({},{})", x, y)),
            Token::EOF => f.write_str(";"),
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0, self.1))
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::TurnOn => f.write_str("on"),
            Operation::TurnOff => f.write_str("off"),
            Operation::Toggle => f.write_str("tg"),
        }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Call::Call(op, c1, c2) => f.write_fmt(format_args!("{}({}; {})", op, c1, c2)),
            Call::EOF => f.write_str("End"),
        }
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

    impl cmp::PartialEq for Operation {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Operation::TurnOn, Operation::TurnOn)
                | (Operation::TurnOff, Operation::TurnOff)
                | (Operation::Toggle, Operation::Toggle) => true,
                _ => false,
            }
        }
    }

    impl fmt::Debug for Operation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!("{}", self))
        }
    }

    impl fmt::Debug for Coord {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!("({},{})", self.0, self.1))
        }
    }

    impl cmp::PartialEq for Coord {
        fn eq(&self, other: &Self) -> bool {
            let Coord(x1, y1) = self;
            let Coord(x2, y2) = other;

            x1 == x2 && y1 == y2
        }
    }

    #[test]
    fn test_partial_eq() {
        assert_ne!(Token::TurnOn, Token::TurnOff);

        assert_ne!(Operation::TurnOn, Operation::TurnOff);
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
        assert_eq!(lexer.next_token(), Token::EOF, "Unexpected Token");
    }

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(String::from("turn on 499,989 through 806,992"));
        if let Call::Call(op, c1, c2) = parser.next_operation() {
            assert_eq!(op, Operation::TurnOn);
            assert_eq!(c1, Coord(499, 989));
            assert_eq!(c2, Coord(806, 992));
        } else { panic!("Shouldn't happen"); }
    }

    #[test]
    fn test_multiline_parser() {
        let mut parser = Parser::new(String::from("turn on 59,99 through 806,99\r\nturn off 812,389 through 865,874"));
        if let Call::Call(op, c1, c2) = parser.next_operation() {
            assert_eq!(op, Operation::TurnOn);
            assert_eq!(c1, Coord(59, 99));
            assert_eq!(c2, Coord(806, 99));
        } else { panic!("Shouldn't happen"); }
        if let Call::Call(op, c1, c2) = parser.next_operation() {
            assert_eq!(op, Operation::TurnOff);
            assert_eq!(c1, Coord(812, 389));
            assert_eq!(c2, Coord(865, 874));
        } else { panic!("Shouldn't happen"); }
    }

    #[test]
    fn test_santa_interpreter_all_on() {
        let input = String::from(format!("turn on 0,0 through {},{}", LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        println!("Testing: {}", input);
        let mut basic = SantaInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        println!("{} bulbs are showing us Christmas", answer);

        assert_eq!(answer, LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, "{} bulbs are showing us Christmas, but we see {} only", LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, answer);
    }

    #[test]
    fn test_santa_interpreter_all_off() {
        let input = String::from(format!("turn on 0,0 through {},{}\r\nturn off 0,0 through {},{}",
                                         LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        let mut basic = SantaInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        assert_eq!(answer, 0, "{} bulbs are showing us Christmas, but we see {} only", 0, answer);
    }

    #[test]
    fn test_santa_interpreter_all_toggle() {
        let input = String::from(format!("turn on 0,0 through {},{}\r\ntoggle 0,0 through {},{}",
                                         LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        let mut basic = SantaInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        assert_eq!(answer, 0, "{} bulbs are showing us Christmas, but we see {} only", 0, answer);
    }

    #[test]
    fn test_santa_interpreter() {
        let mut basic = SantaInterpreter::new();
        basic.interpret(String::from("turn on 0,0 through 99,99\r\ntoggle 100,100 through 199,199"));
        let answer = basic.get_state();
        assert_eq!(answer, 100 * 100 * 2, "{} bulbs are showing us Christmas, but we see {} only", 100 * 100 * 2, answer);
    }

    #[test]
    fn test_santa_reset() {
        let mut basic = SantaInterpreter::new();
        basic.interpret(String::from("turn on 0,0 through 99,99\r\ntoggle 100,100 through 199,199"));
        basic.reset();
        let answer = basic.get_state();

        assert_eq!(answer, 0, "There was a reset");
    }

    #[test]
    fn test_santa_better_interpreter_all_on() {
        let input = String::from(format!("turn on 0,0 through {},{}", LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        println!("Testing: {}", input);
        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        println!("{} Christmas brightness", answer);

        assert_eq!(answer, LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, "{} is Christmas brightness, but we have {} only", LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, answer);
    }

    #[test]
    fn test_santa_better_interpreter_all_off() {
        let input = String::from(format!("turn on 0,0 through {},{}\r\nturn off 0,0 through {},{}",
                                         LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        assert_eq!(answer, 0, "{} is Christmas brightness, but we have {} only", 0, answer);
    }

    #[test]
    fn test_santa_better_interpreter_all_toggle() {
        let input = String::from(format!("toggle 0,0 through {},{}",
                                         LIGHT_MAX_SIZE - 1, LIGHT_MAX_SIZE - 1));
        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        assert_eq!(answer, 2 * LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, "{} is Christmas brightness, but we have {} only", 2 * LIGHT_MAX_SIZE * LIGHT_MAX_SIZE, answer);
    }

    #[test]
    fn test_santa_better_interpreter_one_toggle() {
        let input = String::from("toggle 0,0 through 0,0");
        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(input);
        let answer = basic.get_state();
        assert_eq!(answer, 2, "{} is Christmas brightness, but we have {} only", 2, answer);
    }

    #[test]
    fn test_santa_better_reset() {
        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(String::from("turn on 0,0 through 99,99\r\ntoggle 100,100 through 199,199"));
        basic.reset();
        let answer = basic.get_state();

        assert_eq!(answer, 0, "There was a reset");
    }

    #[test]
    fn test_count_bulbs() {
        let result = count_bulbs("toggle 0,0 through 0,0\ntoggle 1,1 through 1,1");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_brightness() {
        let result = count_brightness("toggle 0,0 through 0,0\ntoggle 1,1 through 1,1");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_lexer_unexpected_token() {
        let mut lexer = Lexer::new(String::from("turn on oops 599,989 through 806,993"));
        assert_eq!(lexer.next_token(), Token::TurnOn, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::EOF, "Unexpected Token");
    }

    #[test]
    fn test_display() {
        assert_eq!(Token::Toggle.to_string(), "tg");
        assert_eq!(Token::TurnOn.to_string(), "on");
        assert_eq!(Token::TurnOff.to_string(), "off");
        assert_eq!(Token::EOF.to_string(), ";");
        assert_eq!(Token::Through.to_string(), ";");
        assert_eq!(Token::Coord(23, 32).to_string(), "(23,32)");

        assert_eq!(Coord(23, 32).to_string(), "(23,32)");

        assert_eq!(Operation::Toggle.to_string(), "tg");
        assert_eq!(Operation::TurnOn.to_string(), "on");
        assert_eq!(Operation::TurnOff.to_string(), "off");

        assert_eq!(Call::Call(Operation::TurnOn, Coord(1, 2), Coord(3, 4)).to_string(), "on((1,2); (3,4))");
        assert_eq!(Call::EOF.to_string(), "End");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Operation::Toggle), format!("{}", Operation::Toggle));

        assert_eq!(format!("{:?}", Coord(1, 2)), "(1,2)");
        assert_eq!(format!("{:?}", NextToken(Token::Toggle, 13)), "NextToken(Toggle, 13)");

        assert_eq!(format!("{:?}", Token::Toggle), "Toggle");
        assert_eq!(format!("{:?}", Token::TurnOn), "TurnOn");
        assert_eq!(format!("{:?}", Token::TurnOff), "TurnOff");
        assert_eq!(format!("{:?}", Token::EOF), "EOF");
        assert_eq!(format!("{:?}", Token::Through), "Through");
        assert_eq!(format!("{:?}", Token::Coord(23, 32)), "Coord(23, 32)");
    }

    #[test]
    fn test_wrong_syntax() {
        let mut basic = SantaInterpreter::new();
        basic.interpret(String::from("turn on through"));

        let mut basic = SantaBetterInterpreter::new();
        basic.interpret(String::from("turn off through"));
    }
}
