mod advent;

use std::env;

fn main() {
    println!("Advent 2015!");
    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            let session = session_value.as_str();
            let mut calculate_all = true;

            let args: Vec<String> = env::args().collect();
            if args.len() == 3 {
                let day_key = &args[1];
                let day_number = &args[2];

                if day_key.eq("-d") || day_key.eq("--day") {
                    if let Ok(day) = day_number.parse::<u8>() {
                        if let Ok(input) = advent::get_input(day, 2015, session) {
                            calculate_all = false;
                            adv2015::print_answers(day, input);
                        }
                    }
                }
            }

            if calculate_all {
                for day in 1..26 {
                    match advent::get_input(day, 2015, session) {
                        Ok(input) => adv2015::print_answers(day, input),
                        Err(_) => eprintln!("Couldn't get Input value for day {} / 2015.", day),
                    }
                }
            }
        }
        Err(e) => println!("Couldn't get {} key from environment variable. Description: {}", key, e),
    }

    println!("Done");
}
