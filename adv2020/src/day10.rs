pub fn find_one_by_three(input: &str) -> usize {
    let input: Vec<usize> = parse_as_sorted(input);
    let OneTwo(ones, threes) = group_by_difference(&input);

    ones * threes
}

pub fn find_answer2(_input: &str) -> usize {
    0
}

fn parse_as_sorted(input: &str) -> Vec<usize> {
    let mut result: Vec<usize> = input.lines()
        .map(&str::trim)
        .map(&str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    // there is always a jolt with 0 and highest + 3 (according to the exercise description)
    result.push(0);
    result.sort();
    result.push(result.last().unwrap() + 3);

    result
}

struct OneTwo(usize, usize);

impl Default for OneTwo {
    fn default() -> Self {
        OneTwo(0, 0)
    }
}


fn group_by_difference(input: &[usize]) -> OneTwo {
    input.windows(2)
        .filter(check_jolt)
        .fold(OneTwo::default(), |mut acc, pair| {
            match pair[1].saturating_sub(pair[0]) {
                1 => acc.0 += 1,
                3 => acc.1 += 1,
                _ => (),
            }

            acc
        })
}

fn check_jolt(pair: &&[usize]) -> bool {
    if pair.len() != 2 {
        return false;
    }

    match pair[1].saturating_sub(pair[0]) {
        1 | 2 | 3 => true,
        _ => false
    }
}

fn is_chain_valid(chain: &[usize]) -> bool {
    chain.windows(2)
        .find(|pair| !check_jolt(pair))
        .is_none()
}

fn count_possible_valid_chains(chain: &[usize]) -> usize {
    let mut count = 0;

    for i in 1..chain.len() {
        //let temp = [&[..i], &[i + 1..]].concat();
        if is_chain_valid(&chain[i..]) {
            count += 1;
        }
        count += count_possible_valid_chains(&chain[i..]);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORT_INPUT: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    const LONG_INPUT: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_one_by_three(""), 0);
        assert_eq!(find_answer2(""), 0);
    }

    #[test]
    fn test_parse_as_sorted() {
        let result = parse_as_sorted("3 \n 1\n2\n\n");
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
        assert_eq!(result[3], 3);
        assert_eq!(result[4], 6);
    }

    #[test]
    fn test_group_by_difference() {
        let result = group_by_difference(&vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 19 + 3]);
        assert_eq!(result.0, 7);
        assert_eq!(result.1, 5);
    }

    #[test]
    fn test_find_one_by_three() {
        assert_eq!(find_one_by_three(SHORT_INPUT), 7 * 5);
        assert_eq!(find_one_by_three(LONG_INPUT), 22 * 10);
    }

    #[test]
    fn test_is_chain_valid() {
        assert!(is_chain_valid(&vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22]));
        assert!(is_chain_valid(&vec![0, 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, 22]));
        assert!(is_chain_valid(&vec![0, 1, 4, 6, 7, 10, 12, 15, 16, 19, 22]));

        assert!(!is_chain_valid(&vec![0, 1, 5, 8]));
        assert!(is_chain_valid(&vec![]));
    }

    #[test]
    fn test_count_possible_valid_chains() {
    //    count_possible_valid_chains()
    }
}
