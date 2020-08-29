use regex::Regex;
use std::collections::{HashSet, HashMap};

pub fn count_unique_molecules(input: &str) -> usize {
    let (replacements, sample_molecule) = parse_all(input);
    let molecules = build_molecules(sample_molecule, &replacements);

    molecules.len()
}

pub fn count_molecules_from_electron(input: &str) -> usize {
    let (replacements, sample_molecule) = parse_all(input);
    let count = build_molecule_from_e(sample_molecule, &replacements);

    count
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
    let mol_reg = Regex::new("(e)|([A-Z][a-d,f-z]?)").unwrap(); // everything but not "Xe". Xa, Xb

    mol_reg.captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str())
        .collect()
}

fn build_molecules(molecule: &str, replacements: &Vec<Replace>) -> HashSet<String> {
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

fn build_molecule_from_e(molecule: &str, replacements: &Vec<Replace>) -> usize {
    let mut mo = build_molecules("e", replacements);
    let mut iter = 1usize;
    let replacements_map = convert_replacements(replacements);
    let molecule_length = molecule.len();

    let mut nonreplaceable_atoms: HashSet<&str> = HashSet::new();
    split_molecule(molecule).iter()
        .filter(|atom| !replacements_map.contains_key(*atom))
        .for_each(|atom| {
            nonreplaceable_atoms.insert(*atom);
        });

    let potential_to_optimize = !nonreplaceable_atoms.is_empty();
    let mut start_optimization: bool = false;

    while !mo.contains(molecule) {
        let mut new_mo: HashSet<String> = HashSet::new();

        mo.iter()
            .for_each(|m| new_mo.extend(build_molecules(m, replacements)));

        if potential_to_optimize {
            if start_optimization {
                mo.clear();
                for new_item in new_mo.iter()
                    .filter(|m| {
                        let small_atoms = split_molecule(m);
                        small_atoms.iter()
                            .find(|p| nonreplaceable_atoms.contains(*p))
                            .is_some()
                    }).filter(|m| m.len() <= molecule_length) {
                    mo.insert(new_item.clone());
                }
                println!("Optimized from {} to {}", new_mo.len(), mo.len());
            } else {
                new_mo.iter()
                    .for_each(|m| {
                        if !start_optimization {
                            let small_atoms = split_molecule(m);
                            for small_atom in small_atoms {
                                if nonreplaceable_atoms.contains(small_atom) {
                                    start_optimization = true;
                                    println!("Start optimization");
                                    break;
                                }
                            }
                        }
                    });
            }
        } else {
            mo.clear();
            mo.extend(new_mo);
        }


        iter += 1;
    }

    iter
}

fn convert_replacements<'a>(replacements: &'a Vec<Replace>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    replacements.iter()
        .for_each(|replace| {
            if let Some(to) = map.get_mut(replace.from) {
                to.push(replace.to);
            } else {
                let to = vec![replace.to];
                map.insert(replace.from, to);
            }
        });

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"H => HO
            H => OH
            O => HH

            HOH"#;
    const SANTA_SAMPLE: &str = r#"H => HO
            H => OH
            O => HH

            HOHOHO"#;

    const E_SAMPLE: &str = r#"e => H
            e => O
            H => HO
            H => OH
            O => HH

            HOH"#;

    const E_SANTA_SAMPLE: &str = r#"e => H
            e => O
            H => HO
            H => OH
            O => HH

            HOHOHO"#;

    #[test]
    fn test_split_molecule() {
        let atoms = split_molecule("CRnSiRneOe");
        assert_eq!(atoms[0], "C");
        assert_eq!(atoms[1], "Rn");
        assert_eq!(atoms[2], "Si");
        assert_eq!(atoms[3], "Rn");
        assert_eq!(atoms[4], "e");
        assert_eq!(atoms[5], "O");
        assert_eq!(atoms[6], "e");
    }

    #[test]
    fn test_parse_single_replace() {
        let replace_atoms = parse_single_replace("O => HH");
        assert_eq!(replace_atoms.from, "O");
        assert_eq!(replace_atoms.to, "HH");
    }

    #[test]
    fn test_parse_all() {
        let (replacements, molecule) = parse_all(SAMPLE);

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
        let (replacements, sample_molecule) = parse_all(SAMPLE);


        let molecules = build_molecules(sample_molecule, &replacements);

        assert_eq!(molecules.len(), 4);
        assert!(molecules.contains("HOOH"));
        assert!(molecules.contains("HOHO"));
        assert!(molecules.contains("OHOH"));
        assert!(molecules.contains("HHHH"));
    }

    #[test]
    fn test_build_molecules_santa() {
        let (replacements, sample_molecule) = parse_all(SANTA_SAMPLE);

        let molecules = build_molecules(sample_molecule, &replacements);
        println!("{:?}", molecules);
        assert_eq!(molecules.len(), 7);
    }

    #[test]
    fn test_count_unique_molecules() {
        let answer = count_unique_molecules(SANTA_SAMPLE);

        assert_eq!(answer, 7);
    }

    #[test]
    fn test_count_unique_molecules_empty() {
        let answer = count_unique_molecules("");

        assert_eq!(answer, 0);
    }

    #[test]
    fn test_convert_replacements() {
        let (replacements, _) = parse_all(E_SAMPLE);
        let convert = convert_replacements(&replacements);

        assert_eq!(convert.len(), 3);
        assert_eq!(convert.get("e").unwrap().len(), 2);
        assert_eq!(convert.get("H").unwrap().len(), 2);
        assert_eq!(convert.get("O").unwrap().len(), 1);
    }

    #[test]
    fn test_build_molecule_from_e() {
        let (replacements, molecule) = parse_all(E_SAMPLE);
        let number_steps = build_molecule_from_e(molecule, &replacements);
        assert_eq!(number_steps, 3);
    }

    #[test]
    fn test_build_santa_molecule_from_e() {
        let (replacements, molecule) = parse_all(E_SANTA_SAMPLE);
        let number_steps = build_molecule_from_e(molecule, &replacements);
        assert_eq!(number_steps, 6);
    }

    #[test]
    fn test_count_molecules_from_electron() {
        let answer = count_molecules_from_electron(E_SANTA_SAMPLE);
        assert_eq!(answer, 6);
    }
}
