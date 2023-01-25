use std::ops::*;

pub fn get_richest_house_before_strike(input: &str) -> usize {
    if let Ok(desired_number) = input.parse::<usize>() {
        find_house(desired_number, count_presents)
    } else {
        0
    }
}

pub fn get_richest_house_after_strike(input: &str) -> usize {
    if let Ok(desired_number) = input.parse::<usize>() {
        find_house(desired_number, count_strike_presents)
    } else {
        0
    }
}

fn find_house(desired_number: usize, algorithm: fn(usize) -> usize) -> usize {
    RangeFrom { start: 1usize }
        .find(|&house_number| algorithm(house_number) >= desired_number)
        .unwrap_or(0)
}

fn count_presents(house: usize) -> usize {
    if house == 0 {
        return 0;
    }

    (divisors::get_divisors(house).iter().sum::<usize>() + 1 + if house > 2 { house } else { 0 })
        * 10
}

fn count_strike_presents(house: usize) -> usize {
    if house == 0 {
        return 0;
    }

    (divisors::get_divisors(house)
        .iter()
        .filter(|elf| house / *elf <= 50)
        .sum::<usize>()
        + match house {
            1..=2 => 1,
            3..=50 => house + 1,
            _ => house,
        })
        * 11
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_presents() {
        assert_eq!(count_presents(0), 0);
        assert_eq!(count_presents(1), 10);
        assert_eq!(count_presents(2), 30);
        assert_eq!(count_presents(3), 40);
        assert_eq!(count_presents(4), 70);
        assert_eq!(count_presents(5), 60);
        assert_eq!(count_presents(6), 120);
        assert_eq!(count_presents(7), 80);
        assert_eq!(count_presents(8), 150);
        assert_eq!(count_presents(9), 130);
    }

    #[test]
    fn test_find_house() {
        assert_eq!(find_house(119, count_presents), 6);
        assert_eq!(find_house(119, count_strike_presents), 6);
    }

    #[test]
    fn test_count_strike_presents() {
        assert_eq!(count_strike_presents(0), 0);
        assert_eq!(count_strike_presents(53), 53 * 11);
        assert_eq!(count_strike_presents(47), 47 * 11 + 11);
        assert_eq!(count_strike_presents(6), 11 + 2 * 11 + 3 * 11 + 6 * 11);
    }

    #[test]
    fn test_get_richest_house_fast() {
        assert_eq!(get_richest_house_before_strike(""), 0);
        assert_eq!(get_richest_house_after_strike(""), 0);
        assert_eq!(get_richest_house_before_strike("360000"), 10080);
        assert_eq!(get_richest_house_after_strike("360000"), 10080);
    }

    #[test]
    #[ignore]
    fn test_get_richest_house_slow() {
        assert_eq!(get_richest_house_before_strike("36000000"), 831600);
        assert_eq!(get_richest_house_after_strike("36000000"), 884520);
    }
}
