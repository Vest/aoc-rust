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
    counter: u32,
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
}
