use std::convert::TryFrom;
use itertools::iproduct;

pub fn find_answer1(input: &str) -> usize {
    let mut plane = Plane::new(input);
    while let Some(_) = plane.next() {}

    plane.count_all()
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Seat {
    Vacant,
    Occupied,
    NoSeat,
}

struct Plane {
    seats: Vec<Vec<Seat>>,

    rows: usize,
    cols: usize,

    round: usize,
}

impl Plane {
    fn new(input: &str) -> Plane {
        let seats: Vec<Vec<Seat>> = input.lines()
            .map(&str::trim)
            .map(|line| line.chars()
                .map(Seat::from)
                .collect())
            .collect();

        let rows = seats.len();
        let cols = seats.get(0).unwrap_or(&Vec::new()).len();

        Plane {
            seats,

            rows,
            cols,

            round: 0,
        }
    }

    fn get_seat(&self, row: i32, col: i32) -> &Seat {
        if let Ok(u_row) = usize::try_from(row) {
            if let Ok(u_col) = usize::try_from(col) {
                if let Some(row) = self.seats.get(u_row) {
                    if let Some(seat) = row.get(u_col) {
                        return seat;
                    }
                }
            }
        }

        &Seat::NoSeat
    }

    fn count_direct_neighbours(&self, row: usize, col: usize) -> usize {
        iproduct!(-1..=1, -1..=1)
            .filter(|&(dr, dc)| !(dr == 0 && dc == 0)) // exclude center
            .filter(|&(dr, dc)| self.get_seat(row as i32 + dr, col as i32 + dc) == &Seat::Occupied)
            .count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..(self.rows as i32) {
            for col in 0..(self.cols as i32) {
                print!("{}", match self.get_seat(row, col) {
                    Seat::Vacant => 'L',
                    Seat::Occupied => '#',
                    Seat::NoSeat => '.',
                });
            }
            println!();
        }
    }

    fn count_all(&self) -> usize {
        self.seats.iter()
            .map(|row| row.iter()
                .filter(|&seat| seat == &Seat::Occupied)
                .count()
            ).sum()
    }
}

impl Iterator for Plane {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = Vec::<Vec<Seat>>::with_capacity(self.rows);
        (0..self.rows).for_each(|_| state.push(vec![Seat::NoSeat; self.cols]));

        iproduct!(0..self.rows, 0..self.cols)
            .for_each(|(row, col)| {
                let neighbours = self.count_direct_neighbours(row, col);

                state[row][col] = match self.get_seat(row as i32, col as i32) {
                    Seat::Vacant if neighbours == 0 => Seat::Occupied,
                    Seat::Occupied if neighbours >= 4 => Seat::Vacant,
                    seat => *seat,
                };
            });

        if state.iter().zip(self.seats.iter())
            .filter(|(r1, r2)|
                r1.eq(r2)
            ).count() == self.rows {
            return None;
        }

        self.seats = state;
        self.round += 1;
        Some(self.round)
    }
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Vacant,
            '#' => Self::Occupied,
            _ => Self::NoSeat,
        }
    }
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
    use std::fmt::{Debug, Formatter, Write, Result};

    const INPUT: &str = r#"L.LL.LL.LL
                           LLLLLLL.LL
                           L.L.L..L..
                           LLLL.LL.LL
                           L.LL.LL.LL
                           L.LLLLL.LL
                           ..L.L.....
                           LLLLLLLLLL
                           L.LLLLLL.L
                           L.LLLLL.LL"#;

    impl Debug for Seat {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_char(match self {
                Seat::Vacant => 'L',
                Seat::Occupied => '#',
                Seat::NoSeat => '.',
            })
        }
    }

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_answer1(""), 0);
        assert_eq!(find_answer2(""), 0);
    }

    #[test]
    fn test_plane() {
        let plane = Plane::new(INPUT);
        assert_eq!(plane.seats.len(), INPUT.lines().count());
        assert_eq!(plane.get_seat(-1, -1), &Seat::NoSeat);
        assert_eq!(plane.get_seat(0, 0), &Seat::Vacant);
        assert_eq!(plane.get_seat(100, 100), &Seat::NoSeat);
        assert_eq!(plane.get_seat(0, 5), &Seat::Vacant);
    }

    #[test]
    fn test_plane_evolution() {
        let mut plane = Plane::new(INPUT);
        while let Some(_) = plane.next() {
            println!();
            plane.print();
        };


        assert_eq!(plane.round, 5);
        assert_eq!(plane.count_all(), 37);
    }
}
