fn get_presents_for_house(n: usize) -> usize {
    let mut presents = 0;

    let mut elf = 1;
    let mut stop = n;
    while elf < stop {
        if n % elf == 0 {
            presents += 10 * elf;
            let q = n / elf;
            if q > elf {
                presents += 10 * q;
            }
            stop = q;
        }
        elf += 1;
    }

    presents
}

fn solve_part_1(input: usize) {
    for i in 1..input / 10 {
        let presents = get_presents_for_house(i);
        if presents >= input {
            println!("First house number at {} presents: {}", input, i);
            break;
        }
    }
}

fn get_presents_for_house_v2(n: usize) -> usize {
    let mut presents = 0;
    for visit_number in 1..=50 {
        if n % visit_number == 0 {
            let elf = n / visit_number;
            presents += 11 * elf;
        }
    }

    presents
}

fn solve_part_2(input: usize) {
    for i in 1..(input + 10) / 11 {
        let presents = get_presents_for_house_v2(i);
        if presents >= input {
            println!("First house number at {} presents: {}", input, i);
            break;
        }
    }
}

pub fn part_1() {
    solve_part_1(36000000);
}

pub fn part_2() {
    solve_part_2(36000000);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_1() {
        super::solve_part_1(9);
    }

    #[test]
    fn test_part_2() {}
}
