use crate::day1::{count_brackets, count_position};

pub mod day1;

pub fn print_answers(day: u8, input: String) {
    let input = input.as_str();
    match day {
        1 => println!("Day1 / 2015: {} and {}", count_brackets(input), count_position(input)),
        _ => { eprintln!("2015: I don't know the answer for day {} :(", day) }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



