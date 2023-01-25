use itertools::iproduct;
use std::cmp::{max, min};
use std::collections::HashSet;

pub fn count_standard_cube(input: &str) -> usize {
    let mut cube = Cube::load_from_string(input);
    for _ in 1..=6 {
        cube.evolve(false);
    }

    cube.count_actives()
}

pub fn count_hyper_cube(input: &str) -> usize {
    let mut cube = Cube::load_from_string(input);
    for _ in 1..=6 {
        cube.evolve(true);
    }

    cube.count_actives()
}

#[derive(Eq, PartialEq, Hash)]
struct Coord(i32, i32, i32, i32);

struct Cube {
    dim_x: (i32, i32),
    dim_y: (i32, i32),
    dim_z: (i32, i32),
    dim_w: (i32, i32),
    cells: HashSet<Coord>,
}

impl Cube {
    fn new() -> Cube {
        Cube {
            dim_x: (0, 0),
            dim_y: (0, 0),
            dim_z: (0, 0),
            dim_w: (0, 0),
            cells: HashSet::new(),
        }
    }

    fn is_active(&self, coord: &Coord) -> bool {
        self.cells.contains(coord)
    }

    fn set(&mut self, coord: &Coord, active: bool) {
        match active {
            true => {
                let &Coord(x, y, z, w) = coord;
                self.cells.insert(Coord(x, y, z, w));

                if !(self.dim_x.0..=self.dim_x.1).contains(&x) {
                    self.dim_x.0 = min(self.dim_x.0, x);
                    self.dim_x.1 = max(self.dim_x.1, x);
                }

                if !(self.dim_y.0..=self.dim_y.1).contains(&y) {
                    self.dim_y.0 = min(self.dim_y.0, y);
                    self.dim_y.1 = max(self.dim_y.1, y);
                }

                if !(self.dim_z.0..=self.dim_z.1).contains(&z) {
                    self.dim_z.0 = min(self.dim_z.0, z);
                    self.dim_z.1 = max(self.dim_z.1, z);
                }

                if !(self.dim_w.0..=self.dim_w.1).contains(&w) {
                    self.dim_w.0 = min(self.dim_w.0, w);
                    self.dim_w.1 = max(self.dim_w.1, w);
                }
            }
            false => {
                self.cells.remove(coord);
            }
        };
    }

    fn load_from_string(input: &str) -> Cube {
        let mut cube = Cube::new();

        input
            .lines()
            .map(&str::trim)
            .enumerate()
            .for_each(|(row, line)| {
                line.chars().enumerate().for_each(|(col, c)| match c {
                    '#' => cube.set(&Coord(col as i32, row as i32, 0, 0), true),
                    _ => (),
                })
            });

        cube
    }

    fn evolve(&mut self, is_hyper: bool) {
        let mut new_cube = Cube::new();

        iproduct!(
            self.dim_x.0 - 1..=self.dim_x.1 + 1,
            self.dim_y.0 - 1..=self.dim_y.1 + 1,
            self.dim_z.0 - 1..=self.dim_z.1 + 1,
            self.dim_w.0 - 1..=self.dim_w.1 + 1
        )
        .filter(|&(_, _, _, w)| is_hyper || w == 0)
        .for_each(|(x, y, z, w)| {
            let coord = Coord(x, y, z, w);
            let should_activate = self.evolve_cell(&coord, is_hyper);

            new_cube.set(&coord, should_activate);
        });

        self.cells = new_cube.cells;
        self.dim_x = new_cube.dim_x;
        self.dim_y = new_cube.dim_y;
        self.dim_z = new_cube.dim_z;
        self.dim_w = new_cube.dim_w;
    }

