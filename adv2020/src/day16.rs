use itertools::Itertools;

struct Validation<'a> {
    name: &'a str,
    rules: Vec<(usize, usize)>,
}

impl Validation<'_> {
    fn is_valid(&self, number: usize) -> bool {
        self.rules.iter()
            .find(|rule| (rule.0..=rule.1).contains(&number))
            .is_some()
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
    let validations_count = validations.len();

    lines.find(|&line| line == "your ticket:");

    let _your_ticket = parse_numbers(lines.next().unwrap());

    lines.find(|&line| line == "nearby tickets:");

    let other_tickets = lines.map(parse_numbers)
        .map(|numbers: Vec<usize>| validate_all_rules(&numbers, &validations))
        .collect::<Vec<Vec<usize>>>();
    let _other_tickets_count = other_tickets.len();

    let _order_validators = validations.iter()
        .permutations(validations_count)
        .find(|permutations| other_tickets.iter()
            .find(|&tickets| !validate_rules_ordered(tickets, permutations))
            .is_none())
        .unwrap();

    println!("Found order");
    0
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
        .find(|Validation { rules, .. }|
            rules.iter()
                .filter(|(from, to)| number >= *from && number <= *to)
                .count() > 0
        ).is_some()
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
}
