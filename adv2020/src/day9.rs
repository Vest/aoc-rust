use itertools::Itertools;

pub fn find_weak_number_25(input: &str) -> usize {
    let input: Vec<usize> = parse_input(input);

    find_weak_number(&input, 25)
        .unwrap_or((0, 0)).0
}

pub fn find_sum_of_any_numbers(input: &str) -> usize {
    let input: Vec<usize> = parse_input(input);

    if let Some((weak_number, weak_number_pos)) = find_weak_number(&input, 25) {
        (3..weak_number_pos)
            .find_map(|window_size|
                find_possible_sum(&input[0..weak_number_pos], window_size, weak_number))
            .unwrap_or_default()
    } else {
        0
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
        .map(&str::trim)
        .map(&str::parse::<usize>)
        .filter_map(Result::ok)
        .collect()
}

fn find_weak_number(input: &Vec<usize>, preamble: usize) -> Option<(usize, usize)> {
    for num in preamble..input.len() {
        let last_numbers = &input[num.saturating_sub(preamble)..num];

        if last_numbers.into_iter()
            .combinations(2)
            .find(|pair| pair.iter()
                .map(|n| *n)
                .sum::<usize>() == input[num])
            .is_none() {
            return Some((input[num], num));
        }
    }

    None
}

fn find_possible_sum(numbers: &[usize], window_size: usize, weak_number: usize) -> Option<usize> {
    numbers.windows(window_size)
        .filter_map(|window| {
            let sum = window.iter().sum::<usize>();
            let min = window.iter().min();
            let max = window.iter().max();

            if sum == weak_number && min.is_some() && max.is_some() {
                Some(min.unwrap() + max.unwrap())
            } else {
                None
            }
        })
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty_answers() {
        assert_eq!(find_weak_number_25(""), 0);
        assert_eq!(find_sum_of_any_numbers(""), 0);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input("  1   \n2   ");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
    }

    #[test]
    fn test_find_weak_number() {
        assert_eq!(find_weak_number(&vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
                                          102, 117, 150, 182, 127, 219, 299, 277, 309, 57], 5), Some((127, 14)));

        assert_eq!(find_weak_number(&vec![35, 20, 15, 25, 47, 40, 62], 5), None);
    }

    #[test]
    fn test_find_possible_sum() {
        assert_eq!(find_possible_sum(&vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
                                           102, 117, 150, 182], 4, 127), Some(62));

        assert_eq!(find_possible_sum(&vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
                                           102, 117, 150, 182], 3, 127), None);
    }
}
