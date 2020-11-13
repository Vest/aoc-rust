const PLAYER: Player = Player {
    health: 50,
    mana: 500,
    armor: 0,
};

pub fn find_easy_result(input: &str) -> usize {
    let enemy = parse_enemy(input);

    let mut game = GameState {
        queue: Vec::new(),
        player: PLAYER,
        enemy,
        won_cost: usize::MAX,
        queue_cost: 0,
    };

    simulate_game(&mut game, false);

    game.won_cost
}

pub fn find_hard_result(input: &str) -> usize {
    let enemy = parse_enemy(input);

    let mut game = GameState {
        queue: Vec::new(),
        player: PLAYER,
        enemy,
        won_cost: usize::MAX,
        queue_cost: 0,
    };

    simulate_game(&mut game, true);

    game.won_cost
}

struct GameState {
    queue: Vec<Action>,
    player: Player,
    enemy: Enemy,
    won_cost: usize,
    queue_cost: usize,
}

impl GameState {
    fn find_possible_actions(&self) -> Vec<Action> {
        let mut possible_actions: Vec<Action> = vec![Action::from_u8(1).unwrap(),
                                                     Action::from_u8(2).unwrap()];
        let mut queue_clone = self.queue.to_vec();
        for action_idx in 3..=5 {
            if let Some(action) = Action::from_u8(action_idx) {
                queue_clone.push(action);
                if is_queue_valid(&queue_clone) {
                    possible_actions.push(Action::from_u8(action_idx).unwrap());
                }
                queue_clone.pop();
            }
        }

        possible_actions
    }
}

fn simulate_game(game: &mut GameState, hard: bool) {
    let result = simulate_battle(&game.player, &game.enemy, &game.queue, hard);

    match result {
        (Battle::Won, cost) => {
            if cost < game.won_cost {
                game.won_cost = cost;
            }
        }
        (Battle::Draw, _) => {
            let actions = game.find_possible_actions();

            for action in actions {
                let action_cost = action.cost();
                if game.queue_cost + action_cost < game.won_cost {
                    game.queue.push(action);
                    game.queue_cost += action_cost;

                    simulate_game(game, hard);

                    game.queue.pop();
                    game.queue_cost -= action_cost;
                }
            }
        }
        _ => {}
    }
}

#[derive(Debug)]
enum Battle {
    Draw,
    Lost,
    Won,
}

#[derive(Clone)]
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

fn is_queue_valid(queue: &Vec<Action>) -> bool {
    let mut shield_duration = 0usize;
    let mut poison_duration = 0usize;
    let mut recharge_duration = 0usize;

    for action in queue {
        // Two actions per Turn (Player + Enemy)
        shield_duration = shield_duration.saturating_sub(2);
        poison_duration = poison_duration.saturating_sub(2);
        recharge_duration = recharge_duration.saturating_sub(2);

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

fn simulate_battle(player: &Player, enemy: &Enemy, actions: &Vec<Action>, hard: bool) -> (Battle, usize) {
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

        // Hard mode enables
        if hard {
            player_clone.health = player_clone.health.saturating_sub(1);
            if player_clone.dead() {
                return (Battle::Lost, cost);
            }
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

    (Battle::Draw, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Battle {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Battle::Won, Battle::Won) | (Battle::Lost, Battle::Lost) | (Battle::Draw, Battle::Draw) => true,
                _ => false,
            }
        }
    }

    fn count_queue_cost(queue: &Vec<Action>) -> usize {
        queue.iter()
            .map(|a| a.cost())
            .sum()
    }

    #[test]
    fn test_battle_eq() {
        assert_eq!(Battle::Lost, Battle::Lost);
        assert_ne!(Battle::Won, Battle::Lost);
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
        assert!(is_queue_valid(&vec![Action::from_u8(1).unwrap(), Action::from_u8(2).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(2).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(4).unwrap(), Action::from_u8(3).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(4).unwrap(), Action::from_u8(4).unwrap()]));
        assert!(!is_queue_valid(&vec![Action::from_u8(5).unwrap(), Action::from_u8(5).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(3).unwrap(), Action::from_u8(4).unwrap(), Action::from_u8(5).unwrap()]));
        assert!(is_queue_valid(&vec![Action::from_u8(5).unwrap(), Action::from_u8(1).unwrap(), Action::from_u8(1).unwrap(), Action::from_u8(5).unwrap()]));
    }

    #[test]
    fn test_simulate_easy_battle_1() {
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

        assert_eq!(simulate_battle(&player, &enemy, &actions, false), (Battle::Won, cost));
    }

    #[test]
    fn test_simulate_easy_battle_2() {
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

        assert_eq!(simulate_battle(&player, &enemy, &actions, false), (Battle::Won, cost));
    }

    #[test]
    fn test_simulate_easy_battle_3() {
        let player = Player {
            health: 10,
            mana: 10,
            armor: 0,
        };

        let enemy = Enemy {
            health: 11,
            damage: 8,
        };

        let actions = vec![Action::from_u8(1).unwrap()];

        assert_eq!(simulate_battle(&player, &enemy, &actions, false).0, Battle::Lost);
    }

    #[test]
    fn test_simulate_easy_battle_4() {
        let player = Player {
            health: 50,
            mana: 500,
            armor: 0,
        };

        let enemy = Enemy {
            health: 70,
            damage: 15,
        };

        let actions = vec![Action::from_u8(1).unwrap(), Action::from_u8(1).unwrap(), Action::from_u8(1).unwrap(), Action::from_u8(1).unwrap(), Action::from_u8(1).unwrap()];

        assert_eq!(simulate_battle(&player, &enemy, &actions, false).0, Battle::Lost);
    }

    #[test]
    fn test_simulate_hard_battle_1() {
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
        assert_eq!(simulate_battle(&player, &enemy, &actions, true), (Battle::Lost, 173));
    }

    #[test]
    fn test_simulate_draw() {
        let player = Player {
            health: 10,
            mana: 250,
            armor: 0,
        };

        let enemy = Enemy {
            health: 13,
            damage: 8,
        };

        let actions = vec![Action::from_u8(1).unwrap()];
        assert_eq!(simulate_battle(&player, &enemy, &actions, true), (Battle::Draw, 53));
    }

    #[test]
    fn test_find_possible_actions() {
        let game = GameState {
            queue: vec![Action::from_u8(5).unwrap()],
            player: Player {
                health: 20,
                mana: 100,
                armor: 0,
            },
            enemy: Default::default(),
            won_cost: 0,
            queue_cost: 0,
        };

        let actions = game.find_possible_actions();
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_structs() {
        let player = Player {
            health: 10,
            mana: 20,
            armor: 30,
        };
        assert!(!player.dead(), "The player should be alive");
        assert!(!player.can_cast(500), "The player cannot cast this");

        let enemy = Enemy::default();
        assert_eq!(enemy.health, 0);
        assert_eq!(enemy.damage, 0);
        assert!(enemy.dead());

        let player_clone = player.clone();
        let enemy_clone = enemy.clone();
        assert_eq!(player_clone.health, 10);
        assert_eq!(player_clone.mana, 20);
        assert_eq!(enemy_clone.health, 0);
        assert_eq!(enemy_clone.damage, 0);
        assert!(!player_clone.dead());
        assert!(enemy_clone.dead());
    }
}
