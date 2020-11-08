enum Action {
    MagickMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

struct Human {
    health: usize,
    damage: usize,
    armor: usize,
}

impl Default for Human {
    fn default() -> Human {
        Human {
            health: 0,
            damage: 0,
            armor: 0,
        }
    }
}

fn parse_enemy(input: &str) -> Human {
    input.to_lowercase()
        .lines()
        .map(|line| line.trim())
        .fold(Human::default(), |mut res, line| {
            let mut pair = line.split(": ");

            let item = pair.next().unwrap();
            if let Ok(value) = pair.next().unwrap().parse::<usize>() {
                match item {
                    "hit points" => res.health = value,
                    "damage" => res.damage = value,
                    _ => {}
                }
            }
            res
        })
}

struct Generator {
    counter: Vec<u8>,
    queue_length: u32,
}

impl Generator {
    fn new() -> Generator {
        Generator {
            counter: vec![0, 10],
            queue_length: 10,
        }
    }
}

impl Iterator for Generator {
    type Item = Vec<Action>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct BigNumber {
    counter: Vec<u8>,
    capped_value: u8,
}

impl BigNumber {
    fn new(capped_value: u8, length: usize) -> BigNumber {
        BigNumber {
            counter: vec![1; length],
            capped_value,
        }
    }
}

impl Iterator for BigNumber {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.iter()
            .filter(|digit| **digit == self.capped_value)
            .count() == self.counter.len() {
            return None;
        }

        let mut next = false;
        let counter_length = self.counter.len();

        self.counter[counter_length - 1] += 1;
        for i in (0..counter_length).rev() {
            if next {
                self.counter[i] += 1;
                next = false;
            }

            if self.counter[i] > self.capped_value {
                next = true;
                self.counter[i] -= self.capped_value;
            }
        }

        Some(self.counter.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enemy() {
        let enemy = parse_enemy(
            r#"Hit Points: 71
            Damage: 10
            Name: Tester"#);

        assert_eq!(enemy.health, 71);
        assert_eq!(enemy.damage, 10);
        assert_eq!(enemy.armor, 0);
    }

    #[test]
    fn test_big_number_small() {
        let mut big = BigNumber::new(2, 2);
        assert_eq!(big.counter, vec![1, 1]);
        assert_eq!(big.next(), Some(vec![1, 2]));
        assert_eq!(big.next(), Some(vec![2, 1]));
        assert_eq!(big.next(), Some(vec![2, 2]));
        assert_eq!(big.next(), None);
    }

    #[test]
    fn test_big_number_big() {
        let mut big = BigNumber::new(3, 3);
        assert_eq!(big.counter, vec![1, 1, 1]);
        big.next();
        big.next();
        assert_eq!(big.next(), Some(vec![1, 2, 1]));
        assert_eq!(big.next(), Some(vec![1, 2, 2]));
        big.next();
        big.next();
        big.next();
        assert_eq!(big.next(), Some(vec![1, 3, 3]));
    }
}