    fn evolve_cell(&self, coord: &Coord, is_hyper: bool) -> bool {
        let &Coord(cell_x, cell_y, cell_z, cell_w) = coord;

        let neighbours_count = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter(|&(_, _, _, w)| is_hyper || w == 0)
            .filter(|&(dx, dy, dz, dw)| !(dx == 0 && dy == 0 && dz == 0 && dw == 0)) // exclude center
            .filter(|&(dx, dy, dz, dw)| {
                self.is_active(&Coord(cell_x + dx, cell_y + dy, cell_z + dz, cell_w + dw))
            })
            .count();

        let current_cell = self.is_active(coord);

        match current_cell {
            true => neighbours_count == 2 || neighbours_count == 3,
            false => neighbours_count == 3,
        }
    }

    fn count_actives(&self) -> usize {
        self.cells.len()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for w in self.dim_w.0..=self.dim_w.1 {
            for z in self.dim_z.0..=self.dim_z.1 {
                println!("z={}, w={}", z, w);
                for y in self.dim_y.0..=self.dim_y.1 {
                    for x in self.dim_x.0..=self.dim_x.1 {
                        let coord = Coord(x, y, z, w);

                        print!("{}", if self.is_active(&coord) { '#' } else { '.' });
                    }
                    println!();
                }

                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".#.
                        ..#
                        ###"#;

    #[test]
    fn test_empty_answers() {
        assert_eq!(count_standard_cube(""), 0);
        assert_eq!(count_hyper_cube(""), 0);
    }

    #[test]
    fn test_cube_set() {
        let mut cube = Cube::new();
        assert!(cube.cells.is_empty());
        assert!(!cube.is_active(&Coord(0, 0, 0, 0)));

        cube.set(&Coord(0, 0, 0, 0), true);
        assert!(cube.is_active(&Coord(0, 0, 0, 0)));

        let coord123 = Coord(1, 2, 3, 0);
        cube.set(&coord123, true);
        assert_eq!(cube.dim_x, (0, 1));
        assert_eq!(cube.dim_y, (0, 2));
        assert_eq!(cube.dim_z, (0, 3));

        cube.set(&coord123, false);
        assert!(!cube.is_active(&coord123));
        assert_eq!(cube.dim_x, (0, 1));
        assert_eq!(cube.dim_y, (0, 2));
        assert_eq!(cube.dim_z, (0, 3));
    }

    #[test]
    fn test_load_from_string() {
        let cube = Cube::load_from_string(INPUT);
        assert_eq!(cube.dim_x, (0, 2));
        assert_eq!(cube.dim_y, (0, 2));
        assert_eq!(cube.dim_z, (0, 0));
        assert_eq!(cube.dim_w, (0, 0));

        assert!(!cube.is_active(&Coord(0, 0, 0, 0)));
        assert!(cube.is_active(&Coord(1, 0, 0, 0)));
        assert!(!cube.is_active(&Coord(2, 0, 0, 0)));
    }

    #[test]
    fn test_evolve() {
        let mut cube = Cube::load_from_string(INPUT);

        cube.evolve(false);
        cube.print();

        assert!(cube.is_active(&Coord(0, 1, 0, 0)));
        assert!(!cube.is_active(&Coord(1, 1, 0, 0)));
        assert!(cube.is_active(&Coord(2, 1, 0, 0)));

        assert!(!cube.is_active(&Coord(0, 2, 0, 0)));
        assert!(cube.is_active(&Coord(1, 2, 0, 0)));
        assert!(cube.is_active(&Coord(2, 2, 0, 0)));

        assert!(!cube.is_active(&Coord(0, 3, 0, 0)));
        assert!(cube.is_active(&Coord(1, 3, 0, 0)));
        assert!(!cube.is_active(&Coord(2, 3, 0, 0)));

        assert_eq!(cube.count_actives(), 11);

        for _ in 2..=6 {
            cube.evolve(false);
        }

        assert_eq!(cube.count_actives(), 112);
    }

    #[test]
    fn test_evolve_hyper_cube() {
        let mut cube = Cube::load_from_string(INPUT);

        for _ in 1..=6 {
            cube.evolve(true);
        }

        assert_eq!(cube.count_actives(), 848);
    }
}
