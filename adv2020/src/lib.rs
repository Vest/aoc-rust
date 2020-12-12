/* <editor-fold desc="mod - days import"> */
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
/* </editor-fold> */

pub fn print_answers(day: u8, input: String) {
    // Trim is mandatory for one-line inputs
    let input = input.as_str().trim();
    let prefix = format!("Day {} / 2020:", day);
    match day {
        1 => println!("{} {} and {}", prefix, day1::find_expenses(input), day1::find_more_expenses(input)),
        2 => println!("{} {} and {}", prefix, day2::count_simple_passwords(input), day2::count_complex_passwords(input)),
        3 => println!("{} {} and {}", prefix, day3::count_single_slope(input), day3::count_multiple_slopes(input)),
        4 => println!("{} {} and {}", prefix, day4::count_simple_results(input), day4::count_advanced_results(input)),
        5 => println!("{} {} and {}", prefix, day5::find_maximum_seat_id(input), day5::find_your_seat(input)),
        6 => println!("{} {} and {}", prefix, day6::find_answer1(input), day6::find_answer2(input)),
        7 => println!("{} {} and {}", prefix, day7::find_answer1(input), day7::find_answer2(input)),
        8 => println!("{} {} and {}", prefix, day8::execute_first_program(input), day8::execute_second_program(input)),
        9 => println!("{} {} and {}", prefix, day9::find_weak_number_25(input), day9::find_sum_of_any_numbers(input)),
        10 => println!("{} {} and {}", prefix, day10::find_one_by_three(input), day10::find_answer2(input)),
        11 => println!("{} {} and {}", prefix, day11::find_answer1(input), day11::find_answer2(input)),
        12 => println!("{} {} and {}", prefix, day12::find_answer1(input), day12::find_answer2(input)),
       // 9 => println!("{} {} and {}", prefix, day9::find_answer1(input), day9::find_answer2(input)),

        _ => { eprintln!("2020: I don't know the answer for day {} :(", day) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_answers() {
        for i in 0..26 {
            let input = String::new();

            // Required to speed-up tests & increase the coverage
            /*
            if i == 4 {
                input = String::from("abcdef");
            }
            */

            print_answers(i, input);
        }
    }
}
