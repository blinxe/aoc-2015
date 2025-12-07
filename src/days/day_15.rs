use itertools::Itertools;
use regex::Regex;

use crate::utils::input::read_input;

#[derive(Debug)]
struct Ingredient {
    name: String,
    properties: [i32; 5],
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| {
            let name = line.split(':').next().unwrap().to_owned();
            let re = Regex::new("-?[0-9]+").unwrap();
            let properties = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect_array()
                .unwrap();
            Ingredient { name, properties }
        })
        .collect()
}

fn get_score(ingredients: &Vec<Ingredient>, spoons: &[i32; 4]) -> i64 {
    (0usize..4) // ignore 5th property, calories
        .map(|p| {
            ingredients
                .iter()
                .enumerate()
                .map(|(i, ing)| (spoons[i] * ing.properties[p]) as i64)
                .sum::<i64>()
        })
        .map(|sum| if sum < 0 { 0 } else { sum })
        .product()
}

fn get_score_v2(ingredients: &Vec<Ingredient>, spoons: &[i32; 4]) -> i64 {
    if ingredients
        .iter()
        .enumerate()
        .map(|(i, ing)| (spoons[i] * ing.properties[4]) as i64)
        .sum::<i64>()
        != 500
    {
        return 0;
    }
    get_score(ingredients, spoons)
}

fn solve_part_1(input: &str) {
    let ingredients = parse_input(input);
    // println!("{:?}", ingredients);

    let mut score_max = 0;
    // spoons of ingredient 0
    for i0 in 0..=100 {
        // spoons of ingredient 1
        for i1 in 0..=100 - i0 {
            // spoons of ingredient 2
            for i2 in 0..=100 - i0 - i1 {
                // spoons of ingredient 3
                let i3 = 100 - i0 - i1 - i2;
                let score = get_score(&ingredients, &[i0, i1, i2, i3]);
                // println!("{:?}: {}", [i0, i1, i2, i3], score);
                if score > score_max {
                    score_max = score;
                }
            }
        }
    }

    println!("Max score: {}", score_max);
}

fn solve_part_2(input: &str) {
    let ingredients = parse_input(input);
    // println!("{:?}", ingredients);

    let mut score_max = 0;
    // spoons of ingredient 0
    for i0 in 0..=100 {
        // spoons of ingredient 1
        for i1 in 0..=100 - i0 {
            // spoons of ingredient 2
            for i2 in 0..=100 - i0 - i1 {
                // spoons of ingredient 3
                let i3 = 100 - i0 - i1 - i2;
                let score = get_score_v2(&ingredients, &[i0, i1, i2, i3]);
                // println!("{:?}: {}", [i0, i1, i2, i3], score);
                if score > score_max {
                    score_max = score;
                }
            }
        }
    }

    println!("Max score: {}", score_max);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
