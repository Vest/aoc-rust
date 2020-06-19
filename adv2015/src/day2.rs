fn calc_paper(l: u8, w: u8, h: u8) -> u32 {
    let dim = (l as u32, w as u32, h as u32);
    let mut sorted_dim = [l as u32, w as u32, h as u32];
    sorted_dim.sort();

    2 * (dim.0 * dim.1 + dim.1 * dim.2 + dim.0 * dim.2) + sorted_dim[0] * sorted_dim[1]
}

#[cfg(test)]
mod tests {
    use crate::day2::{calc_paper};

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
}
