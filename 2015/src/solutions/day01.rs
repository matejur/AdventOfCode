pub fn solve() {
    let input = include_str!("../../inputs/in01.txt");
    println!("Solving day 1");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut floor = 0;

    input.chars().into_iter().for_each(|c| match c {
        '(' => floor = floor + 1,
        ')' => floor = floor - 1,
        _ => panic!("Invalid character"),
    });

    floor.to_string()
}

fn part2(input: &str) -> String {
    let mut floor: i32 = 0;

    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => panic!("Invalid character"),
        }

        if floor < 0 {
            return (i + 1).to_string();
        }
    }

    panic!("Never enters basement!");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day01_part1() {
        assert_eq!(part1("(())"), "0");
        assert_eq!(part1("()()"), "0");

        assert_eq!(part1("((("), "3");
        assert_eq!(part1("(()(()("), "3");
        assert_eq!(part1("))((((("), "3");

        assert_eq!(part1("())"), "-1");
        assert_eq!(part1("))("), "-1");

        assert_eq!(part1(")))"), "-3");
        assert_eq!(part1(")())())"), "-3");
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(")"), "1");
        assert_eq!(part2("()())"), "5");
    }
}
