use std::cmp::min;

const TOTAL_TIME: usize = 2503;

pub fn get_answer(input: &str) -> usize {
    find_fastest_deer(input, TOTAL_TIME)
}

pub fn get_answer_points(input: &str) -> usize {
    deer_race(input, TOTAL_TIME)
}

struct RangiferTarandus {
    _name: String,
    speed: usize,
    endurance: usize,
    sleep: usize,

    is_running: bool,
    running_time: usize,
    sleeping_time: usize,
    distance: usize,
    points: usize,
}

fn parse_line(input: &str) -> RangiferTarandus {
    let mut split = input.split_whitespace();
    let name = String::from(split.next().unwrap());
    let numbers: Vec<usize> = split.filter_map(|s| s.parse().ok())
        .collect();

    RangiferTarandus {
        _name: name,
        speed: numbers[0],
        endurance: numbers[1],
        sleep: numbers[2],

        is_running: true,
        running_time: 0,
        sleeping_time: 0,
        distance: 0,
        points: 0,
    }
}

fn calculate_deer(deer: &RangiferTarandus, time: usize) -> usize {
    let total_time = deer.endurance + deer.sleep;
    let cycle_count = time / total_time;
    let rest = time % total_time;

    let distance1 = deer.speed * deer.endurance * cycle_count;
    let distance2 = deer.speed * min(deer.endurance, rest);

    distance1 + distance2
}

fn find_fastest_deer(input: &str, total: usize) -> usize {
    input.lines()
        .map(|l| parse_line(l))
        .map(|d| calculate_deer(&d, total))
        .max()
        .unwrap()
}

fn deer_race(input: &str, duration: usize) -> usize {
    let mut deers: Vec<RangiferTarandus> = input.lines()
        .map(|l| parse_line(l))
        .collect();

    for _ in 0..duration + 1 {
        deers.iter_mut()
            .for_each(|d| {
                if d.running_time == d.endurance {
                    d.is_running = false;
                    d.sleeping_time = 0;
                    d.running_time = 0;
                }

                if d.sleeping_time == d.sleep {
                    d.is_running = true;
                    d.sleeping_time = 0;
                    d.running_time = 0;
                }

                if d.is_running {
                    d.distance += d.speed;
                    d.running_time += 1;
                } else {
                    d.sleeping_time += 1;
                }
            });

        if let Some(fastest_deer) = deers.iter_mut()
            .max_by_key(|d| d.distance) {
            fastest_deer.points += 1;
        }
    }

    deers.iter()
        .max_by_key(|d| d.points)
        .unwrap()
        .points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let result = parse_line(input);

        assert_eq!(result._name, "Comet");
        assert_eq!(result.speed, 14);
        assert_eq!(result.endurance, 10);
        assert_eq!(result.sleep, 127);
    }

    #[test]
    fn test_calculate_deer() {
        let input1 = parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.");
        let input2 = parse_line("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");

        let result1 = calculate_deer(&input1, 1000);
        let result2 = calculate_deer(&input2, 1000);

        assert_eq!(result1, 1120);
        assert_eq!(result2, 1056);
    }

    #[test]
    fn test_find_fastest_deer() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
            Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

        let result = find_fastest_deer(input, 1000);

        assert_eq!(result, 1120);
    }

    #[test]
    fn test_deer_race() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
            Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

        let result = deer_race(input, 1000);

        assert_eq!(result, 689);
    }
}
