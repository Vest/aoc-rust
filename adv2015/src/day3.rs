#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn step(&mut self, dir: char) {
        match dir {
            '^' => self.y += 1,
            '>' => self.x += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            _ => (),
        };
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn count_houses(path: &str) -> usize {
    let mut houses: Vec<Coord> = Vec::new();
    houses.push(Coord { x: 0, y: 0 });

    let mut house = Coord { x: 0, y: 0 };

    for c in path.chars() {
        house.step(c);

        if !houses.contains(&house) {
            houses.push(house);
        }
    }

    houses.len()
}

pub fn count_houses_together(path: &str) -> usize {
    let mut houses: Vec<Coord> = Vec::new();
    houses.push(Coord { x: 0, y: 0 });

    let mut santa = Coord { x: 0, y: 0 };
    let mut robot = Coord { x: 0, y: 0 };

    let mut santa_turn = true;

    for c in path.chars() {
        if santa_turn {
            santa.step(c);
            if !houses.contains(&santa) {
                houses.push(santa);
            }
        } else {
            robot.step(c);
            if !houses.contains(&robot) {
                houses.push(robot);
            }
        }

        santa_turn = !santa_turn;
    }

    houses.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let mut c = Coord { x: 0, y: 0 };
        c.step('<');
        c.step('>');
        c.step('o');
        c.step('v');
        c.step('^');
        assert_eq!(c, Coord { x: 0, y: 0 }, "We should arrive to (0, 0) instead of {:?}", c);
    }

    #[test]
    fn test_two_steps() {
        let mut c = Coord { x: 0, y: 0 };
        c.step('<');
        c.step('^');
        assert_eq!(c, Coord { x: -1, y: 1 }, "We should arrive to (-1, 1) instead of {:?}", c);
    }

    #[test]
    fn test_count_houses() {
        assert_eq!(count_houses(""), 1, "We always start with one house");

        assert_eq!(count_houses(">"), 2, "> - 2");
        assert_eq!(count_houses("^>v<"), 4, "^>v< - 4");
        assert_eq!(count_houses("^v^v^v^v^v"), 2, "^v^v^v^v^v - 2");
    }

    #[test]
    fn test_count_houses_together() {
        assert_eq!(count_houses_together(""), 1, "We always start with one house");

        assert_eq!(count_houses_together("^v"), 3, "^v - 3");
        assert_eq!(count_houses_together("^>v<"), 3, "^>v< - 3");
        assert_eq!(count_houses_together("^v^v^v^v^v"), 11, "^v^v^v^v^v - 11");
    }
}

