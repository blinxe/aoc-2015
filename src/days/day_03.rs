use std::collections::HashSet;

use crate::utils::input::read_input;

fn _parse_input(_input: &str) -> () {}

fn solve_part_1(input: &str) {
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    let mut x = 0;
    let mut y = 0;
    for c in input.trim().chars() {
        match c {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!(),
        }
        houses.insert((x, y));
    }

    let visited = houses.len();

    println!("Visited houses: {}", visited);
}

fn solve_part_2(input: &str) {
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    let (mut sx, mut sy) = (0, 0);
    let (mut rx, mut ry) = (0, 0);
    let mut robo = false;
    for c in input.trim().chars() {
        let (x, y) = if robo {
            (&mut rx, &mut ry)
        } else {
            (&mut sx, &mut sy)
        };
        match c {
            '^' => *y -= 1,
            'v' => *y += 1,
            '<' => *x -= 1,
            '>' => *x += 1,
            _ => panic!(),
        }
        houses.insert((*x, *y));
        robo = !robo;
    }

    let visited = houses.len();

    println!("Visited houses: {}", visited);
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
