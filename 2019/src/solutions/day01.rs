use anyhow::Result;

use crate::day_test;

pub fn solve(input: &str) -> Result<(String, String)> {
    let masses: Vec<i32> = input.lines().filter_map(|line| line.parse().ok()).collect();

    let part1: i32 = masses.iter().map(|m| m / 3 - 2).sum();

    let part2: i32 = masses
        .iter()
        .map(|m| {
            let mut fuel = 0;
            let mut required = m / 3 - 2;
            while required > 0 {
                fuel += required;
                required = required / 3 - 2;
            }
            fuel
        })
        .sum();

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day01_test,
    1,
    example = ("33583", "50346"),
    input = ("3336439", "5001791")
);
