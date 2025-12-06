use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input::read_input;

type Map = HashMap<(String, String), u32>;

fn parse_input(input: &str) -> (Vec<String>, Map) {
    let mut cities = HashSet::new();
    let map = input
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            let from = words.next().unwrap().to_string();
            let to = words.nth(1).unwrap().to_string();
            let distance = words.nth(1).unwrap().parse().unwrap();
            cities.insert(from.clone());
            cities.insert(to.clone());
            let mut cities = [from, to];
            cities.sort();
            let [from, to] = cities;
            ((from, to), distance)
        })
        .collect();

    (cities.into_iter().sorted().collect(), map)
}

fn traveling_salesman(cities: &Vec<String>, map: &Map) -> u32 {
    cities
        .iter()
        .permutations(cities.len())
        // .inspect(|c| println!("{:?}", c))
        .map(|order| {
            order
                .iter()
                .zip(order.iter().skip(1))
                .map(|(&c1, &c2)| {
                    if c1 < c2 {
                        map.get(&(c1.clone(), c2.clone())).unwrap()
                    } else {
                        map.get(&(c2.clone(), c1.clone())).unwrap()
                    }
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn traveling_tourist(cities: &Vec<String>, map: &Map) -> u32 {
    cities
        .iter()
        .permutations(cities.len())
        // .inspect(|c| println!("{:?}", c))
        .map(|order| {
            order
                .iter()
                .zip(order.iter().skip(1))
                .map(|(&c1, &c2)| {
                    if c1 < c2 {
                        map.get(&(c1.clone(), c2.clone())).unwrap()
                    } else {
                        map.get(&(c2.clone(), c1.clone())).unwrap()
                    }
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn solve_part_1(input: &str) {
    let (cities, map) = parse_input(input);
    // println!("{:?}", cities);
    // println!("{:?}", map);
    let distance = traveling_salesman(&cities, &map);
    println!("Shortest distance: {}", distance);
}

fn solve_part_2(input: &str) {
    let (cities, map) = parse_input(input);
    let distance = traveling_tourist(&cities, &map);
    println!("Longest distance: {}", distance);
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
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
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
