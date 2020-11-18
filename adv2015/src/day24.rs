use combinations::Combinations;

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

    let copy_packages = packages.to_vec();

    Combinations::new(copy_packages, size).collect()
}

fn find_groups_with_weight(packages: &Vec<usize>, weight: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    if packages.is_empty() {
        return result;
    }

    for size in 1..packages.len() {
        create_groups(&packages, size).iter()
            .filter(|group| group.iter().sum::<usize>() == weight)
            .for_each(|group| {
                let copy_group = group.to_vec();
                result.push(copy_group);
            });
    }

    result
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
        assert!(create_groups(&available_packages, 0).is_empty());
        assert!(create_groups(&available_packages, 5).is_empty());
    }

    #[test]
    fn test_find_groups_with_weight() {
        let packages: Vec<usize> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let found_packages = find_groups_with_weight(&packages, packages.iter().sum::<usize>() / 3);

        assert_eq!(found_packages.len(), 25); // taken from demo (indeed in the demo there are 13 combinations)
        println!("{:?}", found_packages);
    }
}
