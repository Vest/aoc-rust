use std::collections::HashSet;
use combination::*;

fn parse_packages(input: &str) -> Vec<usize> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|p| p.parse::<usize>().ok())
        .collect()
}

fn create_groups(packages: &Vec<usize>, size: usize) -> Vec<Vec<usize>> {
    if !(1..=packages.len()).contains(&size) {
        return Vec::new();
    }

    combine::combine_vec(packages, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packages() {
        let packages = parse_packages(r#"1
        2

        6"#);
        assert_eq!(packages.len(), 3);
        assert_eq!(packages[0], 1);
        assert_eq!(packages[1], 2);
        assert_eq!(packages[2], 6);
    }

    #[test]
    fn test_create_groups() {
        let available_packages: Vec<usize> = vec![1, 2, 4, 5];
        let group_1 = create_groups(&available_packages, 2);

        assert_eq!(group_1.len(), 4 * 3 * 2 / (2 * 2));
    }
}
