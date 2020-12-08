use std::collections::HashSet;

pub fn find_answer1(input: &str) -> i32 {
    let mut acc = 0i32;
    let mut cursor: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();

    let vec: Vec<(String, i32)> = input.lines()
        .map(|line: &str| {
            let mut split = line.split_whitespace();
            let command = String::from(split.next().unwrap());
            let value = split.next().unwrap().parse::<i32>().unwrap();

            (command, value)
        }).collect();

    loop {
        let (command, value) = &vec[cursor as usize];
        let command = command.as_str();
        if set.contains(&cursor) {
            break;
        } else {
            set.insert(cursor);
        }

        match command {
            "acc" => {
                acc += value;
            }
            "jmp" => {
                cursor += value - 1;
            }
            _ => {}
        }

        cursor += 1;
    }

    acc
}

pub fn find_answer2(input: &str) -> i32 {
    let mut acc = 0i32;
    let mut cursor: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();

    let mut vec: Vec<(String, i32)> = input.lines()
        .map(|line: &str| {
            let mut split = line.split_whitespace();
            let command = String::from(split.next().unwrap());
            let value = split.next().unwrap().parse::<i32>().unwrap();

            (command, value)
        }).collect();

    for i in 0..vec.len() {
        set.clear();
        acc = 0;
        cursor = 0;

        if vec[i].0 == String::from("jmp") {
            vec[i].0 = String::from("nop");
        } else if vec[i].0 == String::from("nop") {
            vec[i].0 = String::from("jmp");
        } else {
            continue;
        }

        loop {
            if cursor as usize >= vec.len() {
                return acc;
            }

            if set.contains(&cursor) {
                break;
            } else {
                set.insert(cursor);
            }

            let (command, value) = &vec[cursor as usize];
            let command = command.as_str();

            match command {
                "acc" => {
                    acc += value;
                }
                "jmp" => {
                    cursor += value - 1;
                }
                _ => {}
            }

            cursor += 1;
        }

        if vec[i].0 == String::from("jmp") {
            vec[i].0 = String::from("nop");
        } else if vec[i].0 == String::from("nop") {
            vec[i].0 = String::from("jmp");
        }
    }

    acc
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
        assert_eq!(find_answer1(r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#), 5);
        assert_eq!(find_answer2(r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#), 8);
    }
}
