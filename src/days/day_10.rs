use crate::utils::input::read_input;

fn iterate(s: &str) -> String {
    let mut out = String::new();
    let mut iter = s.chars();

    let mut cref = iter.next().unwrap();
    let mut count = 1;
    while let Some(c) = iter.next() {
        if c == cref {
            count += 1;
        } else {
            out.push_str(&count.to_string());
            out.push(cref);
            cref = c;
            count = 1;
        }
    }
    out.push_str(&count.to_string());
    out.push(cref);

    out
}

fn solve_part_1(input: &str) {
    let mut acc = input.trim().to_owned();
    for _ in 0..40 {
        acc = iterate(&acc);
    }
    println!("{}", acc.len());
}

fn solve_part_2(input: &str) {
    let mut acc = input.trim().to_owned();
    for _ in 0..50 {
        acc = iterate(&acc);
    }
    println!("{}", acc.len());
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
        111221
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
