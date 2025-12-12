use anyhow::{Context, Result};

use crate::day_test;

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut parts = input.split("\n\n").collect::<Vec<_>>();
    let regions = parts.pop().context("Malformed input")?;

    let shapes = parts
        .iter()
        .map(|s| s.chars().filter(|c| *c == '#').count())
        .collect::<Vec<_>>();

    let mut count1 = 0;
    for region in regions.lines() {
        let (area, presents) = region.split_once(": ").context("Malformed input")?;
        let available_area: usize = area
            .split("x")
            .map(|n| n.parse::<usize>().expect("Malformed input"))
            .product();

        let present_area: usize = presents
            .split_whitespace()
            .enumerate()
            .map(|(i, num)| shapes[i] * num.parse::<usize>().expect("Malformed input"))
            .sum();

        count1 += available_area / present_area;
    }
    Ok((count1.to_string(), "".to_string()))
}

day_test!(day12_test, 12, example = ("", ""), input = ("", ""));
