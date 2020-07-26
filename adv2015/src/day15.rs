use std::cmp::max;

pub fn get_answer(input: &str) -> usize {
    0
}

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_line(input: &str) -> Ingredient {
    let everything: Vec<&str> = input.split(|c: char| c == ':' || c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty()) // remove empty results, we don't need them
        .collect();
    let name = everything[0];
    let cap_str = everything[2];
    let dur_str = everything[4];
    let fla_str = everything[6];
    let tex_str = everything[8];
    let cal_str = everything[10];

    Ingredient {
        name: String::from(name),
        capacity: cap_str.parse::<i32>().unwrap(),
        durability: dur_str.parse::<i32>().unwrap(),
        flavor: fla_str.parse::<i32>().unwrap(),
        texture: tex_str.parse::<i32>().unwrap(),
        calories: cal_str.parse::<i32>().unwrap(),
    }
}

fn parse_lines(input: &str) -> Vec<Ingredient> {
    input.lines()
        .map(|line| parse_line(line))
        .collect()
}

fn calc_spoons(ingredients: &Vec<Ingredient>, spoons: &Vec<i32>) -> usize {
    let mut result = 1usize;
    assert_eq!(ingredients.len(), spoons.len());
    let mut result_ingredient = Ingredient{
        name: String::from("Result"),
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0
    };

    for (index, spoon) in spoons.iter().enumerate() {
        result_ingredient.capacity += ingredients[index].capacity * spoon;
        result_ingredient.durability += ingredients[index].durability * spoon;
        result_ingredient.flavor += ingredients[index].flavor * spoon;
        result_ingredient.texture += ingredients[index].texture * spoon;

    }
    result *= max(0, result_ingredient.capacity) as usize;
    result *= max(0, result_ingredient.durability) as usize;
    result *= max(0, result_ingredient.flavor) as usize;
    result *= max(0, result_ingredient.texture) as usize;

    result
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

    #[test]
    fn test_parse_lines() {
        let result = parse_lines("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_calc_spoons() {
        let ingredients = parse_lines("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        let spoons = vec!(44, 56);
        let result = calc_spoons(&ingredients, &spoons);

        assert_eq!(result, 62842880);
    }
}