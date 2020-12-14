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
                    value |= 1 << position;
                }
                _ => (),
            }
        });

    value
}

fn create_mask(mut address: usize, bitmask: &str) -> String {
    bitmask.chars()
        .rev()
        .enumerate()
        .map(|(position, bit)| {
            match bit {
                '0' => {
                    match (address >> position) & 1 {
                        0 => '0',
                        _ => '1',
                    }
                }
                '1' => '1',
                _ => 'X',
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

fn generate_addresses(bitmask: &str) -> Vec<String> {
    let total = bitmask.chars().filter(|c| *c == 'X').count();

    (0..2usize.pow(total as u32)).map(|i| {
        let xs: Vec<usize> = bitmask.chars()
            .enumerate()
            .filter(|(pos, c)| *c == 'X')
            .map(|(pos, _)| pos)
            .collect();

        let mut j = 0usize;

        bitmask.chars()
            .map(|c| match c {
                'X' => {
                    let bit = (i & 1 << j) >> j;
                    j += 1;
                    if bit == 0 { '0' } else { '1' }
                }
                _ => c,
            }).collect()
    }).collect()
}

pub fn find_answer2(input: &str) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: String = String::new();

    input.lines()
        .map(&str::trim)
        .for_each(|line: &str| {
            if line.starts_with("mask") {
                mask = extract_mask(line);
            } else {
                let (address, mut value) = extract_address_value(line);
                let addresses = generate_addresses(create_mask(address, mask.as_str()).as_str());
                addresses.iter()
                    .for_each(|addr| {
                        mem.insert(bit_to_usize(addr), value);
                    })
            }
        });

    mem.values()
        .sum()
}

fn bit_to_usize(input: &str) -> usize {
    input.chars()
        .fold(0, |acc, bit_char| acc << 1 | if bit_char == '0' { 0 } else { 1 })
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

    #[test]
    fn test_create_mask() {
        assert_eq!(create_mask(42, "000000000000000000000000000000X1001X"), "000000000000000000000000000000X1101X");
        assert_eq!(create_mask(26, "00000000000000000000000000000000X0XX"), "00000000000000000000000000000001X0XX");
    }

    #[test]
    fn test_generate_addresses() {
        let result = generate_addresses("00000000000000000000000000000001X0XX");
        assert_eq!(result.len(), 2_usize.pow(3));
        assert!(result.contains(&"000000000000000000000000000000010000".to_string()));
        assert!(result.contains(&"000000000000000000000000000000010001".to_string()));
        assert!(result.contains(&"000000000000000000000000000000010010".to_string()));
        assert!(result.contains(&"000000000000000000000000000000010011".to_string()));
        assert!(result.contains(&"000000000000000000000000000000011000".to_string()));
        assert!(result.contains(&"000000000000000000000000000000011001".to_string()));
        assert!(result.contains(&"000000000000000000000000000000011010".to_string()));
        assert!(result.contains(&"000000000000000000000000000000011011".to_string()));
    }

    #[test]
    fn test_find_answer2() {
        assert_eq!(find_answer2(r#"mask = 000000000000000000000000000000X1001X
                                   mem[42] = 100
                                   mask = 00000000000000000000000000000000X0XX
                                   mem[26] = 1"#), 208);
    }

    #[test]
    fn test_bit_to_usize() {
        assert_eq!(bit_to_usize("000000000000000000000000000000010000"), 16);
        assert_eq!(bit_to_usize("000000000000000000000000000000010001"), 17);
        assert_eq!(bit_to_usize("000000000000000000000000000000010010"), 18);
        assert_eq!(bit_to_usize("000000000000000000000000000000010011"), 19);
        assert_eq!(bit_to_usize("000000000000000000000000000000011000"), 24);
        assert_eq!(bit_to_usize("000000000000000000000000000000011001"), 25);
        assert_eq!(bit_to_usize("000000000000000000000000000000011010"), 26);
        assert_eq!(bit_to_usize("000000000000000000000000000000011011"), 27);
    }
}
