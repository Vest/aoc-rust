use itertools::Itertools;
use rand::Rng;
use std::ptr::null_mut;

pub fn find_answer1(input: &str) -> usize {
    let mut input: Vec<usize> = input.lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    for num in 25..input.len() {
        let mut stop = true;

        'outer: for i in num.saturating_sub(25)..num.saturating_sub(1) {
            for j in num.saturating_sub(24)..num {
                if input[i] + input[j] == input[num] {
                    stop = false;
                    break 'outer;
                }
            }
        }

        if stop {
            return input[num];
        }
    }


    0
}

pub fn find_answer2(input: &str) -> usize {
    let mut input: Vec<usize> = input.lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    (2..input.len()).find_map(|window_length| {
        input.windows(window_length)
            .filter_map(|window|
                if window.iter().sum::<usize>() == 675280050 {
                    Some(window.iter().min().unwrap() + window.iter().max().unwrap())
                } else {
                    None
                }).next()
    }).unwrap()
}
/*
fn parse_input<'a>(input: &'a str) -> impl Iterator<Item=Seat> + 'a {
    input.lines()
        .map(&str::trim)
        .map(parse_seat)
}
*/

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(""), 0);
        assert_eq!(find_answer2(""), 0);
    }
}
