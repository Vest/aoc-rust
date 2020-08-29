use regex::Regex;
use std::collections::HashSet;

pub fn count_unique_molecules(input: &str) -> usize {
    let (replacements, sample_molecule) = parse_all(input);
    let molecules = build_molecules(sample_molecule, replacements);

    molecules.len()
}

struct Replace<'a> {
    from: &'a str,
    to: &'a str,
}

fn parse_single_replace(line: &str) -> Replace {
    let atoms: Vec<&str> = line.trim()
        .split(" => ")
        .collect();
    Replace {
        from: atoms[0],
        to: atoms[1],
    }
}

fn parse_all(input: &str) -> (Vec<Replace>, &str) {
    let mut result = Vec::with_capacity(5);
    let mut is_molecule = false;
    let mut molecule: &str = "";

    input.lines()
        .into_iter()
        .map(|line| line.trim())
        .for_each(|line| {
            if line.is_empty() {
                is_molecule = true;
            }

            if !is_molecule {
                result.push(parse_single_replace(line));
            } else {
                molecule = line;
            }
        });

    (result, molecule)
}

fn split_molecule(input: &str) -> Vec<&str> {
    let mol_reg = Regex::new("([A-Z][a-z]?)").unwrap();

    mol_reg.captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str())
        .collect()
}

fn build_molecules(molecule: &str, replacements: Vec<Replace>) -> HashSet<String> {
    let mut molecules = HashSet::with_capacity(replacements.capacity());
    let atoms = split_molecule(molecule);

    replacements.iter()
        .for_each(|replace| {
            for top in 0..atoms.len() {
                let mut molecule: Vec<&str> = Vec::with_capacity(atoms.len());
                if atoms[top] == replace.from {
                    if top > 0 {
                        &atoms[0..top].iter()
                            .for_each(|a| molecule.push(a));
                    }
                    molecule.push(replace.to);
                    if top < atoms.len() - 1 {
                        &atoms[top + 1..].iter()
                            .for_each(|a| molecule.push(a));
                    }

                    let result_molecule = String::from(molecule.concat());
                    molecules.insert(result_molecule);

                    continue;
                }
            }
        });

    molecules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_molecule() {
        let atoms = split_molecule("CRnSiRnO");
        assert_eq!(atoms[0], "C");
        assert_eq!(atoms[1], "Rn");
        assert_eq!(atoms[2], "Si");
        assert_eq!(atoms[3], "Rn");
        assert_eq!(atoms[4], "O");
    }

    #[test]
    fn test_parse_single_replace() {
        let replace_atoms = parse_single_replace("O => HH");
        assert_eq!(replace_atoms.from, "O");
        assert_eq!(replace_atoms.to, "HH");
    }

    #[test]
    fn test_parse_all() {
        let (replacements, molecule) = parse_all(
            r#"H => HO
            H => OH
            O => HH

            HOH"#);

        assert_eq!(replacements.len(), 3);

        assert_eq!(replacements[0].from, "H");
        assert_eq!(replacements[0].to, "HO");

        assert_eq!(replacements[1].from, "H");
        assert_eq!(replacements[1].to, "OH");

        assert_eq!(replacements[2].from, "O");
        assert_eq!(replacements[2].to, "HH");

        assert_eq!(molecule, "HOH");
    }

    #[test]
    fn test_build_molecules() {
        let (replacements, sample_molecule) = parse_all(
            r#"H => HO
            H => OH
            O => HH

            HOH"#);


        let molecules = build_molecules(sample_molecule, replacements);

        assert_eq!(molecules.len(), 4);
        assert!(molecules.contains("HOOH"));
        assert!(molecules.contains("HOHO"));
        assert!(molecules.contains("OHOH"));
        assert!(molecules.contains("HHHH"));
    }

    #[test]
    fn test_build_molecules_santa() {
        let (replacements, sample_molecule) = parse_all(
            r#"H => HO
            H => OH
            O => HH

            HOHOHO"#);

        let molecules = build_molecules(sample_molecule, replacements);
        println!("{:?}", molecules);
        assert_eq!(molecules.len(), 7);
    }

    #[test]
    fn test_count_unique_molecules() {
        let answer = count_unique_molecules(r#"H => HO
            H => OH
            O => HH

            HOHOHO"#);

        assert_eq!(answer, 7);
    }

    #[test]
    fn test_count_unique_molecules_empty() {
        let answer = count_unique_molecules("");

        assert_eq!(answer, 0);
    }
}
