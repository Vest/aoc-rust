/* <editor-fold desc="mod - days import"> */
mod day1;
/* </editor-fold> */

pub fn print_answers(day: u8, input: String) {
    // Trim is mandatory for one-line inputs
    let input = input.as_str().trim();
    let prefix = format!("Day {} / 2020:", day);
    match day {
        1 => println!("{} {} and {}", prefix, day1::answer1(input), day1::answer2(input)),
        _ => { eprintln!("2020: I don't know the answer for day {} :(", day) }
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
            /*
            if i == 4 {
                input = String::from("abcdef");
            }
            */

            print_answers(i, input);
        }
    }
}
