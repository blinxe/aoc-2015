use std::char;

use crate::utils::input::read_input;

fn to_mem(repr: &str) -> String {
    let mut iter = repr[1..repr.len() - 1].chars();
    let mut s = String::new();
    while let Some(c) = iter.next() {
        let insert: char;
        if c == '\\' {
            let escaped = iter.next().unwrap();
            match escaped {
                'x' => {
                    let hh: String = [iter.next().unwrap(), iter.next().unwrap()]
                        .into_iter()
                        .collect();
                    insert = u8::from_str_radix(&hh, 16).map(|n| n as char).unwrap();
                }
                _ => insert = escaped,
            }
        } else {
            insert = c;
        }
        s.push(insert);
    }
    println!("{} -> {}", repr, s);
    s
}

fn escape(raw: &str) -> String {
    let mut out = String::from('"');
    for c in raw.chars() {
        if c == '\\' || c == '"' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

fn solve_part_1(input: &str) {
    let raw = input.len() - input.lines().count();
    let mem: usize = input.lines().map(|line| to_mem(line).chars().count()).sum();
    println!("Difference: {}", raw - mem);
}

fn solve_part_2(input: &str) {
    let raw = input.len() - input.lines().count();
    let esc: usize = input.lines().map(|line| escape(line).chars().count()).sum();
    println!("Difference: {}", esc - raw);
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
        \"\"
        \"abc\"
        \"aaa\\\"aaa\"
        \"\\x27\"
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
