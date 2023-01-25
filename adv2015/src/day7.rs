use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

pub fn count_input_a(input: &str) -> u16 {
    let mut bobby = BobbyInterpreter::new();
    bobby.interpret(String::from(input));

    bobby.evaluate(&String::from("a")).unwrap_or_default()
}

pub fn count_input_a_override(input: &str) -> u16 {
    let mut bobby = BobbyInterpreter::new();
    bobby.interpret(String::from(input));

    let value_for_b = bobby.evaluate(&String::from("a")).unwrap();
    bobby
        .evaluate_override_signal(&String::from("a"), &String::from("b"), value_for_b)
        .unwrap_or_default()
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
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Clone)]
enum RValue {
    Const(u16),
    Var(String),
}

#[derive(Clone)]
enum LValue {
    Var(String),
}

#[derive(Clone)]
enum Command {
    Result(RValue),

    // A op B
    Binary(RValue, Operation, RValue),

    // x A
    Unary(Operation, RValue),
}

#[derive(Clone)]
enum Expression {
    NOP,
    // From - To
    Assign(Command, LValue),
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
                                    return Expression::Assign(
                                        Command::Result(lvalue),
                                        LValue::Var(target),
                                    );
                                }
                            } else if commands.len() == 2 {
                                if let Some(lvalue) = lvalue_from_two(&commands) {
                                    return Expression::Assign(lvalue, LValue::Var(target));
                                }
                            } else if commands.len() == 3 {
                                if let Some(lvalue) = lvalue_from_three(&commands) {
                                    return Expression::Assign(lvalue, LValue::Var(target));
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

fn lvalue_from_one(commands: &Vec<Token>) -> Option<RValue> {
    assert_eq!(commands.len(), 1);
    let lvalue = &commands[0];
    if let Token::Wire(s) = lvalue {
        Some(RValue::Var(s.clone()))
    } else if let Token::Signal(v) = lvalue {
        Some(RValue::Const(v.clone()))
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
        return Some(Command::Unary(Operation::Not, RValue::Var(s.clone())));
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
        Token::And => {
            if let (Token::Wire(s1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(
                    RValue::Var(s1.clone()),
                    Operation::And,
                    RValue::Var(s2.clone()),
                ))
            } else if let (Token::Signal(u1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(
                    RValue::Const(u1.clone()),
                    Operation::And,
                    RValue::Var(s2.clone()),
                ))
            } else {
                None
            }
        }
        Token::Or => {
            if let (Token::Wire(s1), Token::Wire(s2)) = (lvalue1, lvalue2) {
                Some(Command::Binary(
                    RValue::Var(s1.clone()),
                    Operation::Or,
                    RValue::Var(s2.clone()),
                ))
            } else {
                None
            }
        }
        Token::LeftShift => {
            if let (Token::Wire(ls1), Token::Signal(u1)) = (lvalue1, lvalue2) {
                Some(Command::Binary(
                    RValue::Var(ls1.clone()),
                    Operation::LShift,
                    RValue::Const(u1.clone()),
                ))
            } else {
                None
            }
        }
        Token::RightShift => {
            if let (Token::Wire(rs1), Token::Signal(u1)) = (lvalue1, lvalue2) {
                Some(Command::Binary(
                    RValue::Var(rs1.clone()),
                    Operation::RShift,
                    RValue::Const(u1.clone()),
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::NOP => f.write_str("NOP"),
            Expression::Assign(_, _) => f.write_str("Assign"),
        }
    }
}

impl fmt::Display for LValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LValue::Var(name) => f.write_str(name),
        }
    }
}

impl fmt::Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RValue::Const(c) => f.write_fmt(format_args!("{}", c)),
            RValue::Var(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
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
            Command::Result(lvalue) => f.write_fmt(format_args!("{}", lvalue)),
            Command::Binary(lvalue1, op, lvalue2) => {
                f.write_fmt(format_args!("{} {} {}", lvalue1, op, lvalue2))
            }
            Command::Unary(op, lvalue) => f.write_fmt(format_args!("{} {}", op, lvalue)),
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
                    let LValue::Var(var_name) = r;
                    self.tree.insert(var_name, c);
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
            Command::Result(lvalue) => match lvalue {
                RValue::Const(c) => Some(c.clone()),
                RValue::Var(w) => self.evaluate(&w),
            },
            Command::Unary(op, lvalue) => {
                // There is only one Unary operation, for more you can add "match"
                if let Operation::Not = op {
                    match lvalue {
                        RValue::Const(c) => Some(!c.clone()),
                        RValue::Var(w) => Some(!self.evaluate(&w).unwrap()),
                    }
                } else {
                    None
                }
            }
            Command::Binary(l1, op, l2) => {
                let lvalue1 = match l1 {
                    RValue::Const(c) => c.clone(),
                    RValue::Var(w) => self.evaluate(&w).unwrap(),
                };

                let lvalue2 = match l2 {
                    RValue::Const(c) => c.clone(),
                    RValue::Var(w) => self.evaluate(&w).unwrap(),
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
            self.cache.insert(wire.clone(), u);
        }

        result
    }

    fn evaluate_override_signal(
        &mut self,
        wire: &String,
        new_wire: &String,
        new_value: u16,
    ) -> Option<u16> {
        self.cache.clear();
        self.tree.insert(
            (*new_wire).clone(),
            Command::Result(RValue::Const(new_value)),
        );

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

    impl fmt::Debug for RValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RValue::Const(c) => f.write_fmt(format_args!("{}", c)),
                RValue::Var(v) => f.write_fmt(format_args!("{}", v)),
            }
        }
    }

    impl fmt::Debug for Expression {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expression::NOP => f.write_str("NOP"),
                Expression::Assign(command, rvalue) => {
                    f.write_fmt(format_args!("{} = {}", rvalue, command))
                }
            }
        }
    }

    impl cmp::PartialEq for RValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (RValue::Const(c1), RValue::Const(c2)) if c1 == c2 => true,
                (RValue::Var(w1), RValue::Var(w2)) if w1 == w2 => true,
                _ => false,
            }
        }
    }

    impl cmp::PartialEq for Expression {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Expression::NOP, Expression::NOP) => true,
                (Expression::Assign(c1, r1), Expression::Assign(c2, r2))
                    if c1 == c2 && r1 == r2 =>
                {
                    true
                }
                _ => false,
            }
        }
    }

    impl cmp::PartialEq for Command {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Command::Result(l1), Command::Result(l2)) if l1 == l2 => true,
                (Command::Binary(l11, op1, l12), Command::Binary(l21, op2, l22))
                    if l11 == l21 && op1 == op2 && l12 == l22 =>
                {
                    true
                }
                (Command::Unary(op1, l1), Command::Unary(op2, l2)) if op1 == op2 && l1 == l2 => {
                    true
                }
                _ => false,
            }
        }
    }

    impl cmp::PartialEq for NextToken {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    impl cmp::PartialEq for LValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (LValue::Var(r1), LValue::Var(r2)) if r1 == r2 => true,
                _ => false,
            }
        }
    }

    impl cmp::PartialEq for Operation {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Operation::And, Operation::And) => true,
                (Operation::Or, Operation::Or) => true,
                (Operation::LShift, Operation::LShift) => true,
                (Operation::RShift, Operation::RShift) => true,
                (Operation::Not, Operation::Not) => true,
                _ => false,
            }
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
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Signal(123), 3),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 3),
            NextToken(Token::Assign, 6),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 6),
            NextToken(Token::Wire(String::from("x")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_get_token_and() {
        let input = "x AND y -> d";
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Wire(String::from("x")), 1),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 1),
            NextToken(Token::And, 5),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 5),
            NextToken(Token::Wire(String::from("y")), 7),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 7),
            NextToken(Token::Assign, 10),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 10),
            NextToken(Token::Wire(String::from("d")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_get_token_or() {
        let input = "x OR y -> e";
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Wire(String::from("x")), 1),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 1),
            NextToken(Token::Or, 4),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 4),
            NextToken(Token::Wire(String::from("y")), 6),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 6),
            NextToken(Token::Assign, 9),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 9),
            NextToken(Token::Wire(String::from("e")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_get_token_lshift() {
        let input = "x LSHIFT 2 -> f";
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Wire(String::from("x")), 1),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 1),
            NextToken(Token::LeftShift, 8),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 8),
            NextToken(Token::Signal(2), 10),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 10),
            NextToken(Token::Assign, 13),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 13),
            NextToken(Token::Wire(String::from("f")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_get_token_rshift() {
        let input = "y RSHIFT 2 -> g";
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Wire(String::from("y")), 1),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 1),
            NextToken(Token::RightShift, 8),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 8),
            NextToken(Token::Signal(2), 10),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 10),
            NextToken(Token::Assign, 13),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 13),
            NextToken(Token::Wire(String::from("g")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_get_token_not() {
        let input = "NOT x -> h";
        assert_eq!(
            get_token(input, 0),
            NextToken(Token::Not, 3),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 3),
            NextToken(Token::Wire(String::from("x")), 5),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 5),
            NextToken(Token::Assign, 8),
            "Unexpected Token"
        );
        assert_eq!(
            get_token(input, 8),
            NextToken(Token::Wire(String::from("h")), input.len()),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(String::from(
            "123 -> x\r\n456 -> y\r\nx AND y -> d\r\nx OR y -> e",
        ));
        assert_eq!(lexer.next_token(), Token::Signal(123), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("x")),
            "Unexpected Token"
        );

        assert_eq!(lexer.next_token(), Token::Signal(456), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("y")),
            "Unexpected Token"
        );

        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("x")),
            "Unexpected Token"
        );
        assert_eq!(lexer.next_token(), Token::And, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("y")),
            "Unexpected Token"
        );
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("d")),
            "Unexpected Token"
        );

        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("x")),
            "Unexpected Token"
        );
        assert_eq!(lexer.next_token(), Token::Or, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("y")),
            "Unexpected Token"
        );
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("e")),
            "Unexpected Token"
        );
    }

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(String::from(
            "123 -> x
                                           456 -> y
                                           x AND y -> d
                                           x OR y -> e
                                           x LSHIFT 2 -> f
                                           y RSHIFT 2 -> g
                                           NOT x -> h
                                           NOT y -> i",
        ));
        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Result(RValue::Const(u)) = c {
                    assert_eq!(u, 123, "A wrong signal was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "x", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Result(RValue::Const(u)) = c {
                    assert_eq!(u, 456, "A wrong signal was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "y", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(RValue::Var(x), Operation::And, RValue::Var(y)) = c {
                    assert_eq!(x, "x", "A wrong wire was parsed");
                    assert_eq!(y, "y", "A wrong wire was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "d", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(RValue::Var(x), Operation::Or, RValue::Var(y)) = c {
                    assert_eq!(x, "x", "A wrong wire was parsed");
                    assert_eq!(y, "y", "A wrong wire was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "e", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };
    }

    #[test]
    fn test_bobby_interpreter() {
        let input = String::from(
            "123 -> x
                                           456 -> y
                                           x AND y -> d
                                           x OR y -> e
                                           x LSHIFT 2 -> f
                                           y RSHIFT 2 -> g
                                           NOT x -> h
                                           NOT y -> i",
        );
        let mut bobby = BobbyInterpreter::new();
        bobby.interpret(input);
        bobby.print_ast();
        let def = 0xffffu16;

        assert_eq!(
            bobby.evaluate(&String::from("x")).unwrap_or(def.clone()),
            123,
            "Unexpected value X"
        );
        assert_eq!(
            bobby.evaluate(&String::from("y")).unwrap_or(def.clone()),
            456,
            "Unexpected value Y"
        );
        assert_eq!(
            bobby.evaluate(&String::from("h")).unwrap_or(def.clone()),
            65412,
            "Unexpected value H"
        );
        assert_eq!(
            bobby.evaluate(&String::from("i")).unwrap_or(def.clone()),
            65079,
            "Unexpected value I"
        );
        assert_eq!(
            bobby.evaluate(&String::from("d")).unwrap_or(def.clone()),
            72,
            "Unexpected value D"
        );
        assert_eq!(
            bobby.evaluate(&String::from("e")).unwrap_or(def.clone()),
            507,
            "Unexpected value E"
        );
        assert_eq!(
            bobby.evaluate(&String::from("f")).unwrap_or(def.clone()),
            492,
            "Unexpected value F"
        );
        assert_eq!(
            bobby.evaluate(&String::from("g")).unwrap_or(def.clone()),
            114,
            "Unexpected value G"
        );
    }

    #[test]
    fn test_lexer_eof() {
        let mut lexer = Lexer::new(String::from(
            "123 -> x
                                           456 -> y",
        ));
        assert_eq!(lexer.next_token(), Token::Signal(123), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("x")),
            "Unexpected Token"
        );

        assert_eq!(lexer.next_token(), Token::Signal(456), "Unexpected Token");
        assert_eq!(lexer.next_token(), Token::Assign, "Unexpected Token");
        assert_eq!(
            lexer.next_token(),
            Token::Wire(String::from("y")),
            "Unexpected Token"
        );

        assert_eq!(lexer.next_token(), Token::EOF, "Unexpected Token");
    }

    #[test]
    fn test_parser_with_const() {
        let mut parser = Parser::new(String::from(
            "jp RSHIFT 5 -> js
        1 AND io -> ip
        eo LSHIFT 15 -> es",
        ));

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(RValue::Var(x), Operation::RShift, RValue::Const(y)) = c {
                    assert_eq!(x, "jp", "A wrong wire was parsed");
                    assert_eq!(y, 5, "A wrong wire was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "js", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(RValue::Const(x), Operation::And, RValue::Var(y)) = c {
                    assert_eq!(x, 1, "A wrong wire was parsed");
                    assert_eq!(y, "io", "A wrong wire was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "ip", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };

        match parser.next_operation() {
            Expression::Assign(c, v) => {
                if let Command::Binary(RValue::Var(x), Operation::LShift, RValue::Const(y)) = c {
                    assert_eq!(x, "eo", "A wrong wire was parsed");
                    assert_eq!(y, 15, "A wrong wire was parsed");
                } else {
                    panic!("It wasn't parsed correctly");
                }
                let LValue::Var(s) = v;
                assert_eq!(s, "es", "A wrong wire was parsed");
            }
            _ => panic!("Shouldn't happen"),
        };
    }

    #[test]
    fn test_bobby_interpreter_empty() {
        let input = String::from(
            "123 -> x
                                           456 -> y",
        );
        let mut bobby = BobbyInterpreter::new();
        bobby.interpret(input);
        bobby.print_ast();
        let def = 0xffffu16;

        assert_eq!(
            bobby.evaluate(&String::from("vest")).unwrap_or(def.clone()),
            0xffffu16,
            "We shouldn't find any value"
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Token::Signal(23).to_string(), "23");
        assert_eq!(Token::Wire(String::from("Vest")).to_string(), "Vest");
        assert_eq!(Token::And.to_string(), "and");
        assert_eq!(Token::Or.to_string(), "or");
        assert_eq!(Token::LeftShift.to_string(), "lshift");
        assert_eq!(Token::RightShift.to_string(), "rshift");
        assert_eq!(Token::Not.to_string(), "not");
        assert_eq!(Token::Assign.to_string(), "->");
        assert_eq!(Token::EOF.to_string(), "eof");

        assert_eq!(
            Expression::Assign(
                Command::Result(RValue::Const(23)),
                LValue::Var(String::from("abc"))
            )
            .to_string(),
            "Assign"
        );
        assert_eq!(Expression::NOP.to_string(), "NOP");
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", NextToken(Token::Signal(23), 12)),
            "NextToken(Signal(23), 12)"
        );

        assert_eq!(format!("{:?}", RValue::Var(String::from("a"))), "a");
        assert_eq!(format!("{:?}", RValue::Const(23)), "23");

        assert_eq!(
            format!(
                "{:?}",
                Expression::Assign(
                    Command::Result(RValue::Const(23)),
                    LValue::Var(String::from("abc"))
                )
            ),
            "abc = 23"
        );
        assert_eq!(
            format!(
                "{:?}",
                Expression::Assign(
                    Command::Result(RValue::Var(String::from("a"))),
                    LValue::Var(String::from("abc"))
                )
            ),
            "abc = a"
        );

        assert_eq!(format!("{:?}", Expression::NOP), "NOP");
    }

    #[test]
    fn test_parser_errors() {
        let mut parser = Parser::new(String::from("2 -> x"));
        let assign = parser.next_operation();
        assert_eq!(
            assign,
            Expression::Assign(
                Command::Result(RValue::Const(2)),
                LValue::Var(String::from("x"))
            )
        );

        let mut parser = Parser::new(String::from("x RSHIFT 2 -> y"));
        let rshift_good = parser.next_operation();
        assert_eq!(
            rshift_good,
            Expression::Assign(
                Command::Binary(
                    RValue::Var(String::from("x")),
                    Operation::RShift,
                    RValue::Const(2)
                ),
                LValue::Var(String::from("y"))
            )
        );

        let mut parser = Parser::new(String::from("x LSHIFT 3 -> ly"));
        let lshift_good = parser.next_operation();
        assert_eq!(
            lshift_good,
            Expression::Assign(
                Command::Binary(
                    RValue::Var(String::from("x")),
                    Operation::LShift,
                    RValue::Const(3)
                ),
                LValue::Var(String::from("ly"))
            )
        );

        let mut parser = Parser::new(String::from("x AND y -> ay"));
        let and_good = parser.next_operation();
        assert_eq!(
            and_good,
            Expression::Assign(
                Command::Binary(
                    RValue::Var(String::from("x")),
                    Operation::And,
                    RValue::Var(String::from("y"))
                ),
                LValue::Var(String::from("ay"))
            )
        );

        let mut parser = Parser::new(String::from("y OR x -> oy"));
        let or_good = parser.next_operation();
        assert_eq!(
            or_good,
            Expression::Assign(
                Command::Binary(
                    RValue::Var(String::from("y")),
                    Operation::Or,
                    RValue::Var(String::from("x"))
                ),
                LValue::Var(String::from("oy"))
            )
        );

        let mut parser = Parser::new(String::from("NOT y -> ny"));
        let not_good = parser.next_operation();
        assert_eq!(
            not_good,
            Expression::Assign(
                Command::Unary(Operation::Not, RValue::Var(String::from("y"))),
                LValue::Var(String::from("ny"))
            )
        );

        let mut parser = Parser::new(String::from("2 LSHIFT 1 -> z"));
        let lshift = parser.next_operation();
        assert_eq!(lshift, Expression::NOP); // constants are not supported

        let mut parser = Parser::new(String::from("3 AND 1 -> w"));
        let and = parser.next_operation();
        assert_eq!(and, Expression::NOP); // constants are not supported

        let mut parser = Parser::new(String::from("3 OR 1 -> v"));
        let or = parser.next_operation();
        assert_eq!(or, Expression::NOP); // constants are not supported

        let mut parser = Parser::new(String::from("4 RSHIFT 2 -> u"));
        let rshift_bad = parser.next_operation();
        assert_eq!(rshift_bad, Expression::NOP); // constants are not supported

        let mut parser = Parser::new(String::from("x -> t"));
        let assign_bad = parser.next_operation();
        assert_eq!(
            assign_bad,
            Expression::Assign(
                Command::Result(RValue::Var(String::from("x"))),
                LValue::Var(String::from("t"))
            )
        );

        let mut parser = Parser::new(String::from("8 RSHIFT x -> t"));
        let rshift_x = parser.next_operation();
        assert_eq!(rshift_x, Expression::NOP); // constants are not supported

        let mut parser = Parser::new(String::from("2 RSHIFT -> s"));
        let rshift_s = parser.next_operation();
        assert_eq!(rshift_s, Expression::NOP);

        let mut parser = Parser::new(String::from("RSHIFT -> r"));
        let rshift_empty = parser.next_operation();
        assert_eq!(rshift_empty, Expression::NOP);

        let mut parser = Parser::new(String::from("RSHIFT AND OR LSHIFT -> q"));
        let token_mess = parser.next_operation();
        assert_eq!(token_mess, Expression::NOP);

        let mut parser = Parser::new(String::from("2 NOT -> p"));
        let not = parser.next_operation();
        assert_eq!(not, Expression::NOP);

        let mut parser = Parser::new(String::from("2 NOT NOT -> p"));
        let double_not = parser.next_operation();
        assert_eq!(double_not, Expression::NOP);

        let mut parser = Parser::new(String::from("2 -> 2"));
        let lvalue = parser.next_operation();
        assert_eq!(lvalue, Expression::NOP);

        let mut parser = Parser::new(String::from("-> x"));
        let empty = parser.next_operation();
        assert_eq!(empty, Expression::NOP);

        let nop = parser.next_operation();
        assert_eq!(nop, Expression::NOP);
        assert!(!parser.parsing, "Parsing is not stopped");
    }

    #[test]
    fn test_interpreter() {
        let mut bobby = BobbyInterpreter::new();
        bobby.interpret(String::from(
            r#"NOT a -> b
        b AND d -> e
        1 -> a
        a -> d"#,
        ));
        let answer = bobby.evaluate(&String::from("e"));
        assert_eq!(answer.unwrap(), 0);
    }

    #[test]
    fn test_answers() {
        const INPUT: &str = r#"1 -> b
        b AND d -> e
        NOT e -> a
        3 -> d"#;
        assert_eq!(count_input_a(INPUT), 65534);
        assert_eq!(count_input_a_override(INPUT), 65533);
    }
}
