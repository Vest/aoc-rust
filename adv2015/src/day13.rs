use std::collections::{HashMap, HashSet};
use permute::permutations_of;
use std::cmp::max;

pub fn get_answer(input: &str) -> i32 {
    calculate_everyone(input)
}

pub fn get_answer_with_me(input: &str) -> i32 {
    calculate_everyone_and_me(input)
}

struct PersonToPerson {
    who: String,
    next: String,
    attitude: i32,
}

fn parse_line(input: &str) -> PersonToPerson {
    let mut split = input.split_whitespace();
    let who = split.next().unwrap(); // name
    split.next(); // would
    let verb = split.next().unwrap(); // gain/lose
    let attitude = split.next().unwrap().parse::<i32>().unwrap() * if verb == "lose" { -1 } else { 1 }; // amount * verb
    split.next(); // happiness
    split.next(); // units
    split.next(); // by
    split.next(); // sitting
    split.next(); // next
    split.next(); // to
    let mut next = String::from(split.next().unwrap()); // name, with . at the end.
    next.pop();

    PersonToPerson {
        who: String::from(who),
        next,
        attitude,
    }
}

fn parse_input(input: &str) -> Vec<PersonToPerson> {
    let vec: Vec<PersonToPerson> = input.lines()
        .map(|line| parse_line(line))
        .collect();

    vec
}

fn build_relationships(input: &Vec<PersonToPerson>) -> HashMap<(String, String), i32> {
    let mut hash_map: HashMap<(String, String), i32> = HashMap::with_capacity(10);

    for person in input {
        hash_map.insert((person.who.clone(), person.next.clone()), person.attitude);
    }

    hash_map
}

fn extract_names(input: &Vec<PersonToPerson>) -> HashSet<String> {
    let mut hash_set: HashSet<String> = HashSet::with_capacity(5);

    input.iter().for_each(|person| { hash_set.insert(person.who.clone()); });

    hash_set
}

fn calculate_happiness(people: &Vec<String>, relationship: &HashMap<(String, String), i32>) -> i32 {
    let mut result = 0i32;
    let mut previous = String::from(people.last().unwrap());

    for current in people.iter() {
        result += relationship[&(previous.clone(), current.clone())];
        result += relationship[&(current.clone(), previous.clone())];
        previous = current.clone();
    }

    result
}

fn calculate_happiness_with_me(people: &Vec<String>, relationship: &HashMap<(String, String), i32>) -> i32 {
    let mut result = 0i32;
    let mut previous = String::from(people.last().unwrap());

    for current in people.iter() {
        if current.eq("Me") || previous.eq("Me") {
            previous = current.clone();
            continue;
        }

        result += relationship[&(previous.clone(), current.clone())];
        result += relationship[&(current.clone(), previous.clone())];
        previous = current.clone();
    }

    result
}

fn calculate_everyone(input: &str) -> i32 {
    let people = parse_input(input);
    let relationship = build_relationships(&people);
    let table: Vec<String> = extract_names(&people).iter().map(|n| (*n).clone()).collect();

    let mut result = 0i32;

    for permutation in permutations_of(&table) {
        let guessed_table: Vec<String> = permutation.map(|n| (*n).clone()).collect();

        let current_happiness = calculate_happiness(&guessed_table, &relationship);
        result = max(result, current_happiness);
    }

    result
}

fn calculate_everyone_and_me(input: &str) -> i32 {
    let people = parse_input(input);
    let relationship = build_relationships(&people);
    let mut table: Vec<String> = extract_names(&people).iter().map(|n| (*n).clone()).collect();
    table.push("Me".to_string());

    let mut result = 0i32;

    for permutation in permutations_of(&table) {
        let guessed_table: Vec<String> = permutation.map(|n| (*n).clone()).collect();

        let current_happiness = calculate_happiness_with_me(&guessed_table, &relationship);
        result = max(result, current_happiness);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Alice would gain 54 happiness units by sitting next to Bob.
                                   Alice would lose 79 happiness units by sitting next to Carol.
                                   Alice would lose 2 happiness units by sitting next to David.
                                   Bob would gain 83 happiness units by sitting next to Alice.
                                   Bob would lose 7 happiness units by sitting next to Carol.
                                   Bob would lose 63 happiness units by sitting next to David.
                                   Carol would lose 62 happiness units by sitting next to Alice.
                                   Carol would gain 60 happiness units by sitting next to Bob.
                                   Carol would gain 55 happiness units by sitting next to David.
                                   David would gain 46 happiness units by sitting next to Alice.
                                   David would lose 7 happiness units by sitting next to Bob.
                                   David would gain 41 happiness units by sitting next to Carol."#;

    #[test]
    fn test_parse_line_1() {
        let result = parse_line("Alice would gain 54 happiness units by sitting next to Bob.");
        assert_eq!(result.who, "Alice");
        assert_eq!(result.next, "Bob");
        assert_eq!(result.attitude, 54);
    }

    #[test]
    fn test_parse_line_2() {
        let result = parse_line("Alice would lose 79 happiness units by sitting next to Carol.");
        assert_eq!(result.who, "Alice");
        assert_eq!(result.next, "Carol");
        assert_eq!(result.attitude, -79);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);
        let single_line = result.get(10).unwrap();

        assert_eq!(result.len(), 12);
        assert_eq!(single_line.who, "David");
        assert_eq!(single_line.next, "Bob");
        assert_eq!(single_line.attitude, -7);
    }

    #[test]
    fn test_build_relationships() {
        let result = parse_input(INPUT);
        let hash_map = build_relationships(&result);

        let key = (String::from("David"), String::from("Bob")); // David -> Bob

        assert_eq!(hash_map.len(), 12);
        assert_eq!(hash_map[&key], -7);
    }

    #[test]
    fn test_extract_names() {
        let result = parse_input(INPUT);
        let people = extract_names(&result);

        assert_eq!(people.len(), 4);
        assert!(people.contains("Alice"));
        assert!(people.contains("Bob"));
        assert!(people.contains("Carol"));
        assert!(people.contains("David"));
    }

    #[test]
    fn test_calculate_happiness() {
        let people = parse_input(INPUT);
        let hash_map = build_relationships(&people);
        let table: Vec<String> = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string(), "David".to_string()];

        let result = calculate_happiness(&table, &hash_map);
        assert_eq!(result, 330);
    }

    #[test]
    fn test_calculate_everyone() {
        let result = calculate_everyone(INPUT);
        assert_eq!(result, 330);
    }
}