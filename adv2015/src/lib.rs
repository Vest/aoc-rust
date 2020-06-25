mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn print_answers(day: u8, input: String) {
    let input = input.as_str();
    let prefix = format!("Day {} / 2015:", day);
    match day {
        1 => println!("{} {} and {}", prefix, day1::count_brackets(input), day1::count_position(input)),
        2 => println!("{} {} and {}", prefix, day2::calc_packs(input), day2::calc_ribbons(input)),
        3 => println!("{} {} and {}", prefix, day3::count_houses(input), day3::count_houses_together(input)),
        4 => println!("{} {} and {}", prefix, day4::mine_suffix(input, 5), day4::mine_suffix(input, 6)),
        5 => println!("{} {} and {}", prefix, day5::count_nice_lines(input), day5::count_nice_lines_advanced(input)),
        6 => println!("{} {} and {}", prefix, day6::count_bulbs(input), day6::count_brightness(input)),
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



