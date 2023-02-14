use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../../inputs/in03.txt");
    println!("Solving day 3");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn step(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '^' => (pos.0, pos.1 + 1),
        'v' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        _ => panic!("Invalid direction"),
    }
}

fn part1(input: &str) -> String {
    let mut houses = HashSet::new();

    let mut santa = (0, 0);

    houses.insert(santa);

    for d in input.chars() {
        santa = step(santa, d);
        houses.insert(santa);
    }

    houses.len().to_string()
}

fn part2(input: &str) -> String {
    let mut houses = HashSet::new();

    let mut santa = (0, 0);
    let mut robot = (0, 0);

    houses.insert(santa);

    for (i, d) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa = step(santa, d);
            houses.insert(santa);
        } else {
            robot = step(robot, d);
            houses.insert(robot);
        }
    }

    houses.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day03_part1() {
        assert_eq!(part1(">"), "2");
        assert_eq!(part1("^>v<"), "4");
        assert_eq!(part1("^v^v^v^v^v"), "2");
    }

    #[test]
    fn day03_part2() {
        assert_eq!(part2("^v"), "3");
        assert_eq!(part2("^>v<"), "3");
        assert_eq!(part2("^v^v^v^v^v"), "11");
    }
}
