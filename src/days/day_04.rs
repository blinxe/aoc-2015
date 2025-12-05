use crate::utils::input::read_input;

use md5;

fn _parse_input(_input: &str) -> () {}

fn solve_part_1(input: &str) {
    let input = input.trim();
    let mut n = 0;
    loop {
        let s = format!("{}{}", input, n);
        let result = md5::compute(s.as_bytes());
        let hex = format!("{:x}", result);
        if hex.starts_with("00000") {
            break;
        }
        n += 1;
    }

    println!("{}", n);
}

fn solve_part_2(input: &str) {
    let input = input.trim();
    let mut n = 0;
    loop {
        let s = format!("{}{}", input, n);
        let result = md5::compute(s.as_bytes());
        if result.starts_with(&[0; 3]) {
            break;
        }
        n += 1;
    }

    println!("{}", n);
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
