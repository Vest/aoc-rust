const FIRST_CODE: usize = 20151125;
const MULTIPLY: usize = 252533;
const DIVIDE: usize = 33554393;

pub fn find_code(input: &str) -> usize {
    let target = parse_input(input);
    let list = ListCodes::new(&target);
    let mut result = 0usize;

    for code in list {
        result = code;
    }

    result
}

fn parse_input(input: &str) -> Coord {
    let result: Vec<usize> = input.split(|c: char| c.is_whitespace() || c.is_alphabetic() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if result.len() != 2 {
        return Coord { row: 0, col: 0 };
    }

    Coord {
        row: result[0],
        col: result[1],
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

struct ListCodes {
    current_pos: Coord,
    target_pos: Coord,
    current_code: usize,
    length: usize,
    stop: bool,
}

impl ListCodes {
    fn new(target_pos: &Coord) -> ListCodes {
        ListCodes {
            current_pos: Coord { row: 0, col: 0 },
            target_pos: target_pos.clone(),
            current_code: 0,
            length: 0,
            stop: false,
        }
    }
}

impl Iterator for ListCodes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.stop |= self.current_pos == self.target_pos;

        if self.stop {
            return None;
        }

        // move
        if self.current_pos.row <= 1 {
            self.length += 1;
            self.current_pos.row = self.length;
            self.current_pos.col = 1;
        } else {
            self.current_pos.row -= 1;
            self.current_pos.col += 1;
        }

        self.current_code = if (self.current_pos == Coord { row: 1, col: 1 }) {
            FIRST_CODE
        } else {
            find_next_code(self.current_code)
        };

        if self.current_code == 0 {
            return None;
        }

        Some(self.current_code)
    }
}

fn find_next_code(code: usize) -> usize {
    let temp = code.checked_mul(MULTIPLY).unwrap_or_default();
    temp.checked_rem(DIVIDE).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let coord = parse_input("To continue, please consult the code grid in the manual.  Enter the code at row 2947, column 3029.");
        assert_eq!(coord.row, 2947);
        assert_eq!(coord.col, 3029);

        assert_eq!(coord, Coord { row: 2947, col: 3029 });
    }

    #[test]
    fn test_parse_bad_input() {
        let coord = parse_input("To continue, Test 123 please consult the code grid in the manual.  Enter the code at row 2947, column 3029.");
        assert_eq!(coord.row, 0);
        assert_eq!(coord.col, 0);
    }

    #[test]
    fn test_find_next_code() {
        assert_eq!(find_next_code(FIRST_CODE), 31916031usize);
    }

    #[test]
    fn test_list_codes() {
        let mut list = ListCodes::new(&Coord { row: 2, col: 2 });
        assert_eq!(list.next(), Some(20151125));
        assert_eq!(list.next(), Some(31916031));
        assert_eq!(list.next(), Some(18749137));
        assert_eq!(list.next(), Some(16080970));
        assert_eq!(list.next(), Some(21629792));
        assert_eq!(list.next(), None);
    }
}
