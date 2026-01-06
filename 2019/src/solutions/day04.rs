use anyhow::{Context, Result};

use crate::day_test;

fn dfs(
    pos: usize,
    last: u8,
    digits: &mut [u8; 6],
    run_len: u8,
    has_double: bool,
    has_pair: bool,
    lo: i32,
    hi: i32,
    part1: &mut i32,
    part2: &mut i32,
) {
    if pos == 6 {
        let n = digits.iter().fold(0, |a, &d| a * 10 + d as i32);

        if n < lo || n > hi {
            return;
        }

        if has_double {
            *part1 += 1;
        }
        if has_pair || run_len == 2 {
            *part2 += 1;
        }
        return;
    }

    for d in last..=9 {
        digits[pos] = d;

        if d == last {
            dfs(
                pos + 1,
                d,
                digits,
                run_len + 1,
                true,
                has_pair,
                lo,
                hi,
                part1,
                part2,
            );
        } else {
            dfs(
                pos + 1,
                d,
                digits,
                1,
                has_double,
                has_pair || run_len == 2,
                lo,
                hi,
                part1,
                part2,
            );
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let (a, b) = input.split_once('-').context("Malformed input")?;
    let lo: i32 = a.parse().context("Malformed input")?;
    let hi: i32 = b.parse().context("Malformed input")?;

    let mut part1 = 0;
    let mut part2 = 0;

    let mut digits = [0u8; 6];

    for d in 1..=9 {
        digits[0] = d;
        dfs(
            1,
            d,
            &mut digits,
            1,
            false,
            false,
            lo,
            hi,
            &mut part1,
            &mut part2,
        );
    }

    Ok((part1.to_string(), part2.to_string()))
}
day_test!(day04_test, 4, input = ("1767", "1192"));
