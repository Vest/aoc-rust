use std::fmt;
use std::collections::HashMap;
use std::fmt::Formatter;

pub fn count_input_a(input: &str) -> u16 {
    let mut bobby = BobbyInterpreter::new();
    bobby.interpret(String::from(input));

    bobby.evaluate(&String::from("a")).unwrap()
}

pub fn count_input_a_override(input: &str) -> u16 {
    let mut bobby = BobbyInterpreter::new();
    bobby.interpret(String::from(input));

    let value_for_b = bobby.evaluate(&String::from("a")).unwrap();
    bobby.evaluate_override_signal(&String::from("a"), &String::from("b"), value_for_b).unwrap()
}

struct NextToken(Token, usize);

#[derive(Clone, Debug)]
enum Token {
    // value 0-65535
    Signal(u16),
    Wire(String),
    And,
    Or,
    LeftShift,
    RightShift,
    Not,
    Assign,
    EOF,
}


#[derive(Copy, Clone)]
enum Operation {
    Assign,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Clone)]
enum LValue {
    Const(u16),
    Var(String),
}

#[derive(Clone)]
enum RValue {
    Var(String),
}

#[derive(Clone)]
enum Command {
    Result(LValue),

    // A op B
    Binary(LValue, Operation, LValue),

    // x A
    Unary(Operation, LValue),
}

#[derive(Clone)]
enum Expression {
    NOP,
    // From - To
    Assign(Command, RValue),
}

struct Lexer {
    input: String,
    current_pos: usize,
}

struct Parser {
    lexer: Lexer,
    parsing: bool,
}

struct BobbyInterpreter {
    parser: Parser,
    tree: HashMap<String, Command>,
    cache: HashMap<String, u16>,
}


impl Lexer {
    fn new(input: String) -> Lexer {
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

impl Token {
    fn parse_token(input: &str) -> Token {
        match input {
            "AND" => return Token::And,
            "OR" => return Token::Or,
            "LSHIFT" => return Token::LeftShift,
            "RSHIFT" => return Token::RightShift,
            "NOT" => return Token::Not,
            "->" => return Token::Assign,
            str_value => {
                if str_value.is_empty() {
                    return Token::EOF;
                }

                if let Ok(value) = str_value.parse::<u16>() {
                    Token::Signal(value)
                } else {
                    Token::Wire(String::from(str_value))
                }
            }
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Signal(u) => f.write_fmt(format_args!("{}", u)),
            Token::Wire(s) => f.write_str(s.as_str()),
            Token::And => f.write_str("and"),
            Token::Or => f.write_str("or"),
            Token::LeftShift => f.write_str("lshift"),
            Token::RightShift => f.write_str("rshift"),
            Token::Not => f.write_str("not"),
            Token::Assign => f.write_str("->"),
            Token::EOF => f.write_str("eof"),
        }
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
                return NextToken(Token::parse_token(word.as_str()), i);
            }

            // skip whitespace
            _ if c.is_whitespace() && word.is_empty() => {
                continue;
            }

            // any letter
            _ => {
                word.push(c);
            }
        }
    }

    return NextToken(Token::parse_token(word.as_str()), input.chars().count());
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            parsing: true,
        }
    }

    fn next_operation(&mut self) -> Expression {
        if !self.parsing {
            return Expression::NOP;
        }

        let mut commands: Vec<Token> = Vec::new();

        while self.parsing {
            match self.lexer.next_token() {
                Token::EOF => self.parsing = false,

                Token::Assign => {
                    if commands.is_empty() {
                        println!("Error in parsing, there were no commands before the assignment");
                        self.parsing = false;
                    } else {
                        let token = self.lexer.next_token();
                        if let Token::Wire(target) = token {
                            if commands.len() == 1 {
                                if let Some(lvalue) = lvalue_from_one(&commands) {
                                    return Expression::Assign(Command::Result(lvalue), RValue::Var(target));
                                }
                            } else if commands.len() == 2 {
                                if let Some(lvalue) = lvalue_from_two(&commands) {
                                    return Expression::Assign(lvalue, RValue::Var(target));
                                }
                            } else if commands.len() == 3 {
                                if let Some(lvalue) = lvalue_from_three(&commands) {
                                    return Expression::Assign(lvalue, RValue::Var(target));
                                }
                            } else {
                                println!("Cannot parse tokens to a command. The size of the vector is {}:", commands.len());
                                for c in &commands {
                                    print!("{} ", c);
                                }
                                println!();
                                self.parsing = false;
                            }
                        } else {
                            println!("Unexpected token, expected a wire, but got {}", token);
                            self.parsing = false;
                        }
                    }
                }

                token => commands.push(token),
            }
        }
        return Expression::NOP;
    }
}

