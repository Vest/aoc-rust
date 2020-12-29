use std::collections::HashMap;

pub fn find_number_2020(input: &str) -> usize {
    let mut game = Game::new(input);

    game.simulate_rounds(2020)
}

struct Game {
    numbers_rounds: HashMap<usize, usize>,
    round: usize,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut numbers: HashMap<usize, usize> = HashMap::new();
        let round = input.split(|c| c == ',')
            .map(&str::parse::<usize>)
            .filter_map(Result::ok)
            .enumerate()
            .map(|(pos, num)| {
                numbers.insert(num, pos + 1);
                pos + 1
            }).max().unwrap_or_default();

        Game {
            numbers_rounds: numbers,
            round,
        }
    }

    fn simulate_rounds(&mut self, max_rounds: usize) -> usize {
        if self.numbers_rounds.is_empty() {
            return 0;
        }

        let mut last_number: Option<usize>;
        let mut speak_number: Option<usize> = None;
        let mut next_number: Option<usize> = None;

        while self.round < max_rounds {
            self.round += 1;
            last_number = speak_number;

            if last_number.is_none() {
                speak_number = Some(0);
            } else {
                if let Some(last_spoken_round) = self.numbers_rounds.get(&last_number.unwrap()) {
                    next_number = Some(self.round - 1 - last_spoken_round);
                } else {
                    next_number = Some(0);
                }

                self.numbers_rounds.insert(last_number.unwrap(), self.round - 1);
            }

            if next_number.is_some() {
                speak_number = next_number;
            }
        }

        speak_number.unwrap_or_default()
    }
}

pub fn find_number_30000000(input: &str) -> usize {
    let mut game = Game::new(input);

    game.simulate_rounds(30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_number_2020(""), 0);
        assert_eq!(find_number_30000000(""), 0);
    }

    #[test]
    fn test_game_new() {
        let game = Game::new("0,3,6");

        assert_eq!(game.round, 3);
        assert_eq!(game.numbers_rounds.len(), 3);
    }

    #[test]
    fn test_simulate_rounds() {
        let mut game = Game::new("0,3,6");

        assert_eq!(game.simulate_rounds(2020), 436);
    }

    #[test]
    fn test_find_number_2020() {
        assert_eq!(find_number_2020("0,3,6"), 436);

        assert_eq!(find_number_2020("1,3,2"), 1);
        assert_eq!(find_number_2020("2,1,3"), 10);
        assert_eq!(find_number_2020("1,2,3"), 27);
        assert_eq!(find_number_2020("2,3,1"), 78);
        assert_eq!(find_number_2020("3,2,1"), 438);
        assert_eq!(find_number_2020("3,1,2"), 1836);
    }

    #[test]
    #[ignore]
    fn test_find_number_30000000() {
        assert_eq!(find_number_30000000("0,3,6"), 175594);
/*
        assert_eq!(find_number_30000000("1,3,2"), 2578);
        assert_eq!(find_number_30000000("2,1,3"), 3544142);
        assert_eq!(find_number_30000000("1,2,3"), 261214);
        assert_eq!(find_number_30000000("2,3,1"), 6895259);
        assert_eq!(find_number_30000000("3,2,1"), 18);
        assert_eq!(find_number_30000000("3,1,2"), 362);
 */
    }
}
