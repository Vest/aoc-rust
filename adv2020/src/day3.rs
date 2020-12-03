pub fn count_single_slope(input: &str) -> usize {
    const SINGLE: Slope = Slope { right: 3, down: 1 };

    let input = parse_input(input);
    count_trees(&input, &SINGLE)
}

pub fn count_multiple_slopes(input: &str) -> usize {
    let multiple: Vec<Slope> = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let input = parse_input(input);

    multiple.iter()
        .map(|slope| count_trees(&input, slope))
        .fold(1, |count, acc| count * acc)
}

fn count_trees(field: &Vec<Vec<Cell>>, slope: &Slope) -> usize {
    let mut col = 0usize;

    field.iter()
        .enumerate()
        .filter(|&row| {
            if row.0 % slope.down == 0 {
                let is_tree = row.1.get(col % row.1.len())
                    .unwrap_or(&Cell::Empty) == &Cell::Tree;
                col += slope.right;

                is_tree
            } else { false }
        }).count()
}

#[derive(PartialEq)]
enum Cell {
    Empty,
    Tree,
}

struct Slope {
    right: usize,
    down: usize,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input.lines()
        .map(|l| l.trim()
            .chars()
            .map(parse_cell)
            .collect())
        .collect()
}

fn parse_cell(cell: char) -> Cell {
    match cell {
        '#' => Cell::Tree,
        _ => Cell::Empty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    const INPUT: &str = r#"..##.......
                           #...#...#..
                           .#....#..#.
                           ..#.#...#.#
                           .#...##..#.
                           ..#.##.....
                           .#.#.#....#
                           .#........#
                           #.##...#...
                           #...##....#
                           .#..#...#.#"#;

    impl fmt::Debug for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Cell::Empty => ".",
                Cell::Tree => "#",
            })
        }
    }

    #[test]
    fn test_answers() {
        assert_eq!(count_single_slope(""), 0);
        assert_eq!(count_multiple_slopes(""), 0);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input("#...#");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[0][0], Cell::Tree);
        assert_eq!(result[0][3], Cell::Empty);
        assert_eq!(result[0][4], Cell::Tree);
    }

    #[test]
    fn test_cell_debug() {
        assert_eq!(format!("{:?}", Cell::Tree), "#");
        assert_eq!(format!("{:?}", Cell::Empty), ".");
    }

    #[test]
    fn test_count_trees() {
        let field = parse_input(INPUT);
        let count = count_trees(&field, &Slope { right: 3, down: 1 });

        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_trees_advanced() {
        let field = parse_input(INPUT);
        assert_eq!(count_trees(&field, &Slope { right: 1, down: 1 }), 2);
        assert_eq!(count_trees(&field, &Slope { right: 5, down: 1 }), 3);
        assert_eq!(count_trees(&field, &Slope { right: 1, down: 2 }), 2);
    }

    #[test]
    fn test_count_single_slope() {
        assert_eq!(count_single_slope(INPUT), 7);
    }

    #[test]
    fn test_count_multiple_slopes() {
        assert_eq!(count_multiple_slopes(INPUT), 336);
    }
}
