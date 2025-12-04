use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|dim| dim.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn get_paper_area(pkg: &mut [u32; 3]) -> u32 {
    let base = 2 * (pkg[0] * pkg[1] + pkg[1] * pkg[2] + pkg[2] * pkg[0]);
    pkg.sort();
    let extra = pkg[0] * pkg[1];
    base + extra
}

fn get_ribbon_length(pkg: &mut [u32; 3]) -> u32 {
    let volume = pkg[0] * pkg[1] * pkg[2];
    pkg.sort();
    let perimeter = 2 * pkg[0] + 2 * pkg[1];
    volume + perimeter
}

fn solve_part_1(input: &str) {
    let mut pkgs = parse_input(input);
    let area: u32 = pkgs.iter_mut().map(|p| get_paper_area(p)).sum();

    println!("Needed paper area: {}", area);
}

fn solve_part_2(input: &str) {
    let mut pkgs = parse_input(input);
    let length: u32 = pkgs.iter_mut().map(|p| get_ribbon_length(p)).sum();

    println!("Needed ribbon length: {}", length);
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
        2x3x4
        1x1x10
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
