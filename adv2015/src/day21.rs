pub fn find_cheapest_warrior(input: &str) -> usize {
    let mut humanity = Generator::new();
    let mut wealth = usize::MAX;
    let enemy = parse_enemy(input);

    while let Some(human) = humanity.next() {
        if wealth > human.wealth {
            let battle = fight_to_death(&human, &enemy);

            if let Battle::Won = battle {
                wealth = human.wealth;
            }
        }
    }

    wealth
}

#[derive(Debug)]
enum Battle {
    Lost,
    Won,
}

#[derive(Clone)]
struct Human {
    health: usize,
    damage: usize,
    armor: usize,

    wealth: usize,
}

struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

struct Generator {
    weapon_idx: u32,
    armor_idx: u32,
    ring_left_idx: u32,
    ring_right_idx: u32,

    counter: u32,
}

impl Generator {
    fn new() -> Generator {
        Generator {
            weapon_idx: 0,
            armor_idx: 0,
            ring_left_idx: 0,
            ring_right_idx: 0,

            counter: 0x0000,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item { cost: 8, damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];

const ARMORS: [Item; 6] = [
    Item { cost: 0, damage: 0, armor: 0 },
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Item; 7] = [
    Item { cost: 0, damage: 0, armor: 0 }, //no ring
    Item { cost: 20, damage: 0, armor: 1 }, // defense +1
    Item { cost: 25, damage: 1, armor: 0 }, // offense +1
    Item { cost: 40, damage: 0, armor: 2 }, // defense +2
    Item { cost: 50, damage: 2, armor: 0 }, // offense +2
    Item { cost: 80, damage: 0, armor: 3 }, // defense +3
    Item { cost: 100, damage: 3, armor: 0 }, // offense +3
];

impl Default for Human {
    fn default() -> Human {
        Human {
            health: 0,
            damage: 0,
            armor: 0,

            wealth: 0,
        }
    }
}

impl Human {
    fn dead(&self) -> bool {
        self.health == 0
    }
}

impl Iterator for Generator {
    type Item = Human;

    fn next(&mut self) -> Option<Self::Item> {
        let counter_str = format!("{:01$X}", self.counter, 4);
        let mut chars = counter_str.chars();
        self.weapon_idx = chars.next().unwrap().to_digit(16).unwrap() % (WEAPONS.len() as u32);
        self.armor_idx = chars.next().unwrap().to_digit(16).unwrap() % (ARMORS.len() as u32);
        self.ring_left_idx = chars.next().unwrap().to_digit(16).unwrap() % (RINGS.len() as u32);
        self.ring_right_idx = chars.next().unwrap().to_digit(16).unwrap() % (RINGS.len() as u32);

        if self.ring_left_idx == self.ring_right_idx {
            self.ring_right_idx += 1;

            if self.ring_right_idx >= RINGS.len() as u32 {
                self.ring_right_idx = 0;
            }
        }

        // Check to see if we've finished counting or not.
        if self.counter > 0x4445 { // max digits, e.g. 5 weapons, 5 armors, 6 rings
            return None;
        }

        self.counter += 1;

        Some(Human {
            health: 100,
            damage: WEAPONS[self.weapon_idx as usize].damage
                + ARMORS[self.armor_idx as usize].damage
                + RINGS[self.ring_left_idx as usize].damage
                + RINGS[self.ring_right_idx as usize].damage,
            armor: WEAPONS[self.weapon_idx as usize].armor
                + ARMORS[self.armor_idx as usize].armor
                + RINGS[self.ring_left_idx as usize].armor
                + RINGS[self.ring_right_idx as usize].armor,
            wealth: WEAPONS[self.weapon_idx as usize].cost
                + ARMORS[self.armor_idx as usize].cost
                + RINGS[self.ring_left_idx as usize].cost
                + RINGS[self.ring_right_idx as usize].cost,
        })
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
                    "armor" => res.armor = value,
                    _ => {}
                }
            }
            res
        })
}

fn fight_to_death(human_sample: &Human, enemy_sample: &Human) -> Battle {
    let mut human = human_sample.clone();
    let mut enemy = enemy_sample.clone();

    while !enemy.dead() && !human.dead() {
        attack(&human, &mut enemy);
        if enemy.dead() {
            break;
        }
        attack(&enemy, &mut human);
    }

    if human.dead() {
        Battle::Lost
    } else {
        Battle::Won
    }
}

fn attack(attacker: &Human, defender: &mut Human) {
    let damage = attacker.damage.saturating_sub(defender.armor);
    defender.health = defender.health.saturating_sub(if damage == 0 { 1 } else { damage });
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Battle {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Battle::Lost, Battle::Lost) => true,
                (Battle::Won, Battle::Won) => true,
                _ => false,
            }
        }
    }

    #[test]
    fn test_parse_enemy() {
        let enemy = parse_enemy(
            r#"Hit Points: 100
            Damage: 8
            Armor: 2
            Name: Tester"#);

        assert_eq!(enemy.health, 100);
        assert_eq!(enemy.damage, 8);
        assert_eq!(enemy.armor, 2);
        assert_eq!(enemy.wealth, 0);
    }

    #[test]
    fn test_generator() {
        let cnt = Generator::new();
        assert_eq!(cnt.map(|human| human.health).sum::<usize>(), 1747800usize); // magick number
    }

    #[test]
    fn test_fight_to_death() {
        let human = Human { health: 8, damage: 5, armor: 5, wealth: 0 };
        let enemy = Human { health: 12, damage: 7, armor: 2, wealth: 0 };

        let result = fight_to_death(&human, &enemy);
        assert_eq!(result, Battle::Won, "The battle should be won");
        assert_ne!(result, Battle::Lost, "The battle should be won"); // coverage PartialEq
    }

    #[test]
    fn test_fight_with_uber_boss() {
        let human = Human { health: 8, damage: 5, armor: 5, wealth: 0 };
        let enemy = Human { health: 12, damage: 20, armor: 2, wealth: 0 };

        let result = fight_to_death(&human, &enemy);
        assert_eq!(result, Battle::Lost, "The battle should be lost");
    }

    #[test]
    fn test_attack() {
        let human = Human { health: 8, damage: 5, armor: 5, wealth: 0 };
        let mut enemy = Human { health: 12, damage: 7, armor: 2, wealth: 0 };

        attack(&human, &mut enemy);
        assert_eq!(enemy.health, 12 - (5 - 2)); // 9 left

        enemy.armor = 100; // boost
        attack(&human, &mut enemy);
        assert_eq!(enemy.health, 9 - 1);
    }

    #[test]
    fn test_find_cheapest_warrior() {
        let wealth = find_cheapest_warrior(
            r#"Hit Points: 100
            Damage: 8
            Armor: 2"#);

        assert_eq!(wealth, 91);
    }

    #[test]
    fn test_human_death() {
        let alive = Human::default();
        assert!(alive.dead());

        let mut dead = Human::default();
        dead.health = 0;
        assert!(!alive.dead());
    }
}