impl Iterator for Parser {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_operation() {
            Expression::NOP => None,
            result => Some(result),
        }
    }
}

fn lvalue_from_one(commands: &Vec<Token>) -> Option<LValue> {
    assert_eq!(commands.len(), 1);
    let lvalue = &commands[0];
    if let Token::Wire(s) = lvalue {
        Some(LValue::Var(s.clone()))
    } else if let Token::Signal(v) = lvalue {
        Some(LValue::Const(v.clone()))
    } else {
        None
    }
}

fn lvalue_from_two(commands: &Vec<Token>) -> Option<Command> {
    assert_eq!(commands.len(), 2);
    // NOT x
    let op = &commands[0];
    let lvalue = &commands[1];

    if let (Token::Not, Token::Wire(s)) = (op, lvalue) {
        return Some(Command::Unary(Operation::Not, LValue::Var(s.clone())));
    }
    None
}

fn lvalue_from_three(commands: &Vec<Token>) -> Option<Command> {
    assert_eq!(commands.len(), 3);
    // x op Y
    let lvalue1 = &commands[0];
    let op = &commands[1];
    let lvalue2 = &commands[2];

    match op {
        Token::And =>
            if let (Token::Wire(s1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(LValue::Var(s1.clone()), Operation::And, LValue::Var(s2.clone())))
            } else if let (Token::Signal(u1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(LValue::Const(u1.clone()), Operation::And, LValue::Var(s2.clone())))
            } else { None },
        Token::Or =>
            if let (Token::Wire(s1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(LValue::Var(s1.clone()), Operation::Or, LValue::Var(s2.clone())))
            } else { None },
        Token::LeftShift =>
            if let (Token::Wire(s1), Token::Signal(u1)) = (lvalue1, lvalue2) {
                Some(Command::Binary(LValue::Var(s1.clone()), Operation::LShift, LValue::Const(u1.clone())))
            } else { None },
        Token::RightShift =>
            if let (Token::Wire(s1), Token::Signal(u1)) = (lvalue1, lvalue2) {
                Some(Command::Binary(LValue::Var(s1.clone()), Operation::RShift, LValue::Const(u1.clone())))
            } else { None },

        _ => None
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::NOP => { f.write_str("NOP") }
            Expression::Assign(_, _) => { f.write_str("Assign") }
        }
    }
}

impl fmt::Display for LValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LValue::Const(c) => f.write_fmt(format_args!("{}", c)),
            LValue::Var(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Operation::Assign => "->",
            Operation::And => "&",
            Operation::Or => "|",
            Operation::LShift => "<<",
            Operation::RShift => ">>",
            Operation::Not => "!",
        })
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::Result(lvalue) =>
                f.write_fmt(format_args!("{}", lvalue)),
            Command::Binary(lvalue1, op, lvalue2) =>
                f.write_fmt(format_args!("{} {} {}", lvalue1, op, lvalue2)),
            Command::Unary(op, lvalue) =>
                f.write_fmt(format_args!("{} {}", op, lvalue)),
        }
    }
}

