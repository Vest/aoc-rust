use std::collections::{HashMap, HashSet};

pub fn find_answer1(input: &str) -> usize {
    let mut result = 0usize;

    let mut set: HashSet<char> = HashSet::new();

    for line in input.lines() {
        if line.is_empty() {
            result += set.len();
            set.clear();
        } else {
            line.chars()
                .for_each(|c: char| {
                    set.insert(c);
                })
        }
    }

    result + set.len()
}

pub fn find_answer2(input: &str) -> usize {
    let mut result = 0usize;

    let mut map_stats: HashMap<char, usize> = HashMap::new();
    let mut lines = 0usize;

    for line in input.lines() {
        if line.is_empty() {
            result += map_stats.iter()
                .filter(|&(_, v)| *v == lines)
                .count();
            lines = 0;
            map_stats.clear();
        } else {
            line.chars()
                .for_each(|c: char| {
                    if map_stats.contains_key(&c) {
                        let value = map_stats.get(&c).unwrap();
                        let value = *value + 1;
                        map_stats.insert(c, value);
                    } else {
                        map_stats.insert(c, 1);
                    }
                });

            lines += 1;
        }
    }

    result + map_stats.iter()
        .filter(|&(_, v)| *v == lines)
        .count()
}
/*
struct AggregateIterator<'a, T: Iterator<Item=&'a str> + 'a> {
    input: T,
}

impl AggregateIterator<T> {
    fn new(input: &str) -> AggregateIterator<T> {
        AggregateIterator {
            input: input.lines()
        }
    }
}

impl Iterator for AggregateIterator<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec: Vec<String> = Vec::new();

        for str in self.input.map(&str::trim) {
            if str.is_empty {
                break;
            }
            vec.append(str);
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec.join(" "))
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(r#"abc

a
b
c

ab
ac

a
a
a
a

b"#), 11);
        assert_eq!(find_answer2(""), 0);
    }
}
