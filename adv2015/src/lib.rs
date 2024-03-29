/* <editor-fold desc="mod - days import"> */
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
/* </editor-fold> */

pub fn print_answers(day: u8, input: String) {
    // Trim is mandatory for one-line inputs
    let input = input.as_str().trim();
    let prefix = format!("Day {} / 2015:", day);
    match day {
        1 => println!(
            "{} {} and {}",
            prefix,
            day1::count_brackets(input),
            day1::count_position(input).unwrap_or(-1)
        ),
        2 => println!(
            "{} {} and {}",
            prefix,
            day2::calc_packs(input),
            day2::calc_ribbons(input)
        ),
        3 => println!(
            "{} {} and {}",
            prefix,
            day3::count_houses(input),
            day3::count_houses_together(input)
        ),
        4 => println!(
            "{} {} and {}",
            prefix,
            day4::mine_suffix(input, 5),
            day4::mine_suffix(input, 6)
        ),
        5 => println!(
            "{} {} and {}",
            prefix,
            day5::count_nice_lines(input),
            day5::count_nice_lines_advanced(input)
        ),
        6 => println!(
            "{} {} and {}",
            prefix,
            day6::count_bulbs(input),
            day6::count_brightness(input)
        ),
        7 => println!(
            "{} {} and {}",
            prefix,
            day7::count_input_a(input),
            day7::count_input_a_override(input)
        ),
        8 => println!(
            "{} {} and {}",
            prefix,
            day8::calc_difference(input),
            day8::calc_new_difference(input)
        ),
        9 => println!(
            "{} {} and {}",
            prefix,
            day9::calc_shortest(input),
            day9::calc_longest(input)
        ),
        10 => println!(
            "{} {} and {}",
            prefix,
            day10::calc_first(input),
            day10::calc_second(input)
        ),
        11 => println!(
            "{} {} and {}",
            prefix,
            day11::get_expired_once(input),
            day11::get_expired_twice(input)
        ),
        12 => println!(
            "{} {} and {}",
            prefix,
            day12::get_answer(input),
            day12::get_answer_without_red(input)
        ),
        13 => println!(
            "{} {} and {}",
            prefix,
            day13::get_answer(input),
            day13::get_answer_with_me(input)
        ),
        14 => println!(
            "{} {} and {}",
            prefix,
            day14::get_answer(input),
            day14::get_answer_points(input)
        ),
        15 => println!(
            "{} {} and {}",
            prefix,
            day15::get_answer(input),
            day15::get_answer_with_calories(input)
        ),
        16 => println!(
            "{} {} and {}",
            prefix,
            day16::get_answer(input),
            day16::get_answer_from_retroencabulator(input)
        ),
        17 => println!(
            "{} {} and {}",
            prefix,
            day17::get_total_count_of_combinations(input),
            day17::get_minimal_count_of_cans(input)
        ),
        18 => println!(
            "{} {} and {}",
            prefix,
            day18::get_answer_normal(input),
            day18::get_answer_broken(input)
        ),
        19 => println!(
            "{} {} and {}",
            prefix,
            day19::count_unique_molecules(input),
            day19::count_molecules_from_electron(input)
        ),
        20 => println!(
            "{} {} and {}",
            prefix,
            day20::get_richest_house_before_strike(input),
            day20::get_richest_house_after_strike(input)
        ),
        21 => println!(
            "{} {} and {}",
            prefix,
            day21::find_cheapest_warrior(input),
            day21::find_expensive_loser(input)
        ),
        22 => println!(
            "{} {} and {}",
            prefix,
            day22::find_easy_result(input),
            day22::find_hard_result(input)
        ),
        23 => println!(
            "{} {} and {}",
            prefix,
            day23::get_answer_b(input),
            day23::get_answer_b_after_a(input)
        ),
        24 => println!(
            "{} {} and {}",
            prefix,
            day24::find_answer(input),
            day24::find_answer_better(input)
        ),
        25 => println!("{} {}", prefix, day25::find_code(input)),
        _ => {
            eprintln!("2015: I don't know the answer for day {} :(", day)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_answers() {
        for i in 0..26 {
            let mut input = String::new();

            // Required to speed-up tests & increase the coverage
            if i == 4 {
                input = String::from("abcdef");
            } else if i == 7 {
                input = String::from("123 -> a");
            } else if i == 13 {
                input = String::from("Vest would gain 1 happiness units by sitting next to Vest.");
            } else if i == 15 {
                input = String::from(
                    "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
                );
            }

            print_answers(i, input);
        }
    }
}
