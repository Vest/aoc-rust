pub fn get_answer(input: &str) -> usize {
    0
}

struct Ingredient {
    name: String,
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: u8,
}

fn parse_line(input: &str) -> Ingredient {
    let everything: Vec<&str> = input.split(|c: char| c == ':' || c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    let name = everything.get(0).unwrap();
    let cap_str = everything.get(2).unwrap();
    let dur_str = everything.get(4).unwrap();
    let fla_str = everything.get(6).unwrap();
    let tex_str = everything.get(8).unwrap();
    let cal_str = everything.get(10).unwrap();

    Ingredient {
        name: String::from(*name),
        capacity: (*cap_str).parse::<i8>().unwrap(),
        durability: (*dur_str).parse::<i8>().unwrap(),
        flavor: (*fla_str).parse::<i8>().unwrap(),
        texture: (*tex_str).parse::<i8>().unwrap(),
        calories: (*cal_str).parse::<u8>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = parse_line("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8");

        assert_eq!(result.name, String::from("Butterscotch"));
        assert_eq!(result.capacity, -1);
        assert_eq!(result.durability, -2);
        assert_eq!(result.flavor, 6);
        assert_eq!(result.texture, 3);
        assert_eq!(result.calories, 8);
    }
}