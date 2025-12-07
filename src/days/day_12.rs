use regex::Regex;

use crate::utils::input::read_input;

fn solve_part_1(input: &str) {
    let re = Regex::new("-?[0-9]+").unwrap();
    let sum: i32 = re
        .find_iter(input)
        // .inspect(|m| println!("{:?}", m))
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum();
    println!("Sum of all numbers: {}", sum);
}

fn solve_part_2(input: &str) {
    let mut input: Vec<char> = input.trim().chars().collect();
    let mut pos = 0;
    while pos < input.len() - 6 {
        if &input[pos..pos + 6] == [':', '"', 'r', 'e', 'd', '"'] {
            // backtrack to find start of obj
            let mut open = 1;
            let mut backtrack = 1;
            while open > 0 {
                match input[pos - backtrack] {
                    '}' => open += 1,
                    '{' => open -= 1,
                    _ => (),
                }
                backtrack += 1;
            }
            let start = pos - backtrack + 1;

            // advance to end of obj
            open = 1;
            pos += 1;
            while open > 0 {
                match input[pos] {
                    '{' => open += 1,
                    '}' => open -= 1,
                    _ => (),
                }
                pos += 1;
            }

            // println!("Remove: '{}'", input[start..pos].iter().collect::<String>());
            input.drain(start..pos);
            pos = start;
            continue; // don't increment pos again
        }
        pos += 1;
    }

    let clean: String = input.iter().collect();
    let re = Regex::new("-?[0-9]+").unwrap();
    let sum: i32 = re
        .find_iter(&clean)
        // .inspect(|m| println!("{:?}", m))
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum();
    println!("Sum of all numbers: {}", sum);
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
        [1,{\"c\":\"red\",\"b\":2},3]
        {\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}
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
