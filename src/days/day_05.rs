use crate::utils::input::read_input;

fn is_nice_v1(s: &str) -> bool {
    let vowels = "aeiou";
    let has_3_vowels = s.chars().filter(|&c| vowels.contains(c)).count() >= 3;

    let has_double = (0..s.len() - 1).any(|i| s.chars().nth(i) == s.chars().nth(i + 1));

    let forbidden = ["ab", "cd", "pq", "xy"];
    let has_no_forbidden = (0..s.len() - 1).all(|i| !forbidden.contains(&&s[i..i + 2]));

    has_3_vowels && has_double && has_no_forbidden
}

fn is_nice_v2(s: &str) -> bool {
    let has_repeat = (0..s.len() - 3).any(|i| {
        let ll = &s[i..i + 2];
        s[i + 2..].contains(ll)
    });

    let has_mirror = (0..s.len() - 2).any(|i| s[i..=i] == s[i + 2..=i + 2]);

    has_repeat && has_mirror
}

fn solve_part_1(input: &str) {
    let nice = input.lines().filter(|line| is_nice_v1(line)).count();
    println!("Nice strings: {}", nice);
}

fn solve_part_2(input: &str) {
    let nice = input.lines().filter(|line| is_nice_v2(line)).count();
    println!("Nice strings: {}", nice);
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
        ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = indoc! {"
        qjhvhtzxzqqjkmpb
        xxyxx
        uurcxstgmygtbstg
        ieodomkazucvgmuy
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
