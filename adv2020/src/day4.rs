use itertools::Itertools;

pub fn count_simple_results(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter()
        .filter(|pass| has_all_data(pass))
        .count()
}

pub fn count_advanced_results(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter()
        .filter(|pass| has_all_data(pass) && is_valid_advanced(pass))
        .count()
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines()
        .group_by(|elt| elt.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)|
            group.collect_vec()
                .join(" ")
                .split_whitespace()
                .filter(|split| !split.is_empty())
                .join(" ")
        ).collect()
}

fn has_all_data(pass: &str) -> bool {
    pass.split_whitespace()
        .filter(|pair| pair.starts_with("byr:")
            || pair.starts_with("iyr:")
            || pair.starts_with("eyr:")
            || pair.starts_with("hgt:")
            || pair.starts_with("hcl:")
            || pair.starts_with("ecl:")
            || pair.starts_with("pid:"))
        .count() >= 7
}

fn is_valid_advanced(pass: &str) -> bool {
    pass.split_whitespace()
        .filter(|part| {
            let pair: Vec<&str> = part.split(|c: char| c == ':').collect();

            if pair.len() != 2 {
                return false;
            }

            let [field, value_str] = [pair[0], pair[1]];
            let value = value_str.parse::<usize>();

            match field {
                "byr" if value.is_ok() => (1920..=2002usize).contains(&value.unwrap()),

                "iyr" if value.is_ok() => (2010..=2020usize).contains(&value.unwrap()),

                "eyr" if value.is_ok() => (2020..=2030usize).contains(&value.unwrap()),

                "hgt" if value.is_err() && is_height(value_str) => true,

                "hcl" if is_hex_color(value_str) => true,

                "ecl" if is_eye_color(value_str) => true,

                "pid" if is_pid(value_str) => true,

                _ => false,
            }
        }).count() >= 7
}

fn is_hex_color(color: &str) -> bool {
    color.chars()
        .enumerate()
        .filter(|c: &(usize, char)| {
            match c.0 {
                0 => c.1 == '#',
                1..=6 => "0123456789abcdefABCDEF".contains(c.1),
                _ => false,
            }
        }).count() == 7
}

fn is_eye_color(color: &str) -> bool {
    match color {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_pid(input: &str) -> bool {
    let value = input.parse::<usize>();

    value.is_ok() && input.len() == 9
}

fn is_height(input: &str) -> bool {
    input.len() > 2
        && (input.ends_with("cm") && (150..=193).contains(&input[..input.len() - 2].parse::<i32>().unwrap_or(0))
        || (input.ends_with("in") && (59..=76).contains(&input[..input.len() - 2].parse::<i32>().unwrap_or(0))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        assert_eq!(count_simple_results(r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                                           byr:1937 iyr:2017 cid:147 hgt:183cm

                                           iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                                           hcl:#cfa07d byr:1929

                                           hcl:#ae17e1 iyr:2013
                                           eyr:2024
                                           ecl:brn pid:760753108 byr:1931
                                           hgt:179cm

                                           hcl:#cfa07d eyr:2025 pid:166559648
                                           iyr:2011 ecl:brn hgt:59in"#), 2);

        assert_eq!(count_advanced_results(r#"eyr:1972 cid:100
                                             hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                                             iyr:2019
                                             hcl:#602927 eyr:1967 hgt:170cm
                                             ecl:grn pid:012533040 byr:1946

                                             hcl:dab227 iyr:2012
                                             ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                                             hgt:59cm ecl:zzz
                                             eyr:2038 hcl:74454a iyr:2023:iyr:2023
                                             pid:3556412378 byr:2007"#), 0);

        assert_eq!(count_advanced_results(r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                                             hcl:#623a2f

                                             eyr:2029 ecl:blu cid:129 byr:1989
                                             iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                                             hcl:#888785
                                             hgt:164cm byr:2001 iyr:2015 cid:88
                                             pid:545766238 ecl:hzl
                                             eyr:2022

                                             iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#), 4);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input("abc\ndef\n\n  hij   klm");
        assert_eq!(result[0], String::from("abc def"));
        assert_eq!(result[1], String::from("hij klm"));
    }

    #[test]
    fn test_is_hex_color() {
        assert!(is_hex_color("#123abc"));
        assert!(is_hex_color("#def890"));
        assert!(!is_hex_color("def890"));
        assert!(!is_hex_color("def890#"));
        assert!(!is_hex_color("#defgab"));
    }

    #[test]
    fn test_has_all_data() {
        assert!(has_all_data("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"));
        assert!(!has_all_data("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"));
        assert!(has_all_data("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"));
        assert!(!has_all_data("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"));
    }

    #[test]
    fn test_is_valid_advanced() {
        assert!(is_valid_advanced("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"));
        assert!(!is_valid_advanced("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"));
        assert!(is_valid_advanced("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"));
        assert!(!is_valid_advanced("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"));
    }

    #[test]
    fn test_is_pid() {
        assert!(is_pid("087499704"));
        assert!(is_pid("896056539"));
        assert!(is_pid("545766238"));
        assert!(is_pid("093154719"));

        assert!(!is_pid("186cm"));
        assert!(!is_pid("3556412378"));
    }

    #[test]
    fn test_is_height() {
        assert!(is_height("60in"));
        assert!(is_height("190cm"));
        assert!(!is_height("190in"));
        assert!(!is_height("190"));
        assert!(!is_height("5cm"));
    }
}
