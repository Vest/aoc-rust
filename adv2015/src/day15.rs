use std::cmp::max;

pub fn get_answer(input: &str) -> usize {
    let ingredients = parse_lines(input);
    let spoons = find_spoons(&ingredients);

    calc_spoons(&ingredients, &spoons)
}

pub fn get_answer_with_calories(input: &str) -> usize {
    let ingredients = parse_lines(input);
    let spoons = find_spoons_with_calories(&ingredients);

    calc_spoons(&ingredients, &spoons)
}

struct Ingredient {
    #[allow(dead_code)]
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
    let mut result_ingredient = Ingredient {
        name: String::new(),
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
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

fn find_spoons(ingredients: &Vec<Ingredient>) -> Vec<i32> {
    let spoons_count: usize = ingredients.len();
    let max_size: usize = 100usize.pow(spoons_count as u32 - 1) + 1;
    let mut spoons: Vec<i32> = vec![0; spoons_count];
    let mut result = spoons.clone();
    let mut max_result: usize = 0;

    for cur_size in 1..max_size {
        let mut total = 0;

        for i in 0..spoons_count - 1 {
            spoons[i] = (cur_size % 100usize.pow(i as u32 + 1)) as i32;

            if i > 0 {
                spoons[i] -= spoons[i - 1] * 100i32.pow(i as u32 - 1);
                spoons[i] = spoons[i] / 100i32.pow(i as u32);
            }

            total += spoons[i];
        }
        if total > 100 {
            continue;
        }

        spoons[spoons_count - 1] = 100 - total;

        let score = calc_spoons(&ingredients, &spoons);
        if score > max_result {
            result = spoons.clone();
            max_result = score;
        }
    }

    result
}

fn find_spoons_with_calories(ingredients: &Vec<Ingredient>) -> Vec<i32> {
    let spoons_count: usize = ingredients.len();
    const TOTAL_CALORIES: i32 = 500;
    let max_size: usize = 100usize.pow(spoons_count as u32 - 1) + 1;
    let mut spoons: Vec<i32> = vec![0; spoons_count];
    let mut result = spoons.clone();
    let mut max_result: usize = 0;

    for cur_size in 1..max_size {
        let mut total = 0;

        for i in 0..spoons_count - 1 {
            spoons[i] = (cur_size % 100usize.pow(i as u32 + 1)) as i32;

            if i > 0 {
                spoons[i] -= spoons[i - 1] * 100i32.pow(i as u32 - 1);
                spoons[i] = spoons[i] / 100i32.pow(i as u32);
            }

            total += spoons[i];
        }

        if total > 100 {
            continue;
        }

        spoons[spoons_count - 1] = 100 - total;

        let calories: i32 = spoons.iter()
            .enumerate()
            .map(|(i, s)| ingredients[i].calories * s)
            .sum();
        if calories == TOTAL_CALORIES {
            let score = calc_spoons(&ingredients, &spoons);

            if score > max_result {
                result = spoons.clone();
                max_result = score;
            }
        }
    }

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

    #[test]
    fn test_find_spoons() {
        let ingredients = parse_lines("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        let spoons = find_spoons(&ingredients);

        assert_eq!(spoons.len(), 2);
        assert_eq!(spoons[0], 44);
        assert_eq!(spoons[1], 56);
    }

    #[test]
    fn test_get_answer() {
        let result = get_answer("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        assert_eq!(result, 62842880);
    }

    #[test]
    fn test_find_spoons_with_calories() {
        let ingredients = parse_lines("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        let spoons = find_spoons_with_calories(&ingredients);

        assert_eq!(spoons.len(), 2);
        assert_eq!(spoons[0], 40);
        assert_eq!(spoons[1], 60);
    }

    #[test]
    fn test_get_answer_with_calories() {
        let result = get_answer_with_calories("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        assert_eq!(result, 57600000);
    }
}