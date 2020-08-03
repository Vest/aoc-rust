use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

pub fn get_answer(_input: &str) -> usize {
    0
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
        assert_eq!(format!("{:?}",things_error ), r#"ParseThingsError("Vest")"#);
        assert_eq!(format!("{}", things_error), r#"ParseThingsError: couldn't parse 'Vest' to Things"#);
    }
}
