use itertools::Itertools;
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Validation<'a> {
    name: &'a str,
    rules: Vec<(usize, usize)>,
}

impl Validation<'_> {
    fn is_valid(&self, number: usize) -> bool {
        self.rules.iter()
            .any(|rule| (rule.0..=rule.1).contains(&number))
    }
}

pub fn find_answer1(input: &str) -> usize {
    let mut lines = input.lines();
    let validations = extract_rules(&mut lines);

    lines.find(|&line| line == "nearby tickets:");

    lines.map(parse_numbers)
        .map(|numbers: Vec<usize>| validate_all_rules(&numbers, &validations)
            .iter()
            .sum::<usize>())
        .sum::<usize>()
}

pub fn find_answer2(input: &str) -> usize {
    let mut lines = input.lines();
    let validations = extract_rules(&mut lines);

    lines.find(|&line| line == "your ticket:");

    let your_ticket = parse_numbers(lines.next().unwrap());

    lines.find(|&line| line == "nearby tickets:");

    let other_tickets = lines.map(parse_numbers)
        .filter(|numbers| numbers.iter()
            .all(|number| is_number_correct(*number, &validations))
        )
        .collect::<Vec<Vec<usize>>>();

    other_tickets.iter()
        .for_each(|ticket| println!("{:?}", ticket));

    let correct_combination = find_validation_combination(&other_tickets, &validations);

    correct_combination.iter()
        .enumerate()
        .filter(|&(_, rule)| rule.name.starts_with("departure"))
        .map(|(pos, _)| your_ticket[pos])
        .product()
}


fn extract_rules<'a>(lines: &mut dyn Iterator<Item=&'a str>) -> Vec<Validation<'a>> {
    let mut validations: Vec<Validation> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        if let Some(validation) = map_rule(line) {
            validations.push(validation);
        }
    }

    validations
}

fn map_rule(input: &str) -> Option<Validation> {
    let mut splitter = input.split(':');
    let name = splitter.next()?;
    let range = splitter.next()?.split("or")
        .map(&str::trim)
        .map(|part|
            part.split('-')
                .map(&str::parse::<usize>)
                .filter_map(Result::ok)
                .collect::<Vec<usize>>()
        )
        .filter_map(|parts| if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        })
        .collect::<Vec<(usize, usize)>>();

    if range.is_empty() {
        None
    } else {
        Some(Validation {
            name,
            rules: range,
        })
    }
}

fn validate_all_rules(vec: &Vec<usize>, validations: &Vec<Validation>) -> Vec<usize> {
    vec.iter()
        .filter(|&number| !is_number_correct(*number, validations))
        .map(|v| *v)
        .collect::<Vec<usize>>()
}

fn is_number_correct(number: usize, validations: &Vec<Validation>) -> bool {
    validations.iter()
        .any(|Validation { rules, .. }|
            rules.iter()
                .any(|&(from, to)| (from..=to).contains(&number))
        )
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input.split(',')
        .map(&str::parse::<usize>)
        .filter_map(Result::ok)
        .collect::<Vec<usize>>()
}

fn validate_rules_ordered(numbers: &Vec<usize>, validations: &Vec<&Validation>) -> bool {
    if numbers.len() != validations.len() {
        return false;
    }

    numbers.iter()
        .zip(validations)
        .find(|(&num, &validation)| !validation.is_valid(num))
        .is_none()
}

fn find_validation_combination<'a>(tickets: &Vec<Vec<usize>>, validations: &'a Vec<Validation<'a>>) -> Vec<&'a Validation<'a>> {
    let validations_count = validations.len();

    let mut shuffled = validations.iter().collect::<Vec<&Validation>>();


    let mut valid_rules = HashMap::<usize, &Validation>::new();

    while valid_rules.len() != validations_count {
        'another_rule: for rule in &shuffled {
            'position: for pos in 0..validations_count {
                if valid_rules.contains_key(&pos) {
                    continue 'position;
                }
                if tickets.iter()
                    .all(|ticket| rule.is_valid(ticket[pos])) {
                    valid_rules.insert(pos, rule);
                    continue 'another_rule;
                }
            }
            println!("nikaque: {:?}", rule);
        }

        if valid_rules.len() != validations_count {
            println!("Couldn't find the order");
            shuffled.shuffle(&mut thread_rng());
            valid_rules.clear();
        }
    }

    println!("Identified rules, count: {}, {:?}", valid_rules.len(), valid_rules);

    valid_rules.into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, rule)| rule)
        .collect()
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
    fn test_map_rule() {
        let result = map_rule("arrival platform: 42-729 or 751-959").unwrap();
        assert_eq!(result.name, "arrival platform");
        assert_eq!(result.rules.len(), 2);
        assert_eq!(result.rules[0], (42, 729));
        assert_eq!(result.rules[1], (751, 959));

        let result = map_rule("arrival: platform: 42-729 or 751-959");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_numbers() {
        let result = parse_numbers("7,3,47");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 7);
        assert_eq!(result[1], 3);
        assert_eq!(result[2], 47);
    }

    const INPUT: &'static str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    #[test]
    fn test_find_answer1() {
        let result = find_answer1(INPUT);

        assert_eq!(result, 71);
    }

    #[test]
    fn test_validate_all_rules() {
        let mut lines = INPUT.lines();
        let rules = extract_rules(&mut lines);
        let result = validate_all_rules(&vec![7, 3, 47], &rules);
        assert!(result.is_empty());

        let result = validate_all_rules(&vec![40, 4, 50], &rules);
        assert_eq!(result[0], 4);
    }

    #[test]
    fn test_extract_rules() {
        let mut lines = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50"#.lines();
        let result = extract_rules(&mut lines);

        assert_eq!(result.len(), 3);
        assert_eq!(result[2].name, "seat");
    }

    #[test]
    fn test_is_number_correct() {
        let mut lines = INPUT.lines();
        let rules = extract_rules(&mut lines);

        assert!(!is_number_correct(4, &rules));
        assert!(is_number_correct(40, &rules));
        assert!(is_number_correct(50, &rules));
    }

    #[test]
    fn test_validate_rules_ordered() {
        let mut lines = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19"#.lines();
        let rules = extract_rules(&mut lines);
        let ticket_1 = vec![3, 9, 18];
        let ticket_2 = vec![15, 1, 5];
        let ticket_3 = vec![5, 14, 9];

        let rule_1 = vec![&rules[0], &rules[1], &rules[2]];
        let rule_2 = vec![&rules[1], &rules[0], &rules[2]];

        assert!(!validate_rules_ordered(&ticket_1, &rule_1));
        assert!(validate_rules_ordered(&ticket_2, &rule_1));
        assert!(validate_rules_ordered(&ticket_3, &rule_1));

        assert!(validate_rules_ordered(&ticket_1, &rule_2));
        assert!(validate_rules_ordered(&ticket_2, &rule_2));
        assert!(validate_rules_ordered(&ticket_3, &rule_2));
    }

    #[test]
    fn test_find_validation_combination() {
        let mut lines = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19"#.lines();
        let rules = extract_rules(&mut lines);
        let tickets = vec![
            vec![3, 9, 18],
            vec![15, 1, 5],
            vec![5, 14, 9]
        ];

        let ordered_rules = find_validation_combination(&tickets, &rules);

        assert_eq!(ordered_rules.len(), rules.len());
        assert_eq!(ordered_rules[0].name, "row");
        assert_eq!(ordered_rules[1].name, "class");
        assert_eq!(ordered_rules[2].name, "seat");
    }
}
