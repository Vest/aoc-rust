use std::collections::HashSet;

pub fn count_nice_lines(input: &str) -> usize {
    input.lines()
        .filter(|&s| is_nice(s))
        .count()
}

pub fn count_nice_lines_advanced(input: &str) -> usize {
    input.lines()
        .filter(|&s| is_nice_advanced(s))
        .count()
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

        if pair == ('a', 'b') ||
            pair == ('c', 'd') ||
            pair == ('p', 'q') ||
            pair == ('x', 'y') {
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
    let mut pair = (' ', ' ', ' ');  // use last two only.
    let mut three = (' ', ' ', ' ');

    let mut is_three = false;
    let mut is_two = false;
    let mut pairs: HashSet<(char, char)> = HashSet::new();

    for c in s.as_str().chars() {
        if !is_three {
            if three.2 == ' ' {
                three.2 = c;
            } else {
                three = (three.1, three.2, c);

                if three.0 == three.2 {
                    is_three = true;
                }
            }
        }

        if !is_two {
            if pair.2 == ' ' {
                pair.2 = c;
            } else {
                pair = (pair.1, pair.2, c);

                if !pairs.contains(&(pair.1, pair.2))  && pair.0 != pair.1 {
                    pairs.insert((pair.1, pair.2));
                } else {
                    is_two = true;
                }
            }
        }
    }

    return is_three && is_two;
}

fn is_vowel(c: &char) -> bool {
    return match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        ["ugknbfddgicrmopn", "aaa"].iter()
            .for_each(|s| {
                assert!(is_nice(s), "{} is nice, but it wasn't", s);
            });
    }

    #[test]
    fn test_naughty() {
        ["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"].iter()
            .for_each(|s| {
                assert!(!is_nice(s), "{} is naughty, but it wasn't", s);
            });
    }

    #[test]
    fn test_count_nice() {
        let input = ["jchzalrnumimnmhp", "ugknbfddgicrmopn", "haegwjzuvuyypxyu", "aaa", "dvszwmarrgswjxmb"]
            .join("\n");

        assert_eq!(count_nice_lines(input.as_str()), 2, "{} has two nice strings", input);
    }

    #[test]
    fn test_is_nice_advanced() {
        ["qjhvhtzxzqqjkmpb", "xxyxx"].iter()
            .for_each(|s| {
                assert!(is_nice_advanced(s), "{} is nice (advanced), but it wasn't", s);
            });
    }

    #[test]
    fn test_is_naughty_advanced() {
        ["uurcxstgmygtbstg", "ieodomkazucvgmuy"].iter()
            .for_each(|s| {
                assert!(!is_nice_advanced(s), "{} is nice (advanced), but it wasn't", s);
            });
    }
}
