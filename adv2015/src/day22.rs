pub fn find_answer(input: &str) -> usize {
    let enemy = parse_enemy(input);
    let player = Player {
        health: 50,
        mana: 500,
        armor: 0,
    };
    let turns_count = 13;
    let mut cost = usize::MAX;
    let gen = Generator::new(turns_count);
/*
    println!("Start generation");
    let mut sorted_gen: Vec<Vec<Action>> = gen.collect();
    println!("End generation");
    println!("Start sorting");
    sorted_gen.sort_by(|a, b| {
        let cost_a = count_queue_cost(a);
        let cost_b = count_queue_cost(b);

        cost_a.cmp(&cost_b)
    });
    println!("End sorting");
*/
    println!("Start the battle!!!!");
    for queue in gen {
        //    if is_queue_valid(&queue) {
        if let (Battle::Won, new_cost) = simulate_battle(&player, &enemy, &queue) {
            println!("Won with {} mana", new_cost);
            if new_cost < cost {
                cost = new_cost;
            }
        } else {
            // println!("Lost with {} mana", new_cost);
        }
        //       }
    }

    cost
}

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

impl Action {
    fn cost(&self) -> usize {
        match self {
            Action::MagickMissile { cost, .. } => *cost,
            Action::Drain { cost, .. } => *cost,
            Action::Shield { cost, .. } => *cost,
            Action::Poison { cost, .. } => *cost,
            Action::Recharge { cost, .. } => *cost,
        }
    }
}

#[derive(Clone)]
struct Player {
    health: usize,
    mana: usize,
    armor: usize,
}

impl Player {
    fn dead(&self) -> bool {
        self.health == 0
    }

    fn can_cast(&self, mana: usize) -> bool {
        self.mana > mana
    }
}

#[derive(Clone)]
struct Enemy {
    health: usize,
    damage: usize,
}

impl Enemy {
    fn dead(&self) -> bool {
        self.health == 0
    }
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

fn simulate_battle(player: &Player, enemy: &Enemy, actions: &Vec<Action>) -> (Battle, usize) {
    let mut cost = 0usize;
    let mut player_clone = (*player).clone();
    let mut enemy_clone = (*enemy).clone();
    let mut shield_status = (0usize, 0usize);
    let mut poison_status = (0usize, 0usize);
    let mut recharge_status = (0usize, 0usize);

    for action in actions {
        if !player_clone.can_cast(action.cost()) {
            return (Battle::Lost, cost);
        }
        // Cast the spell
        player_clone.mana = player_clone.mana.saturating_sub(action.cost());
        cost += action.cost();

        player_clone.armor = shield_status.1;
        enemy_clone.health = enemy_clone.health.saturating_sub(poison_status.1);
        player_clone.mana = player_clone.mana.saturating_add(recharge_status.1);

        shield_status.0 = shield_status.0.saturating_sub(1);
        poison_status.0 = poison_status.0.saturating_sub(1);
        recharge_status.0 = recharge_status.0.saturating_sub(1);

        if shield_status.0 == 0 {
            shield_status.1 = 0;
        }
        if poison_status.0 == 0 {
            poison_status.1 = 0;
        }
        if recharge_status.0 == 0 {
            recharge_status.1 = 0;
        }

        // Player Turn
        match action {
            Action::MagickMissile { damage, .. } => {
                enemy_clone.health = enemy_clone.health.saturating_sub(*damage);
            }
            Action::Drain { damage, heal, .. } => {
                enemy_clone.health = enemy_clone.health.saturating_sub(*damage);
                player_clone.health = player_clone.health.saturating_add(*heal);
            }
            Action::Shield { duration, armor, .. } => {
                shield_status = (*duration, *armor);
            }
            Action::Poison { duration, damage, .. } => {
                poison_status = (*duration, *damage);
            }
            Action::Recharge { duration, mana, .. } => {
                recharge_status = (*duration, *mana);
            }
        }
        // Enemy Turn
        player_clone.armor = shield_status.1;
        enemy_clone.health = enemy_clone.health.saturating_sub(poison_status.1);
        player_clone.mana = player_clone.mana.saturating_add(recharge_status.1);

        if enemy_clone.dead() {
            return (Battle::Won, cost);
        }

        let damage = enemy_clone.damage.saturating_sub(player_clone.armor);
        player_clone.health = player_clone.health.saturating_sub(if damage == 0 { 1 } else { damage });

        if player_clone.dead() {
            return (Battle::Lost, cost);
        }

        shield_status.0 = shield_status.0.saturating_sub(1);
        poison_status.0 = poison_status.0.saturating_sub(1);
        recharge_status.0 = recharge_status.0.saturating_sub(1);

        if shield_status.0 == 0 {
            shield_status.1 = 0;
        }
        if poison_status.0 == 0 {
            poison_status.1 = 0;
        }
        if recharge_status.0 == 0 {
            recharge_status.1 = 0;
        }
    }

    (Battle::Lost, cost)
}

fn count_queue_cost(queue: &Vec<Action>) -> usize {
    queue.iter()
        .map(|a| a.cost())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Battle {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Battle::Won, Battle::Won) | (Battle::Lost, Battle::Lost) => true,
                _ => false,
            }
        }
    }

    #[test]
    fn test_parse_enemy() {
        let enemy = parse_enemy(
            r#"Hit Points: 71
            Damage: 10
            Name: Tester"#);

        assert_eq!(enemy.health, 71);
        assert_eq!(enemy.damage, 10);
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
    fn test_simulate_battle_1() {
        let player = Player {
            health: 10,
            mana: 250,
            armor: 0,
        };

        let enemy = Enemy {
            health: 13,
            damage: 8,
        };

        let actions = vec![Action::from_u8(4).unwrap(), Action::from_u8(1).unwrap()];
        let cost = count_queue_cost(&actions);

        assert_eq!(simulate_battle(&player, &enemy, &actions), (Battle::Won, cost));
    }

    #[test]
    fn test_simulate_battle_2() {
        let player = Player {
            health: 10,
            mana: 250,
            armor: 0,
        };

        let enemy = Enemy {
            health: 14,
            damage: 8,
        };

        let actions = vec![
            Action::from_u8(5).unwrap(),
            Action::from_u8(3).unwrap(),
            Action::from_u8(2).unwrap(),
            Action::from_u8(4).unwrap(),
            Action::from_u8(1).unwrap()
        ];
        let cost = count_queue_cost(&actions);

        assert_eq!(simulate_battle(&player, &enemy, &actions), (Battle::Won, cost));
    }
}
