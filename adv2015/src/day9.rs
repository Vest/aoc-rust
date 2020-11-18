use combination::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::cmp;

pub fn calc_shortest(input: &str) -> usize {
    let mut santa = Santa::new();

    input.lines()
        .for_each(|line| {
            let path = parse_to_path(line);
            santa.add_path(&path);
        });

    santa.find_path().0
}

pub fn calc_longest(input: &str) -> usize {
    let mut santa = Santa::new();

    input.lines()
        .for_each(|line| {
            let path = parse_to_path(line);
            santa.add_path(&path);
        });

    santa.find_path().1
}

struct Santa {
    cities: HashSet<String>,
    distances: HashMap<(String, String), usize>,
}

struct Path(String, String, usize);

impl Santa {
    fn new() -> Santa {
        Santa {
            cities: HashSet::new(),
            distances: HashMap::new(),
        }
    }

    fn add_path(&mut self, path: &Path) {
        let Path(city1, city2, distance) = path;

        self.cities.insert(city1.clone());
        self.cities.insert(city2.clone());
        self.distances.insert((city1.clone(), city2.clone()), *distance);
        self.distances.insert((city2.clone(), city1.clone()), *distance);
    }

    fn find_path(&self) -> (usize, usize) {
        let vector: Vec<&String> = Vec::from_iter(&self.cities);
        let mut min_distance = usize::max_value();
        let mut max_distance = usize::min_value();
        for permutation in permutate::permutate_vec(&vector) {
            let mut current_distance = 0usize;
            let mut prev_city = String::new();

            for city in permutation {
                if prev_city.is_empty() {
                    prev_city = (*city).clone();
                    continue;
                }

                let path = *self.distances.get(&(prev_city.clone(), (*city).clone())).unwrap();
                prev_city = (*city).clone();

                current_distance += path;
            }

            min_distance = cmp::min(min_distance, current_distance);
            max_distance = cmp::max(max_distance, current_distance);
        }

        (min_distance, max_distance)
    }
}

fn parse_to_path(input: &str) -> Path {
    let split: Vec<&str> = input.split_whitespace().collect();

    Path(split[0].to_string(), split[2].to_string(), split[4].parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_path() {
        let input = "London to Dublin = 464";
        let Path(city1, city2, distance) = parse_to_path(input);
        assert_eq!(city1, "London");
        assert_eq!(city2, "Dublin");
        assert_eq!(distance, 464);
    }

    #[test]
    fn test_santa() {
        let mut santa = Santa::new();
        "London to Dublin = 464
         London to Belfast = 518
         Dublin to Belfast = 141"
            .lines().for_each(|line| {
            let path = parse_to_path(line);
            santa.add_path(&path);
        });

        assert_eq!(santa.cities.len(), 3);
        assert_eq!(santa.distances.len(), 6);

        assert_eq!(santa.find_path(), (605, 982));

        for i in santa.distances {
            println!("{} {} = {}", (i.0).0, (i.0).1, i.1)
        }
    }

    #[test]
    fn test_routes() {
        const INPUT: &str = "London to Dublin = 464
         London to Belfast = 518
         Dublin to Belfast = 141";
        assert_eq!(calc_shortest(INPUT), 605);
        assert_eq!(calc_longest(INPUT), 982);
    }
}
