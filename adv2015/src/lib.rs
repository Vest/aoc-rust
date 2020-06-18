use crate::day1::{get_answer1, get_answer2};

pub mod day1;

pub fn print_answers(session: String) {
    println!("Day1 / 2015: {} and {}", get_answer1(session.clone()), get_answer2(session.clone()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



