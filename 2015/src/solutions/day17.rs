pub fn solve() {
    let input = include_str!("../../inputs/in17.txt");
    println!("Solving day 17");
    println!("Part1: {}", part1(input, 150));
    println!("Part2: {}", part2(input, 150));
}

fn count(containers: &Vec<i32>, liters_to_go: i32, start: usize, limit: i32) -> i32 {
    if limit < 0 {
        return 0;
    }

    if liters_to_go == 0 {
        return 1;
    }

    if liters_to_go < 0 {
        return 0;
    }

    let mut result = 0;
    for i in start..containers.len() {
        result += count(containers, liters_to_go - containers[i], i + 1, limit - 1);
    }

    result
}

fn fewest(containers: &Vec<i32>, liters_to_go: i32, start: usize, num_of_containers: i32) -> i32 {
    if liters_to_go == 0 {
        return num_of_containers;
    }

    if liters_to_go < 0 {
        return i32::MAX;
    }

    let mut result = i32::MAX;
    for i in start..containers.len() {
        let r = fewest(
            containers,
            liters_to_go - containers[i],
            i + 1,
            num_of_containers + 1,
        );
        if r < result {
            result = r;
        }
    }

    result
}

fn part1(input: &str, liters: i32) -> String {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    count(&containers, liters, 0, i32::MAX).to_string()
}

fn part2(input: &str, liters: i32) -> String {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let min = fewest(&containers, liters, 0, 0);
    count(&containers, liters, 0, min).to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "20
15
10
5
5
";

    #[test]
    fn day17_part1() {
        assert_eq!(part1(INPUT, 25), "4");
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(INPUT, 25), "3");
    }
}
