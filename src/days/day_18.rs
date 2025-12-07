use std::collections::HashSet;

use crate::utils::input::read_input;

type Lights = HashSet<(i32, i32)>;

fn parse_input(input: &str) -> Lights {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_neighbor_count(lights: &Lights, x: i32, y: i32) -> usize {
    let mut count = 0;
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        if lights.contains(&(x + dx, y + dy)) {
            count += 1;
        }
    }

    count
}

fn step(old: &Lights) -> Lights {
    let mut new = Lights::new();
    for y in 0..100 {
        for x in 0..100 {
            let n = get_neighbor_count(old, x, y);
            if old.contains(&(x, y)) {
                if n == 2 || n == 3 {
                    new.insert((x, y));
                }
            } else {
                if n == 3 {
                    new.insert((x, y));
                }
            }
        }
    }

    new
}

fn solve_part_1(input: &str) {
    let mut on = parse_input(input);
    for _ in 0..100 {
        on = step(&on);
    }

    println!("ON lights: {}", on.len());
}

fn solve_part_2(input: &str) {
    let mut on = parse_input(input);
    let corners = [(0, 0), (99, 0), (0, 99), (99, 99)];
    for (x, y) in corners {
        on.insert((x, y));
    }
    for _ in 0..100 {
        on = step(&on);
        for (x, y) in corners {
            on.insert((x, y));
        }
    }

    println!("ON lights: {}", on.len());
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
