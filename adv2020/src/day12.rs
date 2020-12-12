pub fn find_answer1(input: &str) -> i32 {
    let mut ship = Ship {
        position: (0, 0),
        direction: 0,
    };

    input.lines()
        .for_each(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let value_str = chars.collect::<String>();
            let value = value_str.parse::<i32>().unwrap();

            match direction {
                'N' => ship.position.1 += value,
                'S' => ship.position.1 -= value,
                'E' => ship.position.0 += value,
                'W' => ship.position.0 -= value,
                'L' => ship.direction += value,
                'R' => ship.direction -= value,
                'F' if ship.direction == 0 => ship.position.0 += value,
                'F' if ship.direction == 90 => ship.position.1 += value,
                'F' if ship.direction == 180 => ship.position.0 -= value,
                'F' if ship.direction == 270 => ship.position.1 -= value,
                'F' if ship.direction == -90 => ship.position.1 -= value,
                'F' if ship.direction == -180 => ship.position.0 -= value,
                'F' if ship.direction == -270 => ship.position.1 += value,
                _ => (
                    println!("pfffffff")
                ),
            }

            ship.direction %= 360;
        });

    ship.position.0.abs() + ship.position.1.abs()
}

struct Ship {
    position: (i32, i32),
    direction: i32,
}


struct Titanic {
    position: (i32, i32),
    waypoint: (i32, i32),
}

pub fn find_answer2(input: &str) -> i32 {
    let mut titanic = Titanic {
        position: (0, 0),
        waypoint: (10, 1),
    };

    input.lines()
        .for_each(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let value_str = chars.collect::<String>();
            let value = value_str.parse::<i32>().unwrap();
            let value_neg = -value;

            let ca = (value as f64).to_radians().cos() as i32;
            let sa = (value as f64).to_radians().sin() as i32;

            let cra = (value_neg as f64).to_radians().cos() as i32;
            let sra = (value_neg as f64).to_radians().sin() as i32;

            match direction {
                'N' => titanic.waypoint.1 += value,
                'S' => titanic.waypoint.1 -= value,
                'E' => titanic.waypoint.0 += value,
                'W' => titanic.waypoint.0 -= value,

                'L' => titanic.waypoint = (ca * titanic.waypoint.0 - sa * titanic.waypoint.1, sa * titanic.waypoint.0 + ca * titanic.waypoint.1),
                'R' => titanic.waypoint = (cra * titanic.waypoint.0 - sra * titanic.waypoint.1, sra * titanic.waypoint.0 + cra * titanic.waypoint.1),

                'F' => {
                    titanic.position.0 += value * titanic.waypoint.0;
                    titanic.position.1 += value * titanic.waypoint.1;
                }

                _ => (
                    println!("pfffffff")
                ),
            }

            println!("{} - {:?} {:?}", line, titanic.position, titanic.waypoint);
        });

    titanic.position.0.abs() + titanic.position.1.abs()
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
        assert_eq!(find_answer1(r#"F10
N3
F7
R90
F11"#), 25);
        assert_eq!(find_answer2(r#"F10
N3
F7
R90
F11"#), 286);
    }
}
