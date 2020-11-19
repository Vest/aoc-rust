use combinations::Combinations;
use std::cmp::min;

pub fn find_answer(input: &str) -> usize {
    let packages = parse_packages(input);
    let weight = packages.iter().sum::<usize>() / 3;

    find_optimal_qe(&packages, weight)
}

pub fn find_answer_better(input: &str) -> usize {
    let packages = parse_packages(input);
    let weight = packages.iter().sum::<usize>() / 4;

    find_optimal_qe(&packages, weight)
}

fn find_optimal_qe(packages: &Vec<usize>, weight: usize) -> usize {
    let mut lowest_qe = usize::MAX;
    let mut lowest_count = usize::MAX;

    let sleigh = SleighCombination::new(&packages, weight);

    for group in sleigh {
        for filtered_group in group.filter(|group| group.iter().sum::<usize>() == weight) {
            let current_qe = calc_qe(&filtered_group);
            let current_count = filtered_group.len();

            if (lowest_qe == usize::MAX && lowest_count == usize::MAX)
                || (current_count < lowest_count && current_qe < lowest_qe) {
                let rest_packages = subtract_vectors(packages, &filtered_group);

                if group_with_weight_exists(&rest_packages, weight) {
                    lowest_qe = min(lowest_qe, current_qe);
                    lowest_count = min(lowest_count, current_count);
                    println!("Found something: {}, {}", lowest_qe, lowest_count);
                }
            }
        }

        if lowest_qe != usize::MAX {
            return lowest_qe;
        }
    }

    lowest_qe
}

fn parse_packages(input: &str) -> Vec<usize> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|p| p.parse::<usize>().ok())
        .collect()
}

fn create_groups(packages: &Vec<usize>, size: usize) -> Vec<Vec<usize>> {
    if !(1..=packages.len()).contains(&size) {
        return Vec::new();
    }

    let copy_packages = packages.to_vec();

    Combinations::new(copy_packages, size).collect()
}

fn create_groups_iter(packages: &Vec<usize>, size: usize) -> Combinations<usize> {
    let copy_packages = packages.to_vec();

    Combinations::new(copy_packages, size)
}

struct SleighCombination {
    size: usize,
    packages: Vec<usize>,
    weight: usize,
}

impl SleighCombination {
    fn new(packages: &Vec<usize>, weight: usize) -> SleighCombination {
        let copy_packages = packages.to_vec();

        SleighCombination {
            size: 0,
            weight,
            packages: copy_packages,
        }
    }
}

impl Iterator for SleighCombination {
    type Item = Combinations<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.size += 1;

        if self.size == self.packages.len() {
            return None;
        }

        let copy_packages = self.packages.to_vec();
        Some(Combinations::new(copy_packages, self.size))
    }
}

fn find_groups_with_weight(packages: &Vec<usize>, weight: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    if packages.is_empty() {
        return result;
    }

    for size in 1..packages.len() {
        create_groups_iter(&packages, size)
            .filter(|group| group.iter().sum::<usize>() == weight)
            .for_each(|group| {
                let copy_group = group.to_vec();
                result.push(copy_group);
            });
    }

    result
}

fn group_with_weight_exists(packages: &Vec<usize>, weight: usize) -> bool {
    for size in 1..packages.len() {
        if create_groups_iter(&packages, size)
            .any(|packages| packages.iter().sum::<usize>() == weight) {
            return true;
        }
    }

    false
}

// quantum entanglement
fn calc_qe(group: &Vec<usize>) -> usize {
    group.iter()
        .fold(1, |acc, p| acc * p)
}

fn subtract_vectors(from: &Vec<usize>, rhs: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();

    result.extend(from.iter()
        .filter(|i| !rhs.contains(i)));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packages() {
        let packages = parse_packages(r#"1
        2

        6"#);
        assert_eq!(packages.len(), 3);
        assert_eq!(packages[0], 1);
        assert_eq!(packages[1], 2);
        assert_eq!(packages[2], 6);
    }

    #[test]
    fn test_create_groups() {
        let available_packages: Vec<usize> = vec![1, 2, 4, 5];
        let group_1 = create_groups(&available_packages, 2);

        assert_eq!(group_1.len(), 4 * 3 * 2 / (2 * 2));
        assert!(create_groups(&available_packages, 0).is_empty());
        assert!(create_groups(&available_packages, 5).is_empty());
    }

    #[test]
    fn test_find_groups_with_weight() {
        let packages: Vec<usize> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let found_packages = find_groups_with_weight(&packages, packages.iter().sum::<usize>() / 3);

        assert_eq!(found_packages.len(), 25); // taken from demo (indeed in the demo there are 13 combinations)
        println!("{:?}", &found_packages[0..10]);
        println!("{:?}", &found_packages[10..20]);
        println!("{:?}", &found_packages[20..25]);
    }

    #[test]
    fn test_calc_qe() {
        assert_eq!(calc_qe(&vec![11, 9]), 99);
        assert_eq!(calc_qe(&vec![10, 4, 3, 2, 1]), 240);
    }

    #[test]
    fn test_subtract_vectors() {
        let packages: Vec<usize> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let result = subtract_vectors(&packages, &vec![10, 4, 3, 2, 1]);
        assert_eq!(result.len(), 5);
        assert!(result.contains(&5));
        assert!(result.contains(&7));
        assert!(result.contains(&8));
        assert!(result.contains(&9));
        assert!(result.contains(&11));
    }

    #[test]
    fn test_find_optimal_qe() {
        let packages: Vec<usize> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(find_optimal_qe(&packages), 99);
    }
}
