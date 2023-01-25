use itertools::Itertools;
use std::collections::HashMap;

pub fn find_answer1(input: &str) -> usize {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();

    input.lines().for_each(|line| {
        let line_split: Vec<&str> = line.split("contain").collect();
        let what = line_split[0].split_whitespace().take(2).join(" ");
        let mut map: HashMap<String, usize> = HashMap::new();
        let bags_split: Vec<&str> = line_split[1].split(", ").collect();
        if !bags_split.contains(&"no other") {
            for bag in bags_split {
                let bag = bag.split_whitespace().take(3).join(" ");
                let num = bag
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or_default();
                let color = bag.split_whitespace().skip(1).take(2).join(" ");
                map.insert(color, num);
            }
        }

        rules.insert(what, map);
    });

    let mut result = 0;

    for (key, _) in rules.iter() {
        if contains(&rules, key, &String::from("shiny gold")) {
            result += 1;
        }
    }
    result
}

fn contains(rules: &HashMap<String, HashMap<String, usize>>, key: &String, what: &String) -> bool {
    if rules.contains_key(key) {
        let nested_bags = rules.get(key).unwrap();
        if nested_bags.is_empty() {
            return false;
        } else {
            let mut result = false;

            for (k, _) in nested_bags {
                if nested_bags.contains_key(what) {
                    return true;
                }

                result = result | contains(rules, k, what);

                if result {
                    return true;
                }
            }

            return result;
        }
    }

    false
}

fn count_bags(rules: &HashMap<String, HashMap<String, usize>>, what: &String) -> usize {
    if !rules.contains_key(what) {
        return 0;
    }

    let bags = rules.get(what).unwrap();
    if bags.is_empty() {
        return 0;
    }

    let mut result = 0usize;
    for (small_bag, count) in bags {
        result += count_bags(rules, small_bag) * count + count;
    }

    result
}

pub fn find_answer2(input: &str) -> usize {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();

    input.lines().for_each(|line| {
        let line_split: Vec<&str> = line.split("contain").collect();
        let what = line_split[0].split_whitespace().take(2).join(" ");
        let mut map: HashMap<String, usize> = HashMap::new();
        let bags_split: Vec<&str> = line_split[1].split(", ").collect();
        if !bags_split.contains(&"no other") {
            for bag in bags_split {
                let bag = bag.split_whitespace().take(3).join(" ");
                let num = bag
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or_default();
                let color = bag.split_whitespace().skip(1).take(2).join(" ");
                map.insert(color, num);
            }
        }

        rules.insert(what, map);
    });

    count_bags(&rules, &String::from("shiny gold"))
}
/*
fn parse_input<'a>(input: &'a str) -> impl Iterator<Item=Seat> + 'a {
    input.lines()
        .map(&str::trim)
        .map(parse_seat)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(""), 0);
        assert_eq!(find_answer2(""), 0);
    }
}
