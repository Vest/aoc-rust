pub fn calc_packs(s: &str) -> u32 {
    s.lines().map(|line| {
        let (l, w, h) = parse_line(line);
        calc_paper(l, w, h)
    }).sum()
}

pub fn calc_ribbons(s: &str) -> u32 {
    s.lines()
        .map(|line| {
            let (l, w, h) = parse_line(line);
            calc_ribbon(l, w, h)
        })
        .sum()
}

fn parse_line(s: &str) -> (u8, u8, u8) {
    let mut result: Vec<u8> = Vec::new();
    let tokens = s.split('x');
    for token in tokens {
        if let Ok(dim) = token.parse::<u8>() {
            result.push(dim);
        }
    }

    if result.len() != 3 {
        return (0, 0, 0);
    }

    (result[0], result[1], result[2])
}

fn calc_paper(l: u8, w: u8, h: u8) -> u32 {
    let dim = (l as u32, w as u32, h as u32);
    let mut sorted_dim = [l as u32, w as u32, h as u32];
    sorted_dim.sort();

    2 * (dim.0 * dim.1 + dim.1 * dim.2 + dim.0 * dim.2) + sorted_dim[0] * sorted_dim[1]
}

fn calc_ribbon(l: u8, w: u8, h: u8) -> u32 {
    let dim = (l as u32, w as u32, h as u32);
    let mut sorted_dim = [l as u32, w as u32, h as u32];
    sorted_dim.sort();

    dim.0 * dim.1 * dim.2 + 2 * (sorted_dim[0] + sorted_dim[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_paper_random() {
        assert_eq!(calc_paper(2, 3, 4), 58);
        assert_eq!(calc_paper(4, 3, 2), 58);
        assert_eq!(calc_paper(2, 4, 3), 58);
    }

    #[test]
    fn test_calc_paper_simple() {
        assert_eq!(calc_paper(2, 3, 4), 58);
        assert_eq!(calc_paper(1, 1, 10), 43);
    }

    #[test]
    fn test_calc_paper_max() {
        assert_eq!(calc_paper(255, 255, 255), 455175);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1x1x1"), (1, 1, 1));
        assert_eq!(parse_line("1x2x3"), (1, 2, 3));
        assert_eq!(parse_line("1x2x3x4"), (0, 0, 0));
        assert_eq!(parse_line("1x2"), (0, 0, 0));
    }

    #[test]
    fn test_calc_packs() {
        assert_eq!(calc_packs("2x3x4\n1x1x10"), 58 + 43);
    }

    #[test]
    fn test_calc_ribbon() {
        assert_eq!(calc_ribbon(2, 3, 4), 34);
        assert_eq!(calc_ribbon(1, 1, 10), 14);
    }

    #[test]
    fn test_calc_ribbons() {
        assert_eq!(calc_ribbons("2x3x4\n1x1x10"), 34 + 14);
    }
}
