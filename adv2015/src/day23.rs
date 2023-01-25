use std::collections::HashMap;
use std::str::FromStr;

pub fn get_answer_b(input: &str) -> i32 {
    let instructions = parse_code(input);
    let mut machine = Machine::new(instructions);

    while machine.do_evaluate() {}

    Machine::get_register(&mut machine.registers, &'b')
}

pub fn get_answer_b_after_a(input: &str) -> i32 {
    let instructions = parse_code(input);
    let mut machine = Machine::new(instructions);
    Machine::set_register(&mut machine.registers, &'a', 1);

    while machine.do_evaluate() {}

    Machine::get_register(&mut machine.registers, &'b')
}

type Offset = i16;
type Register = char;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Instruction {
    // sets register r to half its current value, then continues with the next instruction
    hlf(Register),

    // sets register r to triple its current value, then continues with the next instruction
    tpl(Register),

    // increments register r, adding 1 to it, then continues with the next instruction
    inc(Register),

    // offset is a jump; it continues with the instruction offset away relative to itself
    jmp(Offset),

    // is like jmp, but only jumps if register r is even ("jump if even")
    jie(Register, Offset),

    // is like jmp, but only jumps if register r is 1 ("jump if one", not odd)
    jio(Register, Offset),
}

struct Machine {
    registers: HashMap<Register, i32>,
    position: usize,
    instructions: Vec<Instruction>,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Machine {
        Machine {
            registers: HashMap::with_capacity(2),
            position: 0,
            instructions,
        }
    }

    fn do_evaluate(&mut self) -> bool {
        if let Some(inst) = self.instructions.get(self.position) {
            match inst {
                Instruction::hlf(r) | Instruction::tpl(r) | Instruction::inc(r) => {
                    let register_value = Self::get_register(&mut self.registers, r);

                    let new_register_value = if let Instruction::hlf(_) = inst {
                        register_value.overflowing_div(2)
                    } else if let Instruction::tpl(_) = inst {
                        register_value.overflowing_mul(3)
                    } else {
                        register_value.overflowing_add(1)
                    };

                    Self::set_register(&mut self.registers, r, new_register_value.0);
                    Self::jump(&mut self.position, 1) && !new_register_value.1
                }

                Instruction::jmp(o) => Self::jump(&mut self.position, *o),

                Instruction::jie(r, o) | Instruction::jio(r, o) => {
                    let register_value = Self::get_register(&mut self.registers, r);

                    if let Instruction::jie(_, _) = inst {
                        // Is even
                        Self::jump(
                            &mut self.position,
                            if register_value & 1 == 0 { *o } else { 1 },
                        )
                    } else {
                        Self::jump(&mut self.position, if register_value == 1 { *o } else { 1 })
                    }
                }
            }
        } else {
            false
        }
    }

    fn jump(position: &mut usize, offset: Offset) -> bool {
        let o = if offset.is_negative() {
            position.overflowing_sub(offset.abs() as usize)
        } else {
            position.overflowing_add(offset as usize)
        };
        *position = o.0;

        !o.1
    }

    fn get_register(registers: &mut HashMap<Register, i32>, r: &Register) -> i32 {
        registers.entry(*r).or_insert(0).clone()
    }

    fn set_register(registers: &mut HashMap<Register, i32>, r: &Register, v: i32) {
        registers.insert(*r, v);
    }
}

#[derive(Debug)]
struct ParseInstructionError(String);

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s
            .trim()
            .split(|c: char| c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .collect();

        if split.len() != 2 && split.len() != 3 {
            return Err(ParseInstructionError(String::from(s)));
        }

        let inst = split[0];
        match inst {
            "hlf" => char::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .map(|r| Instruction::hlf(r)),

            "tpl" => char::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .map(|r| Instruction::tpl(r)),

            "inc" => char::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .map(|r| Instruction::inc(r)),

            "jmp" => Offset::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .map(|o| Instruction::jmp(o)),

            "jie" if split.len() == 3 => char::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .and_then(|r| {
                    Offset::from_str(split[2])
                        .map_err(|_| ParseInstructionError(String::from(s)))
                        .map(|o| Instruction::jie(r, o))
                }),

            "jio" if split.len() == 3 => char::from_str(split[1])
                .map_err(|_| ParseInstructionError(String::from(s)))
                .and_then(|r| {
                    Offset::from_str(split[2])
                        .map_err(|_| ParseInstructionError(String::from(s)))
                        .map(|o| Instruction::jio(r, o))
                }),

            _ => Err(ParseInstructionError(String::from(s))),
        }
    }
}

