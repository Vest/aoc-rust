pub fn find_answer1(input: &str) -> usize {
    let mut input: Vec<Vec<char>> =
        input.lines()
            .map(|line| line.chars()
                .collect())
            .collect();

    let mut result = 0usize;

    for i in 0..10 {
        let copy = input.clone();

        for row in 0..copy.len() {
            for col in 0..copy[0].len() {
                let up = (row.checked_sub(1), Some(col));
                let upleft = (row.checked_sub(1), col.checked_sub(1));
                let upright = (row.checked_sub(1), col.checked_add(1));

                let down = (row.checked_add(1), Some(col));
                let downleft = (row.checked_add(1), col.checked_sub(1));
                let downright = (row.checked_add(1), col.checked_add(1));

                let left = (Some(row), col.checked_sub(1));
                let right = (Some(row), col.checked_add(1));

                let neighbors: Vec<char> = vec![get_cell(&copy, up), get_cell(&copy, upleft),
                                                get_cell(&copy, upright), get_cell(&copy, left),
                                                get_cell(&copy, right), get_cell(&copy, down),
                                                get_cell(&copy, downleft), get_cell(&copy, downright)]
                    .iter()
                    .filter_map(|i| *i)
                    .collect();


                if copy[row][col] == '.' {
                    continue;
                } else if copy[row][col] == 'L' {
                    if neighbors.iter().filter(|c| *c == &'#').count() == 0 {
                        input[row][col] = '#'
                    }
                } else if copy[row][col] == '#' {
                    if neighbors.iter().filter(|c| *c == &'#').count() >= 4 {
                        input[row][col] = 'L'
                    }
                }
            }
        }

        for row in 0..copy.len() {
            for col in 0..copy[0].len() {
                print!("{}", input[row][col]);
            }
            println!();
        }
        println!();


        if input.iter()
            .zip(copy)
            .all(|(a, b)| a.iter().zip(b).all(|(c, d)| *c == d)) {
            return result;
        } else {
            result += 1;
        }
    }

    result
}

enum Seat {
    Vacant,
    Occupied,
    NoSeat,
}


fn get_cell(input: &Vec<Vec<char>>, coord: (Option<usize>, Option<usize>)) -> Option<char> {
    if coord.0.is_none() || coord.1.is_none() {
        return None;
    }

    let (row, col) = (coord.0.unwrap(), coord.1.unwrap());
    if row > (input.len() - 1) {
        return None;
    }
    if col > (input[0].len() - 1) {
        return None;
    }

    return Some(input[row][col]);
}

pub fn find_answer2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#), 37);
        assert_eq!(find_answer2(""), 0);
    }
}
