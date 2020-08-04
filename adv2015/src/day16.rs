use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

const IDEAL_AUNT: &str = r#"Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1"#;

pub fn get_answer(input: &str) -> usize {
    let aunts = parse_aunts(input);
    let ideal_aunt = Aunt::from_str(IDEAL_AUNT).unwrap();

    find_ideal_aunt(&aunts, &ideal_aunt)
        .unwrap_or(0)
}

#[derive(Debug)]
struct ParseThingsError(String);

impl fmt::Display for ParseThingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ParseThingsError: couldn't parse '{}' to Things", self.0))
    }
}

#[derive(Debug)]
struct ParseAuntError(String);

impl fmt::Display for ParseAuntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ParseAuntError: couldn't parse '{}' to Aunt", self.0))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Things {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FromStr for Things {
    type Err = ParseThingsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(Things::Children),
            "cats" => Ok(Things::Cats),
            "samoyeds" => Ok(Things::Samoyeds),
            "pomeranians" => Ok(Things::Pomeranians),
            "akitas" => Ok(Things::Akitas),
            "vizslas" => Ok(Things::Vizslas),
            "goldfish" => Ok(Things::Goldfish),
            "trees" => Ok(Things::Trees),
            "cars" => Ok(Things::Cars),
            "perfumes" => Ok(Things::Perfumes),
            _ => Err(ParseThingsError(String::from(s)))
        }
    }
}

struct Aunt {
    number: usize,
    things: HashMap<Things, usize>,
}

impl Aunt {
    fn get_thing(&self, thing: &Things) -> Option<usize> {
        self.things.get(thing)
            .copied()
    }
}

impl fmt::Debug for Aunt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Sue {}: # of things {}", self.number, self.things.len()))
    }
}

impl FromStr for Aunt {
    type Err = ParseAuntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s
            .split(|c: char| c.is_whitespace() || c == ':' || c == ',')
            .filter(|token| *token != "Sue" && !token.is_empty())
            .into_iter();

        if let Some(num_str) = tokens.next() {
            if let Ok(number) = num_str.parse::<usize>() {
                let mut aunt = Aunt {
                    number,
                    things: HashMap::new(),
                };

                let mut parse_aunt = || -> Result<(), ParseThingsError> {
                    while let Some(thing_str) = tokens.next() {
                        let thing = Things::from_str(thing_str)?;

                        if let Some(count_str) = tokens.next() {
                            if let Ok(count) = count_str.parse::<usize>() {
                                aunt.things.insert(thing, count);
                            } else {
                                let mut err_str = String::from(thing_str);
                                err_str.push_str(": ");
                                err_str.push_str(thing_str);

                                return Err(ParseThingsError(err_str));
                            }
                        }
                    }
                    Ok(())
                };

                if let Err(e) = parse_aunt() {
                    eprintln!("{}", e);
                }

                return Ok(aunt);
            }
        }

        Err(ParseAuntError(String::from(s)))
    }
}

fn parse_aunts(input: &str) -> Vec<Aunt> {
    input.lines()
        .map(|l| Aunt::from_str(l))
        .filter(|a| a.is_ok())
        .map(|a| a.unwrap())
        .collect()
}