fn parse_code(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l))
        .filter_map(|r| r.ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day23::Instruction::*;

    impl PartialEq for ParseInstructionError {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl PartialEq for Instruction {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (hlf(r1), hlf(r2)) => r1 == r2,
                (tpl(r1), tpl(r2)) => r1 == r2,
                (inc(r1), inc(r2)) => r1 == r2,
                (jmp(o1), jmp(o2)) => o1 == o2,
                (jie(r1, o1), jie(r2, o2)) => r1 == r2 && o1 == o2,
                (jio(r1, o1), jio(r2, o2)) => r1 == r2 && o1 == o2,
                (_, _) => false,
            }
        }
    }

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(Instruction::from_str("hlf a").unwrap(), hlf('a'));
        assert_eq!(Instruction::from_str(" hlf   a  ").unwrap(), hlf('a'));
        assert_eq!(Instruction::from_str("tpl b").unwrap(), tpl('b'));
        assert_eq!(Instruction::from_str("inc c").unwrap(), inc('c'));
        assert_eq!(Instruction::from_str("jmp 1").unwrap(), jmp(1));
        assert_eq!(Instruction::from_str("jmp +2").unwrap(), jmp(2));
        assert_eq!(Instruction::from_str("jmp -3").unwrap(), jmp(-3));
        assert_eq!(Instruction::from_str("jmp -3").unwrap(), jmp(-3));
        assert_eq!(Instruction::from_str("jie  a, +4").unwrap(), jie('a', 4));
        assert_eq!(Instruction::from_str("jie b, 3").unwrap(), jie('b', 3));
        assert_eq!(Instruction::from_str("jie  c, -2").unwrap(), jie('c', -2));
        assert_eq!(Instruction::from_str("jio a, +4").unwrap(), jio('a', 4));
        assert_eq!(Instruction::from_str("jio  b, 3").unwrap(), jio('b', 3));
        assert_eq!(Instruction::from_str("jio c, -2").unwrap(), jio('c', -2));

        // PartialEq for Instruction
        assert_ne!(Instruction::from_str("jio c, -2").unwrap(), inc('c'));

        // PartialEq for ParseInstructionError
        assert_eq!(
            Instruction::from_str("jio err, -2").unwrap_err(),
            ParseInstructionError(String::from("jio err, -2"))
        );

        // only xxx y or xxx y, z
        assert_eq!(
            Instruction::from_str("jio a, b, c, d").unwrap_err(),
            ParseInstructionError(String::from("jio a, b, c, d"))
        );
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", Instruction::from_str("hlf a").unwrap()),
            "hlf('a')"
        );
        assert_eq!(
            format!("{:?}", Instruction::from_str("tpl b").unwrap()),
            "tpl('b')"
        );
        assert_eq!(
            format!("{:?}", Instruction::from_str("inc c").unwrap()),
            "inc('c')"
        );
        assert_eq!(
            format!("{:?}", Instruction::from_str("jmp 1").unwrap()),
            "jmp(1)"
        );
        assert_eq!(
            format!("{:?}", Instruction::from_str("jie b, 3").unwrap()),
            "jie('b', 3)"
        );
        assert_eq!(
            format!("{:?}", Instruction::from_str("jio  b, 3").unwrap()),
            "jio('b', 3)"
        );

        assert_eq!(
            format!("{:?}", Instruction::from_str("jpa  b, 3").unwrap_err()),
            "ParseInstructionError(\"jpa  b, 3\")"
        );

        assert_eq!(
            format!("{:?}", Instruction::from_str("jio err, -2").unwrap_err()),
            "ParseInstructionError(\"jio err, -2\")"
        );
    }

    #[test]
    fn test_parse_code() {
        let input = r#"jio a, +19
                       inc a
                       jop 123"#;
        let instructions = parse_code(input);
        assert_eq!(instructions[0], Instruction::jio('a', 19));
        assert_eq!(instructions[1], Instruction::inc('a'));
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_machine_1() {
        let input = r#"inc a"#;
        let mut machine = Machine::new(parse_code(input));
        let result1 = machine.do_evaluate();
        let result2 = machine.do_evaluate();
        assert!(result1);
        assert!(!result2);
        assert_eq!(machine.position, 1);
        assert_eq!(machine.registers.get(&'a'), Some(&1));
    }

    #[test]
    fn test_machine_2() {
        let input = r#"inc a
                  inc a
                  hlf a"#;
        let mut machine = Machine::new(parse_code(input));
        machine.do_evaluate(); // a == 1
        machine.do_evaluate(); // a == 2
        assert_eq!(machine.registers.get(&'a'), Some(&2));
        machine.do_evaluate(); // a == 1
        assert_eq!(machine.registers.get(&'a'), Some(&1));
    }

    #[test]
    fn test_machine_3() {
        let input = r#"inc a
                  inc a
                  tpl a"#;
        let mut machine = Machine::new(parse_code(input));
        machine.do_evaluate(); // a == 1
        machine.do_evaluate(); // a == 2
        assert_eq!(machine.registers.get(&'a'), Some(&2));
        machine.do_evaluate(); // a == 6
        assert_eq!(machine.registers.get(&'a'), Some(&6));
    }

    #[test]
    fn test_machine_4() {
        let input = r#"jmp +2
                  inc a
                  tpl a"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 2
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // a == 0 * 3
        assert_eq!(machine.registers.get(&'a'), Some(&0));
    }

    #[test]
    fn test_machine_4_overflow() {
        let input = r#"jmp +2
                  inc a"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        let result = machine.do_evaluate(); // position == 0 + 2
        assert!(result);
        assert_eq!(machine.position, 2);
    }

    #[test]
    fn test_machine_5() {
        let input = r#"inc a
                  inc a
                  jmp -2"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 1
        assert_eq!(machine.position, 1);
        machine.do_evaluate(); // position == 1 + 1
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // position == 2 - 2
        assert_eq!(machine.position, 0);
        assert_eq!(machine.registers.get(&'a'), Some(&2));
    }

    #[test]
    fn test_machine_6a() {
        let input = r#"inc a
                  inc a
                  jie a, -2"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 1
        assert_eq!(machine.position, 1);
        machine.do_evaluate(); // position == 1 + 1
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // position == 2 - 2
        assert_eq!(machine.position, 0);
        assert_eq!(machine.registers.get(&'a'), Some(&2));
    }

    #[test]
    fn test_machine_6b() {
        let input = r#"inc a
                  inc b
                  jie a, -2"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 1
        assert_eq!(machine.position, 1);
        machine.do_evaluate(); // position == 1 + 1
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // position == 2 + 1
        assert_eq!(machine.position, 3);
        assert_eq!(machine.registers.get(&'a'), Some(&1));
        assert_eq!(machine.registers.get(&'b'), Some(&1));
    }

    #[test]
    fn test_machine_7a() {
        let input = r#"inc a
                  inc a
                  jio a, -2"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 1
        assert_eq!(machine.position, 1);
        machine.do_evaluate(); // position == 1 + 1
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // position == 2 + 1
        assert_eq!(machine.position, 3);
        assert_eq!(machine.registers.get(&'a'), Some(&2));
    }

    #[test]
    fn test_machine_7b() {
        let input = r#"inc a
                  inc b
                  jio a, -2"#;
        let mut machine = Machine::new(parse_code(input));
        assert_eq!(machine.position, 0);
        machine.do_evaluate(); // position == 0 + 1
        assert_eq!(machine.position, 1);
        machine.do_evaluate(); // position == 1 + 1
        assert_eq!(machine.position, 2);
        machine.do_evaluate(); // position == 2 - 2
        assert_eq!(machine.position, 0);
        assert_eq!(machine.registers.get(&'a'), Some(&1));
        assert_eq!(machine.registers.get(&'b'), Some(&1));
    }
}
