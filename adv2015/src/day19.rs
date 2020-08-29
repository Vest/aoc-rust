use regex::Regex;

pub fn get_answer(input: &str) -> usize {
    0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_molecule() {
        let molecules = split_molecule("CRnSiRnO");
        assert_eq!(molecules[0], "C");
        assert_eq!(molecules[1], "Rn");
        assert_eq!(molecules[2], "Si");
        assert_eq!(molecules[3], "Rn");
        assert_eq!(molecules[4], "O");
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
}
