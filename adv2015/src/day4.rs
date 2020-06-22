pub fn mine_suffix(secret: &str, zeroes: u8) -> u32 {
    let mut answer = 1;
    let secret = secret.trim();

    let mut pattern = String::with_capacity(zeroes as usize);
    for _ in 0..zeroes {
        pattern.push('0');
    }

    while !is_adventcoin(
        calc_md5(secret, answer).as_str(),
        pattern.as_str(),
    ) {
        answer += 1;
    }

    answer
}

fn calc_md5(secret: &str, suffix: u32) -> String {
    let input = format!("{}{}", secret, suffix);
    let digest = md5::compute(input);

    format!("{:x}", digest)
}

#[inline]
fn is_adventcoin(hash: &str, pattern: &str) -> bool {
    hash.starts_with(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_md5() {
        assert_eq!(calc_md5("abcdef", 609043), "000001dbbfa3a5c83a2d506429c7b00e")
    }

    #[test]
    fn test_is_adventcoin() {
        assert!(is_adventcoin("000001dbbfa3a5c83a2d506429c7b00e", "00000"), "doesn't have five zeroes");
    }

    #[test]
    #[ignore]
    fn test_mine_suffix_long() {
        assert_eq!(mine_suffix("abcdef", 5), 609043, "Didn't mine suffix properly");
        assert_eq!(mine_suffix("pqrstuv", 5), 1048970, "Didn't mine suffix properly");
    }

    #[test]
    fn test_mine_suffix_quick() {
        let abc = mine_suffix("abcdef", 1);
        assert_eq!(abc, 31, "Didn't mine suffix properly");

        let hash = calc_md5("abcdef", abc);
        assert!(hash.starts_with('0'), "Hash doesn't start with 0: {}", hash);
    }
}
