pub fn find_answer1(input: &str) -> usize {
    0
}

pub fn find_answer2(input: &str) -> usize {
    0
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
        assert_eq!(find_answer1(""), 0);
        assert_eq!(find_answer2(""), 0);
    }
}
