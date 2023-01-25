use itertools::iproduct;
use std::convert::TryFrom;

pub fn find_places_direct(input: &str) -> usize {
    let mut plane = Plane::new(input);
    let iter = plane.evo_direct_iter();
    iter.for_each(drop);

    plane.count_all()
}

pub fn find_seats_visible_only(input: &str) -> usize {
    let mut plane = Plane::new(input);
    let iter = plane.evo_visible_iter();
    iter.for_each(drop);

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
}

struct PlaneIter<'a> {
    plane: &'a mut Plane,
    count_fn: fn(&Plane, usize, usize) -> usize,
    should_seat_fn: fn(Seat, usize) -> Seat,

    round: usize,
}

impl Plane {
    fn new(input: &str) -> Plane {
        let seats: Vec<Vec<Seat>> = input
            .lines()
            .map(&str::trim)
            .map(|line| line.chars().map(Seat::from).collect())
            .collect();

        let rows = seats.len();
        let cols = seats.get(0).unwrap_or(&Vec::new()).len();

        Plane { seats, rows, cols }
    }

    fn get_seat(&self, row: i32, col: i32) -> &Seat {
        match (usize::try_from(row), usize::try_from(col)) {
            (Ok(row), Ok(col)) => self
                .seats
                .get(row)
                .and_then(|row| row.get(col))
                .unwrap_or(&Seat::NoSeat),
            _ => &Seat::NoSeat,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..(self.rows as i32) {
            for col in 0..(self.cols as i32) {
                print!(
                    "{}",
                    match self.get_seat(row, col) {
                        Seat::Vacant => 'L',
                        Seat::Occupied => '#',
                        Seat::NoSeat => '.',
                    }
                );
            }
            println!();
        }
    }

    fn count_all(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|&seat| seat == &Seat::Occupied).count())
            .sum()
    }

    fn evo_direct_iter(&mut self) -> PlaneIter {
        PlaneIter {
            plane: self,
            count_fn: count_direct_neighbours,
            should_seat_fn: should_seat_or_free_strict,
            round: 0,
        }
    }

    fn evo_visible_iter(&mut self) -> PlaneIter {
        PlaneIter {
            plane: self,
            count_fn: count_visible_neighbours,
            should_seat_fn: should_seat_or_free_less_strict,
            round: 0,
        }
    }
}

fn count_direct_neighbours(plane: &Plane, row: usize, col: usize) -> usize {
    iproduct!(-1..=1, -1..=1)
        .filter(|&(dr, dc)| !(dr == 0 && dc == 0)) // exclude center
        .filter(|&(dr, dc)| plane.get_seat(row as i32 + dr, col as i32 + dc) == &Seat::Occupied)
        .count()
}

fn should_seat_or_free_strict(current_seat: Seat, neighbours: usize) -> Seat {
    match current_seat {
        Seat::Vacant if neighbours == 0 => Seat::Occupied,
        Seat::Occupied if neighbours >= 4 => Seat::Vacant,
        seat => seat,
    }
}

fn should_seat_or_free_less_strict(current_seat: Seat, neighbours: usize) -> Seat {
    match current_seat {
        Seat::Vacant if neighbours == 0 => Seat::Occupied,
        Seat::Occupied if neighbours >= 5 => Seat::Vacant,
        seat => seat,
    }
}

fn count_visible_neighbours(plane: &Plane, row: usize, col: usize) -> usize {
    iproduct!(-1..=1, -1..=1)
        .filter(|&(dr, dc)| !(dr == 0 && dc == 0)) // exclude center
        .filter(|&(dr, dc)| {
            let (mut r, mut c) = (row as i32, col as i32);
            while r >= 0 && r < plane.rows as i32 && c >= 0 && c < plane.cols as i32 {
                r += dr;
                c += dc;

                match plane.get_seat(r, c) {
                    Seat::Occupied => {
                        return true;
                    }
                    Seat::Vacant => {
                        return false;
                    }
                    _ => (),
                }
            }

            false
        })
        .count()
}

impl<'a> Iterator for PlaneIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = Vec::<Vec<Seat>>::with_capacity(self.plane.rows);
        (0..self.plane.rows).for_each(|_| state.push(vec![Seat::NoSeat; self.plane.cols]));

        iproduct!(0..self.plane.rows, 0..self.plane.cols).for_each(|(row, col)| {
            let neighbours = (self.count_fn)(self.plane, row, col);
            let &current_seat = self.plane.get_seat(row as i32, col as i32);

            state[row][col] = (self.should_seat_fn)(current_seat, neighbours);
        });

        if state
            .iter()
            .zip(self.plane.seats.iter())
            .filter(|(r1, r2)| r1.eq(r2))
            .count()
            == self.plane.rows
        {
            return None;
        }

        self.plane.seats = state;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Debug, Formatter, Result, Write};

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
        assert_eq!(find_places_direct(""), 0);
        assert_eq!(find_seats_visible_only(""), 0);
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
    fn test_plane_direct_evolution() {
        let mut plane = Plane::new(INPUT);

        assert_eq!(plane.evo_direct_iter().max(), Some(5));
        assert_eq!(plane.count_all(), 37);
    }

    #[test]
    fn test_plane_visible_evolution() {
        let mut plane = Plane::new(INPUT);

        assert_eq!(plane.evo_visible_iter().max(), Some(6));
        assert_eq!(plane.count_all(), 26);

        plane.print();
    }

    #[test]
    fn test_count_visible_neighbours() {
        let plane1 = Plane::new(
            r#".......#.
                                        ...#.....
                                        .#.......
                                        .........
                                        ..#L....#
                                        ....#....
                                        .........
                                        #........
                                        ...#....."#,
        );
        assert_eq!(count_visible_neighbours(&plane1, 4, 3), 8);

        let plane2 = Plane::new(
            r#".............
                                        .L.L.#.#.#.#.
                                        ............."#,
        );
        assert_eq!(count_visible_neighbours(&plane2, 1, 1), 0);

        let plane3 = Plane::new(
            r#".##.##.
                                        #.#.#.#
                                        ##...##
                                        ...L...
                                        ##...##
                                        #.#.#.#
                                        .##.##."#,
        );
        assert_eq!(count_visible_neighbours(&plane3, 3, 3), 0);
    }

    #[test]
    fn test_seat_debug() {
        assert_eq!(
            format!("{:?}{:?}{:?}", Seat::Vacant, Seat::NoSeat, Seat::Occupied),
            "L.#"
        );
    }
}
