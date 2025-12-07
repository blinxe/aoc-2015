use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let pairs: Vec<_> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(" => ");
            let src = parts.next().unwrap().to_owned();
            let dst = parts.next().unwrap().to_owned();
            (src, dst)
        })
        .collect();

    let mut map = HashMap::new();
    for (k, v) in pairs {
        map.entry(k).or_insert(Vec::new()).push(v);
    }

    let medicine = input.lines().last().unwrap().to_owned();

    (map, medicine)
}

fn solve_part_1(input: &str) {
    let (map, medicine) = parse_input(input);
    let mut distinct: HashSet<String> = HashSet::new();
    for (k, vv) in map {
        for v in vv {
            let re = Regex::new(&k).unwrap();
            println!("{}", k);
            re.find_iter(&medicine)
                .inspect(|m| println!("{:?}", m.range()))
                .map(|m| {
                    let mut new = medicine.clone();
                    new.replace_range(m.range(), &v);
                    new
                })
                .for_each(|new| _ = distinct.insert(new));
        }
    }
    println!("Distinct new molecules: {}", distinct.len());
}

fn solve_part_2(input: &str) {}

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
