use crate::utils::input::read_input;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

struct RectIterator {
    x1: usize,
    x2: usize,
    y2: usize,
    x: usize,
    y: usize,
}

impl Rect {
    fn each(&self) -> RectIterator {
        RectIterator {
            x1: self.x1,
            x2: self.x2,
            y2: self.y2,
            x: self.x1,
            y: self.y1,
        }
    }
}

impl Iterator for RectIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.y2 {
            return None;
        }
        let opt = Some((self.x, self.y));
        self.x += 1;
        if self.x == self.x2 {
            self.x = self.x1;
            self.y += 1;
        }
        opt
    }
}

fn parse_coords(coords: &str) -> Rect {
    let mut words = coords.split(' ');
    let (mut c1, mut c2) = (
        words.next().unwrap().split(','),
        words.nth(1).unwrap().split(','),
    );
    Rect {
        x1: c1.next().unwrap().parse().unwrap(),
        y1: c1.next().unwrap().parse().unwrap(),
        x2: c2.next().unwrap().parse::<usize>().unwrap() + 1,
        y2: c2.next().unwrap().parse::<usize>().unwrap() + 1,
    }
}

fn parse_input(input: &str) -> Vec<(Action, Rect)> {
    input
        .lines()
        .map(|line| match &line[6..=6] {
            "n" => (Action::TurnOn, parse_coords(&line[8..])),
            "f" => (Action::TurnOff, parse_coords(&line[9..])),
            " " => (Action::Toggle, parse_coords(&line[7..])),
            _ => panic!("oops"),
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let instructions = parse_input(input);
    let mut grid = [[false; 1000]; 1000];

    for (a, r) in instructions {
        for (x, y) in r.each() {
            grid[x][y] = match a {
                Action::TurnOn => true,
                Action::TurnOff => false,
                Action::Toggle => !grid[x][y],
            }
        }
    }

    let lit: usize = grid
        .iter()
        .map(|col| col.iter().filter(|light| **light).count())
        .sum();

    println!("Lit lights: {}", lit);
}

fn solve_part_2(input: &str) {
    let instructions = parse_input(input);
    let mut grid = [[0u32; 1000]; 1000];

    for (a, r) in instructions {
        for (x, y) in r.each() {
            grid[x][y] = match a {
                Action::TurnOn => grid[x][y] + 1,
                Action::TurnOff => {
                    if grid[x][y] == 0 {
                        0
                    } else {
                        grid[x][y] - 1
                    }
                }
                Action::Toggle => grid[x][y] + 2,
            }
        }
    }

    let lit: u32 = grid.iter().map(|col| col.iter().sum::<u32>()).sum();

    println!("Lit lights: {}", lit);
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
