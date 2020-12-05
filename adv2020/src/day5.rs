use itertools::Itertools;

pub fn find_maximum_seat_id(input: &str) -> usize {
    parse_seats(input)
        .map(|seat| seat.id())
        .max()
        .unwrap_or_default()
}

pub fn find_your_seat(input: &str) -> usize {
   if let Err(result) = parse_seats(input)
        .map(|seat| seat.id())
        .sorted()
        .try_fold(0, |acc, item| {
            if item - acc == 1 || acc == 0 {
                Ok(item)
            } else {
                Err(acc + 1) // next ID after the last successful
            }
        }) {
       result
   } else {
       0
   }
}

fn parse_seats<'a>(input: &'a str) -> impl Iterator<Item=Seat> + 'a {
    input.lines()
        .map(&str::trim)
        .map(parse_seat)
}

struct Seat(usize, usize);

fn parse_seat(input: &str) -> Seat {
    let mut row = (0, 127usize);
    let mut column = (0, 7usize);

    input.chars()
        .map(|c| c.to_ascii_uppercase())
        .enumerate()
        .for_each(|(position, c)| {
            match position {
                0..=6 => {
                    let lower = (row.0, (row.1 + row.0 + 1) / 2 - 1);
                    let upper = ((row.1 + row.0 + 1) / 2, row.1);

                    row = if c == 'F' { lower } else { upper };
                }

                7..=9 => {
                    let lower = (column.0, (column.1 + column.0 + 1) / 2 - 1);
                    let upper = ((column.1 + column.0 + 1) / 2, column.1);

                    column = if c == 'L' { lower } else { upper };
                }

                _ => {}
            }
        });


    Seat(row.0, column.0)
}

impl Seat {
    fn id(&self) -> usize {
        self.0 * 8 + self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"BFFFBBFRRR
                                   FFFBBBFRRR
                                   BBFFBBFRLL"#;

    #[test]
    fn test_empty_answers() {
        assert_eq!(find_maximum_seat_id(""), 0);
        assert_eq!(find_your_seat(""), 0);
    }

    #[test]
    fn test_parse_seat() {
        let Seat(row, col) = parse_seat("FBFBBFFRLR");
        assert_eq!(row, 44);
        assert_eq!(col, 5);

        let Seat(row, col) = parse_seat("FFFBBBFRRRRRRRRRRR");
        assert_eq!(row, 14);
        assert_eq!(col, 7);
    }

    #[test]
    fn test_parse_seats() {
        let result = parse_seats(INPUT);
        assert_eq!(result.count(), 3);
    }

    #[test]
    fn test_find_maximum_seat_id() {
        assert_eq!(find_maximum_seat_id(INPUT), 820);
    }

    #[test]
    fn test_find_your_seat() {
        assert_eq!(find_your_seat("BBFFBBFRLL\nBBFFBBFRRL"), 821);
    }
}
