enum Action {
    // 1
    MagickMissile {
        cost: usize,
        damage: usize,
    },

    // 2
    Drain {
        cost: usize,
        damage: usize,
        heal: usize,
    },

    // 3
    Shield {
        cost: usize,
        duration: usize,
        armor: usize,
    },

    // 4
    Poison {
        cost: usize,
        duration: usize,
        damage: usize,
    },

    // 5
    Recharge {
        cost: usize,
        duration: usize,
        mana: usize,
    },
}

struct Human {
    health: usize,
    damage: usize,
    armor: usize,
}

impl Action {
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(Action::MagickMissile { cost: 53, damage: 4 }),
            2 => Some(Action::Drain { cost: 73, damage: 2, heal: 2 }),
            3 => Some(Action::Shield { cost: 113, duration: 6, armor: 7 }),
            4 => Some(Action::Poison { cost: 173, duration: 6, damage: 3 }),
            5 => Some(Action::Recharge { cost: 229, duration: 5, mana: 101 }),
            _ => None,
        }
    }
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
    counter: BigNumber,
}

impl Generator {
    fn new(queue_length: usize) -> Generator {
        Generator {
            counter: BigNumber::new(5, queue_length),
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

fn is_queue_valid(queue: Vec<Action>) -> bool {
    let mut shield_state = (0usize, false);
    let mut poison_state = (0usize, false);
    let mut recharge_state = (0usize, false);

    for action in queue {
        shield_state.0 = shield_state.0.saturating_sub(1);
        poison_state.0 = poison_state.0.saturating_sub(1);
        recharge_state.0 = recharge_state.0.saturating_sub(1);

        match action {
            Action::Shield { duration, .. } => {
                if shield_state.1 {
                    return false;
                }
                shield_state.0 = duration;
                shield_state.1 = true;
            }
            Action::Poison { duration, .. } => {
                if poison_state.1 {
                    return false;
                }
                poison_state.0 = duration;
                poison_state.1 = true;
            }
            Action::Recharge { duration, .. } => {
                if recharge_state.1 {
                    return false;
                }
                recharge_state.0 = duration;
                recharge_state.1 = true;
            }
            _ => {}
        }
    }

    true
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

    #[test]
    fn test_from_u8() {
        assert!(Action::from_u8(0).is_none());
        assert!(Action::from_u8(1).is_some());
        assert!(Action::from_u8(2).is_some());
        assert!(Action::from_u8(3).is_some());
        assert!(Action::from_u8(4).is_some());
        assert!(Action::from_u8(5).is_some());
        assert!(Action::from_u8(6).is_none());
    }

    #[test]
    fn test_is_queue_valid() {
        assert!(is_queue_valid(vec![Action::from_u8(1).unwrap(), Action::from_u8(2).unwrap()]));
        assert!(is_queue_valid(vec![Action::from_u8(2).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(vec![Action::from_u8(3).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(is_queue_valid(vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(is_queue_valid(vec![Action::from_u8(4).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(vec![Action::from_u8(4).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(!is_queue_valid(vec![Action::from_u8(5).unwrap(), Action::from_u8(5).unwrap()]));
        assert!(is_queue_valid(vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap(), Action::from_u8(5).unwrap()]));
    }
}
