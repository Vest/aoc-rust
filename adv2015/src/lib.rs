mod day2;
mod day1;

pub fn print_answers(day: u8, input: String) {
    let input = input.as_str();
    let prefix = format!("Day {} / 2015:", day);
    match day {
        1 => println!("{} {} and {}", prefix, day1::count_brackets(input), day1::count_position(input)),
        2 => println!("{} {} and {}", prefix, day2::calc_packs(input), day2::calc_ribbons(input)),
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



