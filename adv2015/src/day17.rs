use combinations::Combinations;

const REQUIRED_VOLUME: usize = 150;

pub fn get_total_count_of_combinations(input: &str) -> usize {
    let available_cans = parse_to_available_cans(input);
    let found_cans = find_cans_combination_total_count(&available_cans, REQUIRED_VOLUME);

    found_cans
}

pub fn get_minimal_count_of_cans(input: &str) -> usize {
    let available_cans = parse_to_available_cans(input);
    let found_cans = find_cans_combination_minimal_count(&available_cans, REQUIRED_VOLUME);

    found_cans
}

fn parse_to_available_cans(input: &str) -> Vec<usize> {
    input.lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<usize>())
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect()
}

fn find_cans_combination_total_count(vec: &Vec<usize>, required_volume: usize) -> usize {
    if vec.is_empty() {
        return 0;
    }

    let k_tuple = find_k(vec, required_volume);

    let mut answer = 0;

    if k_tuple.0 > 0 {
        for k in k_tuple.0..=k_tuple.1 {
            let cloned_vec: Vec<(usize, usize)> = vec.iter()
                .enumerate()
                .map(|e| (e.0, *e.1))
                .collect();
            let computed = Combinations::new(cloned_vec, k);

            answer += computed.map(|c| {
                c.iter().map(|can| can.1)
                    .sum()
            })
                .filter(|&volume: &usize| volume == required_volume)
                .count();
        }
    }

    answer
}

fn find_cans_combination_minimal_count(vec: &Vec<usize>, required_volume: usize) -> usize {
    if vec.is_empty() {
        return 0;
    }

    let k_tuple = find_k(vec, required_volume);

    if k_tuple.0 > 0 {
        for k in k_tuple.0..=k_tuple.1 {
            let cloned_vec: Vec<(usize, usize)> = vec.iter().enumerate()
                .map(|e| (e.0, *e.1))
                .collect();
            let computed = Combinations::new(cloned_vec, k);

            let answer = computed.map(|c| {
                c.iter().map(|can| can.1)
                    .sum()
            })
                .filter(|&volume: &usize| volume == required_volume)
                .count();
            if answer > 0 {
                return answer;
            }
        }
    }

    0
}

fn find_k(vec: &Vec<usize>, required_volume: usize) -> (usize, usize) {
    let mut sorted_input_cans = vec.clone();
    sorted_input_cans.sort();

    let mut max_k = sorted_input_cans.len() - 1;
    let mut min_k = 0;
    let mut sum = 0;

    for i in (0..sorted_input_cans.len()).rev() {
        if sum >= required_volume {
            min_k = sorted_input_cans.len() - i - 1;
            break;
        }

        sum += sorted_input_cans[i];
    }

    let mut sum = 0;
    for i in 0..sorted_input_cans.len() {
        if sum >= required_volume {
            max_k = i - 1;
            break;
        }

        sum += sorted_input_cans[i];
    }

    (min_k, max_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_count_of_combinations() {
        assert_eq!(get_total_count_of_combinations(r#"50
        50
        100"#), 2);
    }

    #[test]
    fn test_get_minimal_count_of_cans() {
        assert_eq!(get_minimal_count_of_cans(r#"10
        140
        50
        50
        50"#), 1);
    }

    #[test]
    fn test_parse_to_available_cans() {
        let cans = parse_to_available_cans(r#"20
        15
        10
        5
        5"#);

        assert_eq!(cans.len(), 5);

        assert_eq!(cans[0], 20);
        assert_eq!(cans[4], 5);
    }

    #[test]
    fn test_find_cans_combination_count() {
        assert_eq!(find_cans_combination_total_count(&vec![1, 2, 3, 4, 5], 5), 3);
        assert_eq!(find_cans_combination_total_count(&vec![20, 15, 10, 5, 5], 25), 4);
    }

    #[test]
    fn test_find_k() {
        let cans1 = find_k(&vec![1, 2, 3, 4, 5], 5);
        assert_eq!(cans1.0, 1usize);
        assert_eq!(cans1.1, 2usize);

        let cans2 = find_k(&vec![20, 15, 10, 5, 5], 25);
        assert_eq!(cans2.0, 2usize);
        assert_eq!(cans2.1, 3usize);

        let cans3 = find_k(&vec![50, 50, 100], REQUIRED_VOLUME);
        assert_eq!(cans3.0, 2usize);
        assert_eq!(cans3.1, 2usize);
    }

    #[test]
    fn test_zeroes() {
        assert_eq!(get_total_count_of_combinations(r#""#), 0);
        assert_eq!(get_minimal_count_of_cans(r#""#), 0);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(get_total_count_of_combinations(r#"5"#), 0);
        assert_eq!(get_minimal_count_of_cans(r#"5"#), 0);

        assert_eq!(get_total_count_of_combinations(r#"5 155"#), 0);
        assert_eq!(get_minimal_count_of_cans(r#"5 155"#), 0);
    }
}
