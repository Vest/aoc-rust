use regex::Regex;

pub fn answer1(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter()
        .filter(|pass| is_valid(pass))
        .count()
}

pub fn answer2(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter()
        .filter(|pass| is_valid(pass) && is_valid_advanced(pass))
        .count()
}

fn parse_input(input: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut tmp = String::new();

    for line in input.lines() {
        if line.is_empty() {
            result.push(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(' ');
            tmp.push_str(&line);
        }
    }

    if !tmp.is_empty() {
        result.push(tmp.clone());
        tmp.clear();
    }

    result
}

fn is_valid(pass: &String) -> bool {
    pass.contains("byr:") && pass.contains("iyr:")
        && pass.contains("eyr:") && pass.contains("hgt:")
        && pass.contains("hcl:") && pass.contains("ecl:")
        && pass.contains("pid:")
}

fn is_valid_advanced(pass: &String) -> bool {
    let parts: Vec<&str> = pass.split_whitespace().collect();
    let re = Regex::new(r"#([a-fA-F0-9]{6})").unwrap();

    for part in parts {
        let pair: Vec<&str> = part.split(|c: char| c == ':').collect();

        if pair[0].contains("byr") {
            let num = pair[1].parse::<usize>().unwrap();

            if num < 1920 || num > 2002 {
                return false;
            }
        }
        if pair[0].contains("iyr") {
            let num = pair[1].parse::<usize>().unwrap();

            if num < 2010 || num > 2020 {
                return false;
            }
        }
        if pair[0].contains("eyr") {
            let num = pair[1].parse::<usize>().unwrap();

            if num < 2020 || num > 2030 {
                return false;
            }
        }
        if pair[0].contains("hgt") {
            let num = pair[1][..pair[1].len() - 2].parse::<usize>();

            if num.is_err() {
                return false;
            }

            let num = num.unwrap();

            if pair[1].ends_with("cm") && (num < 150 || num > 193) {
                return false;
            } else if pair[1].ends_with("in") && (num < 59 || num > 76) {
                return false;
            } else if !pair[1].ends_with("cm") && !pair[1].ends_with("in") {
                return false;
            }
        }
        if pair[0].contains("hcl") {
            if !re.is_match(pair[1]) {
                return false;
            }
        }
        if pair[0].contains("ecl") {
            match pair[1] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => {
                    return false;
                }
            }
        }
        if pair[0].contains("pid") {
            let num = pair[1].parse::<usize>();

            if num.is_err() || pair[1].len() != 9 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        assert_eq!(answer1(""), 0);


        assert_eq!(answer2(r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#), 4);
    }
}
