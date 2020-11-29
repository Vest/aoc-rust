const FIRST_CODE: usize = 20151125;
const MULTIPLY: usize = 252533;
const DIVIDE: usize = 33554393;

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

struct Coord {
    row: usize,
    col: usize,
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
}
