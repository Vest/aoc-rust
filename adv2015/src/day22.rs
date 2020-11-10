#[derive(Debug)]
enum Battle {
    Lost,
    Won,
}

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

struct Player {
    health: usize,
    mana: usize,
}

struct Enemy {
    health: usize,
    damage: usize,
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

impl Default for Enemy {
    fn default() -> Enemy {
        Enemy {
            health: 0,
            damage: 0,
        }
    }
}

fn parse_enemy(input: &str) -> Enemy {
    input.to_lowercase()
        .lines()
        .map(|line| line.trim())
        .fold(Enemy::default(), |mut res, line| {
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
    finish: bool,
}

impl Generator {
    fn new(queue_length: usize) -> Generator {
        Generator {
            counter: BigNumber::new(5, queue_length),
            finish: false,
        }
    }
}

impl Iterator for Generator {
    type Item = Vec<Action>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            return None;
        }

        let mut queue = create_queue_from_vec(&self.counter.counter);
        while !is_queue_valid(&queue) {
            let next = self.counter.next();
            if next.is_none() {
                self.finish = true;
                return None;
            }

            queue = create_queue_from_vec(&next.unwrap());
        }
        self.counter.next();
        Some(queue)
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

fn create_queue_from_vec(input: &Vec<u8>) -> Vec<Action> {
    input.iter()
        .map(|c| Action::from_u8(*c))
        .filter_map(|a| a)
        .collect()
}

fn is_queue_valid(queue: &Vec<Action>) -> bool {
    let mut shield_duration = 0usize;
    let mut poison_duration = 0usize;
    let mut recharge_duration = 0usize;

    for action in queue {
        shield_duration = shield_duration.saturating_sub(1);
        poison_duration = poison_duration.saturating_sub(1);
        recharge_duration = recharge_duration.saturating_sub(1);

        match action {
            Action::Shield { duration, .. } => {
                if shield_duration > 0 {
                    return false;
                }
                shield_duration = *duration;
            }
            Action::Poison { duration, .. } => {
                if poison_duration > 0 {
                    return false;
                }
                poison_duration = *duration;
            }
            Action::Recharge { duration, .. } => {
                if recharge_duration > 0 {
                    return false;
                }
                recharge_duration = *duration;
            }
            _ => {}
        }
    }

    true
}

fn simulate_battle(player: &Player, enemy: &Enemy, actions: &Vec<Action>) -> Battle {
    Battle::Lost
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
    fn test_create_queue_from_vec() {
        let mut big = BigNumber::new(3, 3);
        let next_number = big.next();
        let actions = create_queue_from_vec(&next_number.unwrap());
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn test_is_queue_valid() {
        assert!(is_queue_valid(&vec![Action::from_u8(1).unwrap(), Action::from_u8(2).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(2).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(4).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(4).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(5).unwrap(), Action::from_u8(5).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap(), Action::from_u8(5).unwrap()]));
    }

    #[test]
    fn test_generator() {
        let mut generator = Generator::new(2);
        assert_eq!(generator.count(), 5 + 5 + 4 + 4 + 4);

        generator = Generator::new(3);
        assert_eq!(generator.count(), 86);
    }

    #[test]
    fn test_simulate_battle() {
        let player = Player {
            health: 10,
            mana: 250,
        };

        let enemy = Enemy {
            health: 13,
            damage: 8,
        };

        let actions = vec![Action::from_u8(4).unwrap(), Action::from_u8(1).unwrap()];

        assert_eq!(simulate_battle(&player, &enemy, &actions), Battle::Won);
    }
}
