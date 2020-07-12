use std::collections::HashMap;

pub fn get_answer(input: &str) -> i32 {
    0
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
    let mut hashMap: HashMap<(String, String), i32> = HashMap::with_capacity(10);

    for person in input {
        hashMap.insert((person.who.clone(), person.next.clone()), person.attitude);
    }

    hashMap
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
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

        let result = parse_input(input);
        let single_line = result.get(10).unwrap();

        assert_eq!(result.len(), 12);
        assert_eq!(single_line.who, "David");
        assert_eq!(single_line.next, "Bob");
        assert_eq!(single_line.attitude, -7);
    }

    #[test]
    fn test_build_relationships() {}
}