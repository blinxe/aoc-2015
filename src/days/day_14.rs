use std::collections::HashMap;

use crate::utils::input::read_input;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    stamina: u32,
    rest: u32,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            let name = words.next().unwrap().to_owned();
            let speed = words.nth(2).unwrap().parse().unwrap();
            let stamina = words.nth(2).unwrap().parse().unwrap();
            let rest = words.nth(6).unwrap().parse().unwrap();
            Reindeer {
                name,
                speed,
                stamina,
                rest,
            }
        })
        .collect()
}

fn get_distance(r: &Reindeer, time: u32) -> u32 {
    let periods = time / (r.stamina + r.rest);
    let rem = time % (r.stamina + r.rest);
    let rem_fly = rem.min(r.stamina);

    (periods * r.stamina + rem_fly) * r.speed
}

fn solve_part_1(input: &str) {
    let reindeers = parse_input(input);
    let max = reindeers
        .iter()
        .map(|r| get_distance(r, 2503))
        .max()
        .unwrap();
    println!("Longest distance: {}", max);
}

fn solve_part_2(input: &str) {
    let reindeers = parse_input(input);
    let mut scores: HashMap<_, _> = reindeers.iter().map(|r| (r, 0u32)).collect();
    for t in 1..=2503 {
        let mut dist: Vec<_> = reindeers.iter().map(|r| (r, get_distance(r, t))).collect();
        dist.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
        // println!("{:?}", dist);
        dist.iter()
            .take_while(|(_, d)| *d == dist[0].1)
            .for_each(|(r, _)| {
                scores.entry(r).and_modify(|s| *s += 1);
            });
    }

    let &max = scores.iter().max_by_key(|(_, s)| *s).unwrap().1;
    println!("Highest score: {}", max);
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
