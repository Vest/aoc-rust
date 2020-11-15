use std::str::FromStr;

pub fn get_answer(input: &str) -> usize {
    0
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

#[derive(Debug)]
struct ParseInstructionError(String);

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim()
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

            _ => Err(ParseInstructionError(String::from(s)))
        }
    }
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
                (_, _) => false
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
        assert_eq!(Instruction::from_str("jio err, -2").unwrap_err(), ParseInstructionError(String::from("jio err, -2")));

        // only xxx y or xxx y, z
        assert_eq!(Instruction::from_str("jio a, b, c, d").unwrap_err(), ParseInstructionError(String::from("jio a, b, c, d")));
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Instruction::from_str("hlf a").unwrap()), "hlf('a')");
        assert_eq!(format!("{:?}", Instruction::from_str("tpl b").unwrap()), "tpl('b')");
        assert_eq!(format!("{:?}", Instruction::from_str("inc c").unwrap()), "inc('c')");
        assert_eq!(format!("{:?}", Instruction::from_str("jmp 1").unwrap()), "jmp(1)");
        assert_eq!(format!("{:?}", Instruction::from_str("jie b, 3").unwrap()), "jie('b', 3)");
        assert_eq!(format!("{:?}", Instruction::from_str("jio  b, 3").unwrap()), "jio('b', 3)");

        assert_eq!(format!("{:?}", Instruction::from_str("jpa  b, 3").unwrap_err()), "ParseInstructionError(\"jpa  b, 3\")");

        assert_eq!(format!("{:?}", Instruction::from_str("jio err, -2").unwrap_err()), "ParseInstructionError(\"jio err, -2\")");

    }
}
