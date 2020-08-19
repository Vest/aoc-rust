const SIZE: usize = 100;
const COUNT: usize = 100;

pub fn get_answer_normal(input: &str) -> usize {
    let mut grid = Grid::new(SIZE);

    grid.parse_grid(input);

    for _ in 0..COUNT {
        grid.evolve();
    }

    grid.count_lights()
}

pub fn get_answer_broken(input: &str) -> usize {
    let mut grid = Grid::new(SIZE);

    grid.parse_grid(input);
    grid.break_circuit();

    for _ in 0..COUNT {
        grid.evolve();
        grid.break_circuit();
    }

    grid.count_lights()
}

struct Grid {
    size: usize,
    grid: Vec<Vec<bool>>,
}


impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            size,
            grid: vec![vec![false; size]; size],
        }
    }

    fn is_on(&self, row: usize, col: usize) -> Option<&bool> {
        if let Some(row_vec) = self.grid.get(row) {
            row_vec.get(col)
        } else {
            None
        }
    }

    fn neighbors_count(&self, row: usize, col: usize) -> usize {
        let mut results: Vec<Option<&bool>> = Vec::with_capacity(8);

        // 7-8-9
        if let (Some(row7), Some(col7)) = (row.checked_sub(1), col.checked_sub(1)) {
            results.push(self.is_on(row7, col7));
        }

        if let (Some(row8), Some(col8)) = (row.checked_sub(1), Some(col)) {
            results.push(self.is_on(row8, col8));
        }

        if let (Some(row9), Some(col9)) = (row.checked_sub(1), col.checked_add(1)) {
            results.push(self.is_on(row9, col9));
        }

        // 4- -6
        if let (Some(row4), Some(col4)) = (Some(row), col.checked_sub(1)) {
            results.push(self.is_on(row4, col4));
        }

        if let (Some(row6), Some(col6)) = (Some(row), col.checked_add(1)) {
            results.push(self.is_on(row6, col6));
        }

        // 1-2-3
        if let (Some(row1), Some(col1)) = (row.checked_add(1), col.checked_sub(1)) {
            results.push(self.is_on(row1, col1));
        }

        if let (Some(row2), Some(col2)) = (row.checked_add(1), Some(col)) {
            results.push(self.is_on(row2, col2));
        }

        if let (Some(row3), Some(col3)) = (row.checked_add(1), col.checked_add(1)) {
            results.push(self.is_on(row3, col3));
        }

        results.iter()
            .filter(|&r| r.is_some())
            .map(|r| r.unwrap())
            .filter(|&r| *r)
            .count()
    }

    fn parse_grid(&mut self, lines: &str) {
        lines.lines()
            .enumerate()
            .for_each(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .for_each(|(col, light)| {
                        self.grid[row][col] = light == '#';
                    })
            });
    }

    fn evolve(&mut self) {
        let mut new_grid = vec![vec![false; self.size]; self.size];

        for row in 0..self.size {
            for col in 0..self.size {
                let count = self.neighbors_count(row, col);

                new_grid[row][col] = match self.is_on(row, col).unwrap() {
                    true => count == 2 || count == 3,
                    false => count == 3,
                }
            }
        }

        self.grid = new_grid;
    }

    fn count_lights(&self) -> usize {
        self.grid.iter()
            .map(|line|
                line.iter()
                    .filter(|&light| *light)
                    .count()
            )
            .sum()
    }

    fn break_circuit(&mut self) {
        self.grid[0][0] = true;
        self.grid[0][self.size - 1] = true;
        self.grid[self.size - 1][0] = true;
        self.grid[self.size - 1][self.size - 1] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".#.#.#
                             ...##.
                             #....#
                             ..#...
                             #.#..#
                             ####.."#;

    #[test]
    fn test_build_grid() {
        let grid = Grid::new(6);
        assert_eq!(grid.size, 6);

        for row in 0..grid.size {
            for col in 0..grid.size {
                assert!(!grid.is_on(row, col).unwrap());
            }
        }
    }

    #[test]
    fn test_neighbors_count_zero() {
        let grid = Grid::new(6);
        for row in 0..grid.size {
            for col in 0..grid.size {
                assert_eq!(grid.neighbors_count(row, col), 0);
            }
        }
    }

    #[test]
    fn test_neighbors_count_top_left() {
        let mut grid = Grid::new(6);
        grid.grid[0][1] = true;
        grid.grid[1][0] = true;
        grid.grid[1][1] = true;
        assert_eq!(grid.neighbors_count(0, 0), 3);
    }

    #[test]
    fn test_neighbors_count_top_right() {
        let mut grid = Grid::new(6);
        grid.grid[0][4] = true;
        grid.grid[1][4] = true;
        grid.grid[1][5] = true;
        assert_eq!(grid.neighbors_count(0, 5), 3);
    }

    #[test]
    fn test_neighbors_count_middle() {
        let mut grid = Grid::new(6);
        grid.grid[2][2] = true;
        grid.grid[4][4] = true;
        grid.grid[4][2] = true;
        grid.grid[2][4] = true;
        assert_eq!(grid.neighbors_count(3, 3), 4);
    }

    #[test]
    fn test_parse_grid() {
        let mut grid = Grid::new(6);
        grid.parse_grid(EXAMPLE);

        assert_eq!(grid.count_lights(), 15);
    }

    #[test]
    fn test_parse_empty_grid() {
        let mut grid = Grid::new(6);
        grid.parse_grid("");

        assert_eq!(grid.count_lights(), 0);
    }

    #[test]
    fn test_evolve() {
        let mut grid = Grid::new(6);
        grid.parse_grid(EXAMPLE);

        grid.evolve();
        grid.evolve();
        grid.evolve();
        grid.evolve();

        assert_eq!(grid.count_lights(), 4);

        assert_eq!(grid.is_on(2, 2), Some(&true));
        assert_eq!(grid.is_on(2, 3), Some(&true));
        assert_eq!(grid.is_on(3, 2), Some(&true));
        assert_eq!(grid.is_on(3, 3), Some(&true));
    }

    #[test]
    fn test_empty_get_answer_normal() {
        assert_eq!(get_answer_normal(""), 0);
    }

    #[test]
    fn test_empty_get_answer_broken() {
        assert_eq!(get_answer_broken(""), 4);
    }

    #[test]
    fn test_break_circuit() {
        let mut grid = Grid::new(6);
        assert_eq!(grid.count_lights(), 0);

        grid.break_circuit();
        assert_eq!(grid.count_lights(), 4);
    }
}