fn find_ideal_aunt(aunts: &Vec<Aunt>, aunt: &Aunt) -> Option<usize> {
    let opt_aunt = aunts.iter()
        .find(|&sample| {
            aunt.things.iter()
                .filter(|aunt_thing| {
                    if let Some(sample_value) = sample.get_thing(aunt_thing.0) {
                        *aunt_thing.1 == sample_value
                    } else {
                        false
                    }
                })
                .count() == sample.things.len()
        });

    if let Some(aunt) = opt_aunt {
        Some(aunt.number)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aunt_from_str() {
        let aunt = Aunt::from_str("Sue 30: vizslas: 3, perfumes: 8, akitas: 2")
            .unwrap();
        assert_eq!(aunt.number, 30);
        assert_eq!(aunt.get_thing(&Things::Vizslas), Some(3usize));
        assert_eq!(aunt.get_thing(&Things::Perfumes), Some(8usize));
        assert_eq!(aunt.get_thing(&Things::Akitas), Some(2usize));

        assert_eq!(format!("{:?}", aunt), "Sue 30: # of things 3");
    }

    #[test]
    fn test_aunt_from_str_error_token() {
        let aunt = Aunt::from_str("Sue 32: perfumes: 8, children: -3, akitas: 2")
            .unwrap();
        assert_eq!(aunt.number, 32);
        assert_eq!(aunt.things.len(), 1);
        assert_eq!(aunt.get_thing(&Things::Children), None);
    }

    #[test]
    fn test_aunt_from_str_error_aunt() {
        let err = Aunt::from_str("Marry 32: children: 13, perfumes: 8, akitas: 2");
        let aunt_error = err.unwrap_err();
        assert_eq!(format!("{:?}", aunt_error), r#"ParseAuntError("Marry 32: children: 13, perfumes: 8, akitas: 2")"#);
        assert_eq!(format!("{}", aunt_error), r#"ParseAuntError: couldn't parse 'Marry 32: children: 13, perfumes: 8, akitas: 2' to Aunt"#);
    }

    #[test]
    fn test_things_from_str() {
        assert_eq!(Things::from_str("children").unwrap(), Things::Children);
        assert_eq!(Things::from_str("cats").unwrap(), Things::Cats);
        assert_eq!(Things::from_str("samoyeds").unwrap(), Things::Samoyeds);
        assert_eq!(Things::from_str("pomeranians").unwrap(), Things::Pomeranians);
        assert_eq!(Things::from_str("akitas").unwrap(), Things::Akitas);
        assert_eq!(Things::from_str("vizslas").unwrap(), Things::Vizslas);
        assert_eq!(Things::from_str("goldfish").unwrap(), Things::Goldfish);
        assert_eq!(Things::from_str("trees").unwrap(), Things::Trees);
        assert_eq!(Things::from_str("cars").unwrap(), Things::Cars);
        assert_eq!(Things::from_str("perfumes").unwrap(), Things::Perfumes);

        let things_result = Things::from_str("Vest");
        assert!(things_result.is_err());
        let things_error = things_result.unwrap_err();
        assert_eq!(format!("{:?}", things_error), r#"ParseThingsError("Vest")"#);
        assert_eq!(format!("{}", things_error), r#"ParseThingsError: couldn't parse 'Vest' to Things"#);
    }

    #[test]
    fn test_parse_aunts() {
        let aunts = parse_aunts(r#"Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: akitas: 9, children: 3, samoyeds: 9
        Sue 3: trees: 6, cars: 6, children: 4
        Sue 4: trees: 4, vizslas: 4, goldfish: 9
        Vest
        Sue 5: akitas: 9, vizslas: 7, cars: 5
        Sue 6: vizslas: 6, goldfish: 6, akitas: 3"#);

        assert_eq!(aunts.len(), 6);
    }

    #[test]
    fn test_parse_ideal_aunt() {
        let aunt = Aunt::from_str(IDEAL_AUNT);
        assert!(aunt.is_ok());
    }

    #[test]
    fn test_find_ideal_aunt() {
        let aunts = parse_aunts(r#"Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: children: 3, cats: 7, pomeranians: 3"#);
        let ideal_aunt = Aunt::from_str(IDEAL_AUNT).unwrap();
        let aunt = find_ideal_aunt(&aunts, &ideal_aunt);
        assert!(aunt.is_some());
        assert_eq!(aunt.unwrap(), 2);
    }

    #[test]
    fn test_cannot_find_ideal_aunt() {
        let aunts = parse_aunts(r#"Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: children: 4, cats: 7, pomeranians: 3"#); // this aunt doesn't exist
        let ideal_aunt = Aunt::from_str(IDEAL_AUNT).unwrap();
        let aunt = find_ideal_aunt(&aunts, &ideal_aunt);
        assert!(aunt.is_none());
    }

    #[test]
    fn test_get_answer() {
        let answer = get_answer(r#"Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: children: 3, cats: 7, pomeranians: 3"#);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_get_empty_answer() {
        let answer = get_answer("");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_cannot_get_answer() {
        let answer = get_answer(r#"Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: children: 4, cats: 7, pomeranians: 3"#); // this aunt doesn't exist
        assert_eq!(answer, 0);
    }
}
