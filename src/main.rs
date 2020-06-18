use std::env;

fn main() {
    println!("Advent 2015!");
    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            adv2015::print_answers(session_value);
        }
        Err(e) => println!("Couldn't get {} key from environment variable. Description: {}", key, e),
    }

    println!("Done");
}
