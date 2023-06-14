use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../../inputs/in24.txt");
    println!("Solving day 24");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug)]
struct NotFoundError;

fn get_quantum(weights: Vec<usize>, compartments: usize) -> Result<usize, NotFoundError> {
    let sum: usize = weights.iter().sum();
    let compartment_weight = sum / compartments;

    // This has probably some counter examples, because I'm not checking if it's posible to create 2(3) more combinations that weight the same
    for i in 1..=weights.len() {
        let min_quantum = weights
            .iter()
            .copied()
            .combinations(i)
            .filter(|v| v.iter().sum::<usize>() == compartment_weight)
            .map(|v| v.iter().product::<usize>())
            .min();

        if let Some(x) = min_quantum {
            return Ok(x);
        }
    }

    Err(NotFoundError)
}

fn part1(input: &str) -> String {
    let weights: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    get_quantum(weights, 3).unwrap().to_string()
}

fn part2(input: &str) -> String {
    let weights: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    get_quantum(weights, 4).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn day24_part1() {
        assert_eq!(part1(INPUT), "99");
    }

    #[test]
    fn day24_part2() {
        assert_eq!(part2(INPUT), "44");
    }
}
