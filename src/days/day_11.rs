use crate::utils::input::read_input;

fn increment(pw: &mut [char]) {
    let lastpos = pw.len() - 1;
    let mut c = (pw[lastpos] as u8 + 1) as char;
    if c > 'z' {
        c = 'a';
        increment(&mut pw[..lastpos]);
    }
    pw[lastpos] = c;
}

fn is_valid(pw: &[char]) -> bool {
    if (0..pw.len() - 3)
        .all(|i| (pw[i] as u8 + 1 != pw[i + 1] as u8) || (pw[i] as u8 + 2 != pw[i + 2] as u8))
    {
        return false;
    }

    if pw.iter().any(|c| "iol".contains(*c)) {
        return false;
    }

    let mut doubledouble = false;
    for i in 0..pw.len() - 3 {
        if pw[i] == pw[i + 1] {
            for j in i + 2..pw.len() - 1 {
                if pw[j] != pw[i] && pw[j] == pw[j + 1] {
                    doubledouble = true;
                    break;
                }
            }
            break;
        }
    }
    if !doubledouble {
        return false;
    }

    true
}

fn solve_part_1(input: &str) {
    let mut pw: Vec<char> = input.trim().chars().collect();
    loop {
        increment(pw.as_mut_slice());
        if is_valid(pw.as_slice()) {
            break;
        }
    }

    let pw: String = pw.iter().collect();
    println!("New password: {}", pw);
}

fn solve_part_2(input: &str) {
    let mut pw: Vec<char> = input.trim().chars().collect();
    for _ in 0..2 {
        loop {
            increment(pw.as_mut_slice());
            if is_valid(pw.as_slice()) {
                break;
            }
        }
    }

    let pw: String = pw.iter().collect();
    println!("New password: {}", pw);
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
