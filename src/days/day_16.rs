use std::collections::HashMap;

use crate::utils::input::read_input;

const CRITERIA: [&str; 10] = [
    "children",
    "cats",
    "samoyeds",
    "pomeranians",
    "akitas",
    "vizslas",
    "goldfish",
    "trees",
    "cars",
    "perfumes",
];

fn get_crit_id(s: &str) -> usize {
    CRITERIA.iter().take_while(|&&c| c != s).count()
}

type AuntMap = HashMap<usize, usize>;

fn parse_input(input: &str) -> (AuntMap, Vec<AuntMap>) {
    let reference: HashMap<_, _> = [
        (get_crit_id("children"), 3usize),
        (get_crit_id("cats"), 7usize),
        (get_crit_id("samoyeds"), 2usize),
        (get_crit_id("pomeranians"), 3usize),
        (get_crit_id("akitas"), 0usize),
        (get_crit_id("vizslas"), 0usize),
        (get_crit_id("goldfish"), 5usize),
        (get_crit_id("trees"), 3usize),
        (get_crit_id("cars"), 2usize),
        (get_crit_id("perfumes"), 1usize),
    ]
    .into_iter()
    .collect();

    let aunts = input
        .lines()
        .map(|line| {
            let pos = line.find(": ").unwrap();
            let line = &line[pos + 2..];
            line.split(", ")
                .map(|pair| {
                    let mut kv = pair.split(": ");
                    let crit = get_crit_id(kv.next().unwrap());
                    let count = kv.next().unwrap().parse::<usize>().unwrap();
                    (crit, count)
                })
                .collect()
        })
        .collect();

    (reference, aunts)
}

fn solve_part_1(input: &str) {
    let (reference, aunts) = parse_input(input);
    let (sue_id, _sue) = aunts
        .iter()
        .enumerate()
        .filter(|(_, a)| a.iter().all(|(k, v)| v == reference.get(k).unwrap()))
        .next()
        .unwrap();
    println!("{}", sue_id + 1);
}

fn solve_part_2(input: &str) {
    let (reference, aunts) = parse_input(input);
    let (sue_id, _sue) = aunts
        .iter()
        .enumerate()
        .filter(|(_, a)| {
            a.iter().all(|(crit_id, crit_count)| {
                let ref_count = reference.get(crit_id).unwrap();
                if *crit_id == 1 || *crit_id == 7 {
                    crit_count > ref_count
                } else if *crit_id == 3 || *crit_id == 6 {
                    crit_count < ref_count
                } else {
                    crit_count == ref_count
                }
            })
        })
        .next()
        .unwrap();
    println!("{}", sue_id + 1);
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
