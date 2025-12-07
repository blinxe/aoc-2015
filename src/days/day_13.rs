use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input::read_input;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Relationship {
    who: String,
    whom: String,
}

type Happiness = HashMap<Relationship, i32>;

fn parse_input(input: &str) -> (Vec<String>, Happiness) {
    let mut set = HashSet::new();
    let hap = input
        .lines()
        .map(|line| {
            let mut words = line[..line.len() - 1].split(' ');
            let who = words.next().unwrap().to_owned();
            set.insert(who.clone());
            let polarity = if words.nth(1).unwrap() == "gain" {
                1
            } else {
                -1
            };
            let score = words.next().unwrap().parse::<i32>().unwrap() * polarity;
            let whom = words.last().unwrap().to_owned();
            (Relationship { who, whom }, score)
        })
        .collect();

    (set.into_iter().sorted().collect(), hap)
}

fn solve_part_1(input: &str) {
    let (people, hap) = parse_input(input);
    // println!("{:?}", hap);

    let opti = people
        .iter()
        .permutations(people.len())
        // .inspect(|order| print!("{:?}: ", order))
        .map(|order| {
            order
                .iter()
                .zip(order.iter().cycle().skip(1).take(people.len()))
                // .inspect(|(&p1, &p2)| println!("{},{}", p1, p2))
                .map(|(&p1, &p2)| {
                    hap.get(&Relationship {
                        who: p1.clone(),
                        whom: p2.clone(),
                    })
                    .unwrap()
                        + hap
                            .get(&Relationship {
                                who: p2.clone(),
                                whom: p1.clone(),
                            })
                            .unwrap()
                })
                .sum::<i32>()
        })
        // .inspect(|&score| println!("{:?}", score))
        .max()
        .unwrap();

    println!("Optimum configuration: {}", opti);
}

fn solve_part_2(input: &str) {
    let (mut people, mut hap) = parse_input(input);
    for p in people.iter() {
        hap.insert(
            Relationship {
                who: "Self".to_owned(),
                whom: p.clone(),
            },
            0,
        );
        hap.insert(
            Relationship {
                who: p.clone(),
                whom: "Self".to_owned(),
            },
            0,
        );
    }
    people.push("Self".to_owned());
    // println!("{:?}", hap);

    let opti = people
        .iter()
        .permutations(people.len())
        // .inspect(|order| print!("{:?}: ", order))
        .map(|order| {
            order
                .iter()
                .zip(order.iter().cycle().skip(1).take(people.len()))
                // .inspect(|(&p1, &p2)| println!("{},{}", p1, p2))
                .map(|(&p1, &p2)| {
                    hap.get(&Relationship {
                        who: p1.clone(),
                        whom: p2.clone(),
                    })
                    .unwrap()
                        + hap
                            .get(&Relationship {
                                who: p2.clone(),
                                whom: p1.clone(),
                            })
                            .unwrap()
                })
                .sum::<i32>()
        })
        // .inspect(|&score| println!("{:?}", score))
        .max()
        .unwrap();

    println!("Optimum configuration: {}", opti);
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
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
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
