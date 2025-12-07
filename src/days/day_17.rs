use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_valid_combinations(containers: &[usize], remaining_volume: usize) -> usize {
    let mut valid_count = 0;
    for (i, &c) in containers.iter().enumerate() {
        if c == remaining_volume {
            valid_count += 1;
        } else if c < remaining_volume {
            valid_count += get_valid_combinations(&containers[i + 1..], remaining_volume - c);
        }
    }
    valid_count
}

fn get_valid_min_combinations(containers: &[usize], remaining_volume: usize) -> (usize, usize) {
    let mut used = usize::MAX;
    let mut valid_count = 0;
    for (i, &c) in containers.iter().enumerate() {
        if c == remaining_volume {
            if used > 1 {
                valid_count = 0;
            }
            used = 1;
            valid_count += 1;
        } else if c < remaining_volume {
            let (valid_after, used_after) =
                get_valid_min_combinations(&containers[i + 1..], remaining_volume - c);
            if valid_after == 0 {
                continue;
            }
            if 1 + used_after < used {
                used = 1 + used_after;
                valid_count = valid_after;
            } else if 1 + used_after == used {
                valid_count += valid_after;
            }
        }
    }
    (valid_count, used)
}

fn solve_part_1(input: &str) {
    let containers = parse_input(input);
    let combinations = get_valid_combinations(&containers[..], 150);
    println!("Valid combinations: {}", combinations);
}

fn solve_part_2(input: &str) {
    let containers = parse_input(input);
    let (combinations, _) = get_valid_min_combinations(&containers[..], 150);
    println!("Valid combinations: {}", combinations);
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
        20
        15
        10
        5
        5
    "};

    #[test]
    fn test_part_1() {
        let containers = super::parse_input(EXAMPLE_1);
        let combinations = super::get_valid_combinations(&containers[..], 25);
        println!("Valid combinations: {}", combinations);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        let containers = super::parse_input(EXAMPLE_2);
        let (combinations, _) = super::get_valid_min_combinations(&containers[..], 25);
        println!("Valid combinations: {}", combinations);
    }
}
