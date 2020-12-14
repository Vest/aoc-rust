use std::collections::HashMap;

pub fn find_answer1(input: &str) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = String::new();

    input.lines()
        .map(&str::trim)
        .for_each(|line: &str| {
            if line.starts_with("mask") {
                current_mask = extract_mask(line);
            } else {
                let (address, mut value) = extract_address_value(line);
                value = apply_mask(value, current_mask.as_str());
                mem.insert(address, value);
            }
        });

    mem.values()
        .sum()
}

fn extract_mask(input: &str) -> String {
    String::from(&input[7..])
}

fn extract_address_value(input: &str) -> (usize, usize) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let address = tokens[0][4..tokens[0].len() - 1].parse::<usize>().unwrap();
    let value = tokens[2].parse::<usize>().unwrap();

    (address, value)
}

fn apply_mask(mut value: usize, bitmask: &str) -> usize {
    bitmask.chars()
        .rev()
        .enumerate()
        .for_each(|(position, bit)| {
            match bit {
                '0' => {
                    value &= !(1 << position);
                }
                '1' => {
                    value |= (1 << position);
                }
                _ => (),
            }
        });

    value
}

pub fn find_answer2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(""), 0);
        assert_eq!(find_answer2(""), 0);
    }

    #[test]
    fn test_extract_mask() {
        assert_eq!(extract_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    }

    #[test]
    fn test_extract_address_value() {
        assert_eq!(extract_address_value("mem[23073] = 4721114"), (23073, 4721114));
    }


    #[test]
    fn test_apply_mask() {
        assert_eq!(apply_mask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 73);
        assert_eq!(apply_mask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 101);
        assert_eq!(apply_mask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 64);
    }

    #[test]
    fn test_find_answer1() {
        assert_eq!(find_answer1(r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                                   mem[8] = 11
                                   mem[7] = 101
                                   mem[8] = 0"#), 165);
    }
}
