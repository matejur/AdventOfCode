pub fn solve() {
    let input = include_str!("../../inputs/in02.txt");
    println!("Solving day 2");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for present in input.lines() {
        let mut sizes: Vec<i32> = present
            .split('x')
            .map(|num| num.parse().unwrap())
            .take(3)
            .collect();

        sizes.sort();

        let l = sizes[0];
        let h = sizes[1];
        let w = sizes[2];

        let lw = l * w;
        let lh = l * h;
        let hw = h * w;

        sum += lh + 2 * (lw + lh + hw);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for present in input.lines() {
        let mut sizes: Vec<i32> = present
            .split('x')
            .map(|num| num.parse().unwrap())
            .take(3)
            .collect();

        sizes.sort();

        let l = sizes[0];
        let h = sizes[1];
        let w = sizes[2];

        let volume = l * h * w;

        sum += volume + 2 * (l + h);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day02_part1() {
        assert_eq!(part1("2x3x4"), "58");
        assert_eq!(part1("1x1x10"), "43");
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2("2x3x4"), "34");
        assert_eq!(part2("1x1x10"), "14");
    }
}
