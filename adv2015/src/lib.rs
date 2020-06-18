use crate::day1::get_answer;

pub mod day1;

pub fn print_answers(session: String) {
    println!("Day1 / 2015: {}", get_answer(session));
}

#[cfg(test)]
mod tests {
    use crate::day1::print_answer;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}



