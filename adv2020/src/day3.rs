pub fn answer1(input: &str) -> usize {
    let input = parse_input(input);
    let mut x_pos = 0usize;
    let mut count = 0usize;
    for line in input {
        if line.chars().nth(x_pos % line.len()).unwrap() == '#' {
            count += 1;
        }
        x_pos += 3;
    }

    count
}

pub fn answer2(input: &str) -> usize {
    let input = parse_input(input);

    let mut x_pos_1 = 0usize;
    let mut x_pos_3 = 0usize;
    let mut x_pos_5 = 0usize;
    let mut x_pos_7 = 0usize;

    let mut x_pos_2 = 0usize;
    let mut y_pos_2 = 2usize;

    let mut count_1 = 0usize;
    let mut count_3 = 0usize;
    let mut count_5 = 0usize;
    let mut count_7 = 0usize;
    let mut count_2_down = 0usize;

    for line in input.iter().enumerate() {
        if line.1.chars().nth(x_pos_1 % line.1.len()).unwrap() == '#' {
            count_1 += 1;
        }
        if line.1.chars().nth(x_pos_3 % line.1.len()).unwrap() == '#' {
            count_3 += 1;
        }
        if line.1.chars().nth(x_pos_5 % line.1.len()).unwrap() == '#' {
            count_5 += 1;
        }
        if line.1.chars().nth(x_pos_7 % line.1.len()).unwrap() == '#' {
            count_7 += 1;
        }

        if line.0 == y_pos_2 {
            x_pos_2 += 1;
            y_pos_2 += 2;
            if line.1.chars().nth(x_pos_2 % line.1.len()).unwrap() == '#' {
                count_2_down += 1;
            }
        }

        x_pos_1 += 1;
        x_pos_3 += 3;
        x_pos_5 += 5;
        x_pos_7 += 7;
    }

    count_1 * count_3 * count_5 * count_7 * count_2_down
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| String::from(l.trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        assert_eq!(answer1(""), 0);
        assert_eq!(answer2(""), 0);
    }
}
