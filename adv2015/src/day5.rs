use std::collections::HashSet;

pub fn count_nice_lines(input: &str) -> usize {
    input.lines().filter(|&s| is_nice(s)).count()
}

pub fn count_nice_lines_advanced(input: &str) -> usize {
    input.lines().filter(|&s| is_nice_advanced(s)).count()
}

fn is_nice(s: &str) -> bool {
    let mut vowels = 0u8;
    let s = s.to_lowercase();
    let mut pair = (' ', ' ');
    let mut has_dups = false;

    for c in s.as_str().chars() {
        if is_vowel(&c) {
            vowels += 1;
        }

        if pair.1 == ' ' {
            pair.1 = c;
            continue;
        }

        pair = (pair.1, c);

        if pair == ('a', 'b') || pair == ('c', 'd') || pair == ('p', 'q') || pair == ('x', 'y') {
            return false;
        }

        if !has_dups && pair.0 == pair.1 {
            has_dups = true;
        }
    }

    return vowels >= 3 && has_dups;
}

fn is_nice_advanced(s: &str) -> bool {
    let s = s.to_lowercase();

    let mut is_three = false;
    let mut is_two = false;

    let mut overlap = (' ', ' ', ' ');
    let mut three = (' ', ' ', ' ');
    let mut pairs: HashSet<(char, char)> = HashSet::new();

    for c in s.as_str().chars() {
        if !is_two {
            // Bugfix: handle xxxx, aaaa, bbbb etc
            if overlap.0 == overlap.1 && overlap.1 == overlap.2 && overlap.2 == c {
                is_two = true;
            }

            overlap = (overlap.1, overlap.2, c);
            let pair = (overlap.1, overlap.2);

            if overlap.0 != overlap.1 || overlap.1 != overlap.2 {
                if pairs.contains(&pair) {
                    is_two = true;
                }

                pairs.insert(pair);
            }
        }

        if !is_three {
            three = (three.1, three.2, c);

            if three.0 == three.2 {
                is_three = true;
            }
        }
    }

    return is_two && is_three;
}

fn is_vowel(c: &char) -> bool {
    return match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        ["ugknbfddgicrmopn", "aaa"].iter().for_each(|s| {
            assert!(is_nice(s), "{} is nice, but it wasn't", s);
        });
    }

    #[test]
    fn test_naughty() {
        ["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"]
            .iter()
            .for_each(|s| {
                assert!(!is_nice(s), "{} is naughty, but it wasn't", s);
            });
    }

    #[test]
    fn test_count_nice() {
        let input = [
            "jchzalrnumimnmhp",
            "ugknbfddgicrmopn",
            "haegwjzuvuyypxyu",
            "aaa",
            "dvszwmarrgswjxmb",
        ]
        .join("\n");

        assert_eq!(
            count_nice_lines(input.as_str()),
            2,
            "{} has two nice strings",
            input
        );
    }

    #[test]
    fn test_count_nice_lines_advanced() {
        let input = ["qjhvhtzxzqqjkmpb", "xxyxx", "dieatyxxxlvhneoj", "xxxx"].join("\n");

        assert_eq!(
            count_nice_lines_advanced(input.as_str()),
            3,
            "{} has three nice strings",
            input
        );
    }

    #[test]
    fn test_is_nice_advanced() {
        ["qjhvhtzxzqqjkmpb", "xxyxx"].iter().for_each(|s| {
            assert!(
                is_nice_advanced(s),
                "{} is nice (advanced), but it wasn't",
                s
            );
        });
    }

    #[test]
    fn test_is_naughty_advanced() {
        ["uurcxstgmygtbstg", "ieodomkazucvgmuy"]
            .iter()
            .for_each(|s| {
                assert!(
                    !is_nice_advanced(s),
                    "{} is nice (advanced), but it wasn't",
                    s
                );
            });
    }

    #[test]
    fn test_is_naughty_advanced_bugfix() {
        assert!(
            !is_nice_advanced("dieatyxxxlvhneoj"),
            "dieatyxxxlvhneoj is nice (advanced), but it wasn't"
        );
    }

    #[test]
    fn test_is_nice_advanced_bugfix() {
        assert!(
            is_nice_advanced("xxxx"),
            "xxxx is nice (advanced), but it wasn't"
        );
        assert!(
            is_nice_advanced("xxaxx"),
            "xxaxx is nice (advanced), but it wasn't"
        );
    }
}
