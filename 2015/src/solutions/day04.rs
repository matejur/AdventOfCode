use md5;

pub fn solve() {
    let input = include_str!("../../inputs/in04.txt");
    println!("Solving day 4");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut i = 0;

    loop {
        let str = input.to_owned() + &i.to_string();
        let digest = md5::compute(str);
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            return i.to_string();
        }
        i += 1;
    }
}

fn part2(input: &str) -> String {
    let mut i = 0;

    loop {
        let str = input.to_owned() + &i.to_string();
        let digest = md5::compute(str);
        let hash = format!("{:x}", digest);

        if hash.starts_with("000000") {
            return i.to_string();
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn day04_part1() {
        assert_eq!(part1("abcdef"), "609043");
        assert_eq!(part1("pqrstuv"), "1048970");
    }

    #[test]
    fn day04_part2() {}
}
