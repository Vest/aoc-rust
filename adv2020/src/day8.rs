use std::collections::HashSet;

pub fn execute_first_program(input: &str) -> i32 {
    let program = parse_program(input);

    if let Execution::Infinite(result) = execute_program(&program) {
        result
    } else {
        0
    }
}

pub fn execute_second_program(input: &str) -> i32 {
    let mut program = parse_program(input);

    for i in 0..program.len() {
        if let Operation::ACC(_) = program[i] {
            continue;
        }

        program[i] = swap_statement(&program[i]);

        if let Execution::Finished(result) = execute_program(&program) {
            return result;
        }

        program[i] = swap_statement(&program[i]);
    }
    0
}

fn swap_statement(statement: &Operation) -> Operation {
    match statement {
        Operation::ACC(value) => Operation::ACC(*value),
        Operation::NOP(value) => Operation::JMP(*value),
        Operation::JMP(value) => Operation::NOP(*value),
    }
}

#[derive(PartialEq, Debug)]
enum Operation {
    ACC(i32),
    NOP(i32),
    JMP(i32),
}

fn parse_line(input: &str) -> Option<Operation> {
    let input = input.trim().to_ascii_lowercase();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 2 {
        return None;
    }

    let command = tokens[0];
    let value_str = tokens[1];
    if let Ok(value) = value_str.parse::<i32>() {
        match command {
            "acc" => Some(Operation::ACC(value)),
            "nop" => Some(Operation::NOP(value)),
            "jmp" => Some(Operation::JMP(value)),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_program(input: &str) -> Vec<Operation> {
    input.lines().filter_map(parse_line).collect()
}

fn execute_program(program: &Vec<Operation>) -> Execution {
    let mut history: HashSet<usize> = HashSet::new();
    let mut acc = 0i32;
    let mut index = 0usize;

    loop {
        if !history.insert(index) {
            return Execution::Infinite(acc);
        }

        if let Some(op) = program.get(index) {
            match op {
                Operation::ACC(value) => {
                    acc += value;
                    index += 1;
                }

                Operation::JMP(value) if value.is_negative() => {
                    if let Some(result) = index.checked_sub(value.saturating_abs() as usize) {
                        index = result;
                    } else {
                        return Execution::None;
                    }
                }

                Operation::JMP(value) => {
                    index += *value as usize;
                }

                Operation::NOP(_) => {
                    index += 1;
                }
            }
        } else {
            return Execution::Finished(acc);
        }
    }
}

#[derive(PartialEq, Debug)]
enum Execution {
    Infinite(i32),
    Finished(i32),
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PROGRAM: &'static str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("acc +1"), Some(Operation::ACC(1)));
        assert_eq!(parse_line("jmp +4"), Some(Operation::JMP(4)));
        assert_eq!(parse_line("nop -2"), Some(Operation::NOP(-2)));
        assert_eq!(parse_line("nop"), None);
        assert_eq!(parse_line("nop nop nop"), None);
        assert_eq!(parse_line("nop nop"), None);
        assert_eq!(parse_line("abc -3"), None);
    }

    #[test]
    fn test_parse_program() {
        let program = parse_program(INPUT_PROGRAM);
        assert_eq!(program.len(), 9);
        assert_eq!(program[0], Operation::NOP(0));
        assert_eq!(program[8], Operation::ACC(6));
    }

    #[test]
    fn test_execute_program() {
        let program = parse_program(INPUT_PROGRAM);
        assert_eq!(execute_program(&program), Execution::Infinite(5));

        let program = parse_program("jmp 1000");
        assert_eq!(execute_program(&program), Execution::Finished(0));

        let program = parse_program("jmp -1000");
        assert_eq!(execute_program(&program), Execution::None);
    }

    #[test]
    fn test_execute_programs() {
        assert_eq!(execute_first_program(INPUT_PROGRAM), 5);
        assert_eq!(execute_second_program(INPUT_PROGRAM), 8);

        assert_eq!(execute_first_program(""), 0);
        assert_eq!(execute_second_program(""), 0);
    }
}