impl BobbyInterpreter {
    fn new() -> BobbyInterpreter {
        BobbyInterpreter {
            parser: Parser::new(String::from("")),
            tree: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn interpret(&mut self, input: String) {
        self.parser = Parser::new(input);
        self.tree.clear();
        self.cache.clear();

        while let Some(e) = self.parser.next() {
            match e {
                Expression::Assign(c, r) => {
                    if let RValue::Var(var_name) = r {
                        self.tree.insert(var_name, c);
                    }
                }
                Expression::NOP => {}
            };
        }
    }

    #[allow(dead_code)]
    fn print_ast(&self) {
        println!("Print AST:");
        for i in self.tree.iter() {
            println!("{} = {}", i.0, i.1);
        }
    }

    fn evaluate(&mut self, wire: &String) -> Option<u16> {
        // Read from cache, if the value exists there
        if let Some(&cached_value) = self.cache.get(wire) {
            return Some(cached_value);
        }

        let next_command = self.tree.get(&wire.clone());
        if next_command.is_none() {
            eprintln!("Wire '{}' doesn't have a command", wire);
            return None;
        }

        let result = match next_command.unwrap().clone() {
            Command::Result(lvalue) => {
                match lvalue {
                    LValue::Const(c) => {
                        Some(c.clone())
                    }
                    LValue::Var(w) => self.evaluate(&w),
                }
            }
            Command::Unary(op, lvalue) => {
                // There is only one Unary operation, for more you can add "match"
                if let Operation::Not = op {
                    match lvalue {
                        LValue::Const(c) => Some(!c.clone()),
                        LValue::Var(w) => {
                            if let Some(value) = self.evaluate(&w) {
                                Some(!value)
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Command::Binary(l1, op, l2) => {
                let lvalue1 = match l1 {
                    LValue::Const(c) => c.clone(),
                    LValue::Var(w) => {
                        self.evaluate(&w).unwrap()
                    }
                };

                let lvalue2 = match l2 {
                    LValue::Const(c) => c.clone(),
                    LValue::Var(w) => {
                        if let Some(value) = self.evaluate(&w) {
                            value
                        } else {
                            return None;
                        }
                    }
                };

                match op {
                    Operation::And => Some(lvalue1 & lvalue2),
                    Operation::Or => Some(lvalue1 | lvalue2),
                    Operation::LShift => Some(lvalue1 << lvalue2),
                    Operation::RShift => Some(lvalue1 >> lvalue2),
                    _ => None,
                }
            }
        };

        if let Some(u) = result {
            // Store to cache, if we can
            // println!("Store to cache: {}={}", wire, u);
            self.cache.insert(wire.clone(), u);
        }

        result
    }

    fn evaluate_override_signal(&mut self, wire: &String, new_wire: &String, new_value: u16) -> Option<u16> {
        self.cache.clear();
        self.tree.insert((*new_wire).clone(), Command::Result(LValue::Const(new_value)));

        self.evaluate(wire)
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

    impl fmt::Debug for LValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LValue::Const(c) => f.write_fmt(format_args!("{}", c)),
                LValue::Var(v) => f.write_fmt(format_args!("{}", v)),
            }
        }
    }

    impl cmp::PartialEq for LValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (LValue::Const(c1), LValue::Const(c2)) if c1 == c2 => true,
                (LValue::Var(w1), LValue::Var(w2)) if w1 == w2 => true,
                _ => false,
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
                (Token::Signal(x1), Token::Signal(x2)) if x1 == x2 => true,
                (Token::Wire(s1), Token::Wire(s2)) if s1 == s2 => true,
                (Token::And, Token::And) => true,
                (Token::Or, Token::Or) => true,
                (Token::LeftShift, Token::LeftShift) => true,
                (Token::RightShift, Token::RightShift) => true,
                (Token::Not, Token::Not) => true,
                (Token::Assign, Token::Assign) => true,
                (Token::EOF, Token::EOF) => true,
                _ => false,
            }
        }
    }

    #[test]
    fn test_get_token_assign() {
        let input = "123 -> x";
        assert_eq!(get_token(input, 0), NextToken(Token::Signal(123), 3), "Unexpected Token");
        assert_eq!(get_token(input, 3), NextToken(Token::Assign, 6), "Unexpected Token");
        assert_eq!(get_token(input, 6), NextToken(Token::Wire(String::from("x")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_and() {
        let input = "x AND y -> d";
        assert_eq!(get_token(input, 0), NextToken(Token::Wire(String::from("x")), 1), "Unexpected Token");
        assert_eq!(get_token(input, 1), NextToken(Token::And, 5), "Unexpected Token");
        assert_eq!(get_token(input, 5), NextToken(Token::Wire(String::from("y")), 7), "Unexpected Token");
        assert_eq!(get_token(input, 7), NextToken(Token::Assign, 10), "Unexpected Token");
        assert_eq!(get_token(input, 10), NextToken(Token::Wire(String::from("d")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_or() {
        let input = "x OR y -> e";
        assert_eq!(get_token(input, 0), NextToken(Token::Wire(String::from("x")), 1), "Unexpected Token");
        assert_eq!(get_token(input, 1), NextToken(Token::Or, 4), "Unexpected Token");
        assert_eq!(get_token(input, 4), NextToken(Token::Wire(String::from("y")), 6), "Unexpected Token");
        assert_eq!(get_token(input, 6), NextToken(Token::Assign, 9), "Unexpected Token");
        assert_eq!(get_token(input, 9), NextToken(Token::Wire(String::from("e")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_lshift() {
        let input = "x LSHIFT 2 -> f";
        assert_eq!(get_token(input, 0), NextToken(Token::Wire(String::from("x")), 1), "Unexpected Token");
        assert_eq!(get_token(input, 1), NextToken(Token::LeftShift, 8), "Unexpected Token");
        assert_eq!(get_token(input, 8), NextToken(Token::Signal(2), 10), "Unexpected Token");
        assert_eq!(get_token(input, 10), NextToken(Token::Assign, 13), "Unexpected Token");
        assert_eq!(get_token(input, 13), NextToken(Token::Wire(String::from("f")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_rshift() {
        let input = "y RSHIFT 2 -> g";
        assert_eq!(get_token(input, 0), NextToken(Token::Wire(String::from("y")), 1), "Unexpected Token");
        assert_eq!(get_token(input, 1), NextToken(Token::RightShift, 8), "Unexpected Token");
        assert_eq!(get_token(input, 8), NextToken(Token::Signal(2), 10), "Unexpected Token");
        assert_eq!(get_token(input, 10), NextToken(Token::Assign, 13), "Unexpected Token");
        assert_eq!(get_token(input, 13), NextToken(Token::Wire(String::from("g")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_get_token_not() {
        let input = "NOT x -> h";
        assert_eq!(get_token(input, 0), NextToken(Token::Not, 3), "Unexpected Token");
        assert_eq!(get_token(input, 3), NextToken(Token::Wire(String::from("x")), 5), "Unexpected Token");
        assert_eq!(get_token(input, 5), NextToken(Token::Assign, 8), "Unexpected Token");
        assert_eq!(get_token(input, 8), NextToken(Token::Wire(String::from("h")), input.len()), "Unexpected Token");
    }

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(String::from("123 -> x\r\n456 -> y\r\nx AND y -> d\r\nx OR y -> e"));
        assert_eq!(lexer.next_token(), Token::Signal(123), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("x")), "Unexpected Token");

        assert_eq!(lexer.next_token(), Token::Signal(456), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("y")), "Unexpected Token");

        assert_eq!(lexer.next_token(), Token::Wire(String::from("x")), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::And, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("y")), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("d")), "Unexpected Token");

        assert_eq!(lexer.next_token(), Token::Wire(String::from("x")), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Or, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("y")), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("e")), "Unexpected Token");
    }

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(String::from("123 -> x
                                           456 -> y
                                           x AND y -> d
                                           x OR y -> e
                                           x LSHIFT 2 -> f
                                           y RSHIFT 2 -> g
                                           NOT x -> h
                                           NOT y -> i"));
        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Result(LValue::Const(u)) = c {
                    assert_eq!(u, 123, "A wrong signal was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "x", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Result(LValue::Const(u)) = c {
                    assert_eq!(u, 456, "A wrong signal was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "y", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(LValue::Var(x), Operation::And, LValue::Var(y)) = c {
                    assert_eq!(x, "x", "A wrong wire was parsed");
                    assert_eq!(y, "y", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "d", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(LValue::Var(x), Operation::Or, LValue::Var(y)) = c {
                    assert_eq!(x, "x", "A wrong wire was parsed");
                    assert_eq!(y, "y", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "e", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };
    }

    #[test]
    fn test_bobby_interpreter() {
        let input = String::from("123 -> x
                                           456 -> y
                                           x AND y -> d
                                           x OR y -> e
                                           x LSHIFT 2 -> f
                                           y RSHIFT 2 -> g
                                           NOT x -> h
                                           NOT y -> i");
        let mut bobby = BobbyInterpreter::new();
        bobby.interpret(input);
        bobby.print_ast();
        let def = 0xffffu16;

        assert_eq!(bobby.evaluate(&String::from("x")).unwrap_or(def.clone()), 123, "Unexpected value X");
        assert_eq!(bobby.evaluate(&String::from("y")).unwrap_or(def.clone()), 456, "Unexpected value Y");
        assert_eq!(bobby.evaluate(&String::from("h")).unwrap_or(def.clone()), 65412, "Unexpected value H");
        assert_eq!(bobby.evaluate(&String::from("i")).unwrap_or(def.clone()), 65079, "Unexpected value I");
        assert_eq!(bobby.evaluate(&String::from("d")).unwrap_or(def.clone()), 72, "Unexpected value D");
        assert_eq!(bobby.evaluate(&String::from("e")).unwrap_or(def.clone()), 507, "Unexpected value E");
        assert_eq!(bobby.evaluate(&String::from("f")).unwrap_or(def.clone()), 492, "Unexpected value F");
        assert_eq!(bobby.evaluate(&String::from("g")).unwrap_or(def.clone()), 114, "Unexpected value G");
    }

    #[test]
    fn test_lexer_eof() {
        let mut lexer = Lexer::new(String::from("123 -> x
                                           456 -> y"));
        assert_eq!(lexer.next_token(), Token::Signal(123), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("x")), "Unexpected Token");

        assert_eq!(lexer.next_token(), Token::Signal(456), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Wire(String::from("y")), "Unexpected Token");

        assert_eq!(lexer.next_token(), Token::EOF, "Unexpected Token");
    }

    #[test]
    fn test_parser_with_const() {
        let mut parser = Parser::new(String::from("jp RSHIFT 5 -> js
        1 AND io -> ip
        eo LSHIFT 15 -> es"));

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(LValue::Var(x), Operation::RShift, LValue::Const(y)) = c {
                    assert_eq!(x, "jp", "A wrong wire was parsed");
                    assert_eq!(y, 5, "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "js", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(LValue::Const(x), Operation::And, LValue::Var(y)) = c {
                    assert_eq!(x, 1, "A wrong wire was parsed");
                    assert_eq!(y, "io", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "ip", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(LValue::Var(x), Operation::LShift, LValue::Const(y)) = c {
                    assert_eq!(x, "eo", "A wrong wire was parsed");
                    assert_eq!(y, 15, "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
                if let RValue::Var(s) = v {
                    assert_eq!(s, "es", "A wrong wire was parsed");
                } else {
                    assert!(false, "It wasn't parsed correctly");
                }
            }
            _ => assert!(false, "Shouldn't happen"),
        };
    }

    #[test]
    fn test_bobby_interpreter_empty() {
        let input = String::from("123 -> x
                                           456 -> y");
        let mut bobby = BobbyInterpreter::new();
        bobby.interpret(input);
        bobby.print_ast();
        let def = 0xffffu16;

        assert_eq!(bobby.evaluate(&String::from("vest")).unwrap_or(def.clone()), 0xffffu16, "We shouldn't find any value");
    }
}
