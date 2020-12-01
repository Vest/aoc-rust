use combinations::Combinations;

const DESIRED_SUM: usize = 2020;

pub fn find_expenses(input: &str) -> usize {
    let input = parse_input(input);
    let result = find_pair_with_sum(input, DESIRED_SUM);

    result.0 * result.1
}

pub fn find_more_expenses(input: &str) -> usize {
    let input = parse_input(input);
    let result = find_triple_with_sum(input, DESIRED_SUM);

    result.0 * result.1 * result.2
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
        .map(|l| l.trim())
        .filter_map(|n| n.parse::<usize>().ok())
        .collect()
}

fn find_pair_with_sum(nums: Vec<usize>, desired_sum: usize) -> (usize, usize) {
    let mut computed = Combinations::new(nums, 2);
    if let Some(result) = computed.find(|pair| pair[0] + pair[1] == desired_sum) {
        return (result[0], result[1]);
    }

    return (0, 0);
}

fn find_triple_with_sum(nums: Vec<usize>, desired_sum: usize) -> (usize, usize, usize) {
    let mut computed = Combinations::new(nums, 3);
    if let Some(result) = computed.find(|pair| pair[0] + pair[1] + pair[2] == desired_sum) {
        return (result[0], result[1], result[2]);
    }

    return (0, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"1721
                   979
                   366
                   299
                   675
                   1456"#;
        let result = parse_input(input);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 1721);
        assert_eq!(result[4], 675);
    }

    #[test]
    fn test_find_pair_with_sum() {
        let result = find_pair_with_sum(vec![1721, 979, 366, 299, 675, 1456], DESIRED_SUM);
        assert_eq!(result.1, 1721);
        assert_eq!(result.0, 299);
    }

    #[test]
    fn test_find_expenses() {
        assert_eq!(find_expenses(r#"1721
                   979
                   366
                   299
                   675
                   1456"#), 514579);
    }

    #[test]
    fn test_find_more_expenses() {
        assert_eq!(find_more_expenses(r#"1721
                   979
                   366
                   299
                   675
                   1456"#), 241861950);
    }
}


