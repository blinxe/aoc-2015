use crate::utils::input::read_input;

fn _parse_input(_input: &str) -> () {}

fn solve_part_1(input: &str) {
    let ups = input.chars().filter(|c| *c == '(').count();
    let downs = input.trim().len() - ups;
    let final_floor = ups - downs;

    println!("Final floor: {}", final_floor);
}

fn solve_part_2(input: &str) {
    let mut floor = 0;
    let mut pos = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            _ => {
                floor -= 1;
                if floor < 0 {
                    pos = i + 1;
                    break;
                }
            }
        }
    }

    println!("Underground at step: {}", pos);
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
