use permute::*;
use std::iter::FromIterator;
use std::collections::HashSet;

const REQUIRED_VOLUME: usize = 150;

pub fn get_answer(input: &str) -> usize {
    let available_cans = parse_to_available_cans(input);
    let found_cans = find_cans(&available_cans, REQUIRED_VOLUME);

    found_cans.len()
}

fn parse_to_available_cans(input: &str) -> Vec<(usize, usize)> {
    input.lines().enumerate()
        .map(|l| (l.0, l.1.trim()))
        .map(|l| (l.0, l.1.parse::<usize>()))
        .filter(|c| c.1.is_ok())
        .map(|c| (c.0, c.1.unwrap()))
        .collect()
}

fn find_cans(vec: &Vec<(usize, usize)>, required_volume: usize) -> HashSet<Vec<(usize, usize)>> {
    let size = vec.len();
    let mut hash_set: HashSet<Vec<(usize, usize)>> = HashSet::new();

    for permutation in permutations_of(vec) {
        let permutation_vector: Vec<(usize, usize)> = permutation.cloned().collect();

        for c in 1..size {
            let mut candidate: Vec<(usize, usize)> = Vec::from_iter(permutation_vector[0..c].iter().cloned());
            candidate.sort_by(|c1, c2| c1.0.cmp(&(c2.0)));

            if hash_set.contains(&candidate) {
                continue;
            }

            let slice: Vec<usize> = candidate.iter()
                .map(|c| c.1)
                .collect();
            let volume = calc_volume_vec(&slice);

            if volume == required_volume {
                println!("{:?}", candidate);
                hash_set.insert(candidate);
            }
        }
    }

    hash_set
}

fn calc_volume_vec(cans: &Vec<usize>) -> usize {
    cans.into_iter().sum()
}


fn combinations<T: Clone>(vec: &Vec<T>, number: usize) -> Vec<Vec<T>> {
    let vec_len = vec.len();
    let mut vec = Vec::with_capacity(factorial(vec_len) / factorial(number) / factorial(vec_len - number));



    vec
}

fn factorial(num: usize) -> usize {
    if num == 2 { 2 } else { num * factorial(num - 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cans() {
        let available_cans = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let found_cans = find_cans(&available_cans, 5);

        assert_eq!(found_cans.len(), 3)
    }

    #[test]
    fn test_find_cans_example() {
        let available_cans = vec![(0, 20), (1, 15), (2, 10), (3, 5), (4, 5)];
        let found_cans = find_cans(&available_cans, 25);

        assert_eq!(found_cans.len(), 4);
    }

    #[test]
    fn test_calc_volume_vec() {
        assert_eq!(calc_volume_vec(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(calc_volume_vec(&vec![3, 4, 5]), 12);
    }

    #[test]
    fn test_parse_to_available_cans() {
        let cans = parse_to_available_cans(r#"20
        15
        10
        5
        5"#);

        assert_eq!(cans.len(), 5);

        assert_eq!(cans[0].0, 0);
        assert_eq!(cans[0].1, 20);

        assert_eq!(cans[4].0, 4);
        assert_eq!(cans[4].1, 5);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }
}
