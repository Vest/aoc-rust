pub fn find_earliest_bus(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut lines = input.lines();
    let time = lines.next().unwrap().parse::<usize>().unwrap();
    let bus = lines
        .next()
        .unwrap()
        .split(|c| c == ',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .map(|bus| (bus, find_next_bus(time, bus)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    (bus.1 - time) * bus.0
}

pub fn find_earliest_timestamp(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let buses: Vec<(usize, usize)> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(|c| c == ',')
        .enumerate()
        .filter_map(|(pos, str)| {
            if let Ok(num) = str.parse::<usize>() {
                Some((pos, num))
            } else {
                None
            }
        })
        .map(|(pos, num)| {
            if pos != 0 {
                let mut checked_sub = None;
                let mut i = 1usize;
                while checked_sub.is_none() {
                    checked_sub = (num * i).checked_sub(pos);
                    i += 1;
                }

                (checked_sub.unwrap(), num)
            } else {
                (pos, num)
            }
        })
        .collect();

    chinese_remainder_theorem(&buses).0
}

fn chinese_remainder_theorem(nums: &[(usize, usize)]) -> (usize, usize) {
    let m_product: usize = nums.iter().map(|(_, modulo)| modulo).product();

    let mi = nums
        .iter()
        .map(|(_, modulo)| m_product / modulo)
        .collect::<Vec<usize>>();

    let yi = nums
        .iter()
        .map(|(_, modulo)| *modulo)
        .zip(mi.iter())
        .map(|(modulo, &mi)| (1..mi).find(|&rem| (rem * mi) % modulo == 1).unwrap())
        .collect::<Vec<usize>>();

    let x = (0..nums.len())
        .map(|i| nums[i].0 * mi[i] * yi[i])
        .sum::<usize>()
        % m_product;

    (x, m_product)
}

fn find_next_bus(time: usize, bus: usize) -> usize {
    let div = time % bus;

    if div == 0 {
        time
    } else {
        time + (bus - div)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_earliest_bus(""), 0);
        assert_eq!(find_earliest_timestamp(""), 0);
    }

    #[test]
    fn test_find_next_bus() {
        assert_eq!(find_next_bus(939, 7), 945);
        assert_eq!(find_next_bus(939, 13), 949);
        assert_eq!(find_next_bus(939, 59), 944);
        assert_eq!(find_next_bus(939, 31), 961);
        assert_eq!(find_next_bus(939, 19), 950);
    }

    #[test]
    fn test_find_earliest_bus() {
        assert_eq!(find_earliest_bus("939\n7,13,x,x,59,x,31,19"), 295);
    }

    #[test]
    fn test_find_earliest_timestamp() {
        assert_eq!(find_earliest_timestamp("939\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(find_earliest_timestamp("939\n17,x,13,19"), 3417);
        assert_eq!(find_earliest_timestamp("939\n67,7,59,61"), 754018);
        assert_eq!(find_earliest_timestamp("939\n67,x,7,59,61"), 779210);
        assert_eq!(find_earliest_timestamp("939\n67,7,x,59,61"), 1261476);
        assert_eq!(find_earliest_timestamp("939\n1789,37,47,1889"), 1202161486);
    }

    #[test]
    fn test_chinese_remainder_theorem() {
        assert_eq!(
            chinese_remainder_theorem(&[(2, 3), (3, 5), (1, 7)]),
            (8, 105)
        );
        assert_eq!(
            chinese_remainder_theorem(&[(0, 7), (12, 13), (55, 59), (25, 31), (12, 19)]),
            (1068781, 7 * 13 * 59 * 31 * 19)
        );
    }
}
