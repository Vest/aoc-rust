mod advent;

use clap::{Arg, Command};
use std::env;

fn main() {
    let matches = Command::new("Advent 2015, 2020!")
        .author("Vest <vest at github.com>")
        .about("Solves advent calendar from https://adventofcode.com")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .required(false)
                .help("A day of the advent"),
        )
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .required(false)
                .default_value("2020")
                .help("A year of the calendar"),
        )
        .get_matches();

    let day_num = matches.get_one::<u8>("day");
    let year_num = matches.get_one::<u16>("year").unwrap();

    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            let session = session_value.as_str();

            for day in 1..26 {
                if let Some(day_num) = day_num {
                    if *day_num != day {
                        continue;
                    }
                }

                let input = advent::get_input(day, *year_num, session);
                if input.is_err() {
                    eprintln!(
                        "Couldn't get input value for day {} / {}. Error: {}",
                        day,
                        year_num,
                        input.unwrap_err()
                    );
                    return;
                }

                print_answers(day, *year_num, input.ok().unwrap());
            }
        }
        Err(e) => println!(
            "Couldn't get {} key from environment variable. Description: {}",
            key, e
        ),
    }

    println!("Done");
}

#[inline]
fn print_answers(day: u8, year_num: u16, input: String) {
    match year_num {
        2015 => adv2015::print_answers(day, input),
        _ => adv2020::print_answers(day, input),
    }
}
