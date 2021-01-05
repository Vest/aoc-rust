mod advent;

use std::env;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent 2015, 2020!")
        .author("Vest <vest at github.com>")
        .about("Solves advent calendar from https://adventofcode.com")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .takes_value(true)
            .required(false)
            .help("A day of the advent"))
        .arg(Arg::with_name("year")
            .short("y")
            .long("year")
            .takes_value(true)
            .required(false)
            .help("A year of the calendar"))
        .get_matches();

    let day_str = matches.value_of("day");
    let year_str = matches.value_of("year").unwrap_or("2020");
    let year_num = year_str.parse::<u16>().unwrap_or(2020);

    let key = "ADVENT_SESSION";
    match env::var(key) {
        Ok(session_value) => {
            let session = session_value.as_str();

            for day_num in 1..26 {
                if day_str.is_some() {
                    let day_parsed = day_str.unwrap().parse::<u8>();
                    if day_parsed.is_ok() && day_parsed.unwrap() != day_num {
                        continue;
                    }
                }

                let input = advent::get_input(day_num, year_num, session);
                if input.is_err() {
                    eprintln!("Couldn't get input value for day {} / {}. Error: {}", day_num, year_num, input.unwrap_err());
                    return;
                }

                print_answers(day_num, year_num, input.ok().unwrap());
            }
        }
        Err(e) => println!("Couldn't get {} key from environment variable. Description: {}", key, e),
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
