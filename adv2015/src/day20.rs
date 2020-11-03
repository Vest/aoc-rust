use std::ops::*;

pub fn get_richest_house(input: &str) -> usize {
    if let Ok(desired_number) = input.parse::<usize>() {
        find_house(desired_number)
    } else { 0 }
}

fn find_house(desired_number: usize) -> usize {
    RangeFrom {
        start: 1usize
    }.find(|&house_number| count_presents(house_number) >= desired_number)
        .unwrap_or(0)
}

fn count_presents(house: usize) -> usize {
    if house == 0 {
        return 0;
    }

    (divisors::get_divisors(house)
        .iter()
        .sum::<usize>()
        + 1
        + if house > 2 { house } else { 0 }
    ) * 10
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
        assert_eq!(find_house(119), 6);
    }
}
