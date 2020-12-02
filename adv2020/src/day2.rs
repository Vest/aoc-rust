pub fn answer1(input: &str) -> usize {
    let i = parse_input(input);

    i.iter()
        .filter(|rule| {
            let actual_count = rule.password.chars()
                .filter(|c| *c == rule.letter)
                .count();
            actual_count >= rule.from && actual_count <= rule.to
        }).count()
}

pub fn answer2(input: &str) -> usize {
    let i = parse_input(input);

    i.iter()
        .filter(|rule| {
            let actual_count: Vec<char> = rule.password.chars().collect();
            if actual_count[rule.from - 1] == rule.letter || actual_count[rule.to - 1] == rule.letter {
                actual_count[rule.from - 1] != actual_count[rule.to - 1]
            } else { false }
        }).count()
}

struct Rule {
    from: usize,
    to: usize,
    letter: char,
    password: String,
}

fn parse_input(input: &str) -> Vec<Rule> {
    input.lines()
        .map(|l| {
            let v: Vec<&str> = l.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation()
            ).filter(|s| !s.is_empty())
                .collect();

            let from = v[0].parse::<usize>().unwrap();
            let to = v[1].parse::<usize>().unwrap();

            Rule {
                from,
                to,
                letter: v[2].parse::<char>().unwrap(),
                password: String::from(v[3]),
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input("17-18 f: fffffffffffffffffff");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, 17);
        assert_eq!(result[0].to, 18);
        assert_eq!(result[0].letter, 'f');
        assert_eq!(result[0].password, "fffffffffffffffffff");
    }
}

