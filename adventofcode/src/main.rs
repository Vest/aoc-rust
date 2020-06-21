mod advent;

use std::env;

fn main() {
    println!("Advent 2015!");
    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            let session = session_value.as_str();

            for day in 1..26 {
                match advent::get_input(day, 2015, session) {
                    Ok(input) => adv2015::print_answers(day, input),
                    Err(_) => eprintln!("Couldn't get Input value for day {} / 2015.", day),
                }
            }
        }
        Err(e) => println!("Couldn't get {} key from environment variable. Description: {}", key, e),
    }

    println!("Done");
}
