use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(inp: &str) -> Result<Self> {
        let (start, end) = inp.split_once('-').context("Malformed input")?;

        Ok(Self {
            start: start.parse().context("Not a number")?,
            end: end.parse().context("Not a number")?,
        })
    }

    fn count(&self, all_block_sizes: bool) -> i64 {
        let start_digits = self.start.ilog10() + 1;
        let end_digits = self.end.ilog10() + 1;

        if start_digits < end_digits {
            let r1 = Range {
                start: self.start,
                end: 10i64.pow(end_digits - 1) - 1,
            };

            let r2 = Range {
                start: 10i64.pow(end_digits - 1),
                end: self.end,
            };

            return r1.count(all_block_sizes) + r2.count(all_block_sizes);
        }

        let block_sizes = if all_block_sizes {
            block_sizes(start_digits)
        } else {
            if start_digits % 2 == 1 {
                return 0;
            }
            vec![start_digits / 2]
        };

        let mut res = 0;
        let mut seen = HashSet::new();

        for block in block_sizes {
            let pow = 10i64.pow(start_digits - block);

            let start_block = self.start / pow;
            let end_block = self.end / pow;

            for val in start_block..=end_block {
                let recons = reconstruct(val, block, start_digits);
                if self.start <= recons && recons <= self.end && !seen.contains(&recons) {
                    res += recons;
                    seen.insert(recons);
                }
            }
        }

        res
    }
}

fn reconstruct(block: i64, size: u32, all_digits: u32) -> i64 {
    let mut out = 0;
    let pow = 10i64.pow(size);
    for _ in 0..all_digits / size {
        out *= pow;
        out += block;
    }

    out
}

fn block_sizes(n: u32) -> Vec<u32> {
    let mut results = Vec::new();

    for d in 1..n {
        if n % d == 0 {
            results.push(d);
        }
    }

    results
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let ranges: Vec<Range> = input.split(",").map(Range::new).collect::<Result<_>>()?;

    let part1: i64 = ranges.iter().map(|r| r.count(false)).sum();
    let part2: i64 = ranges.iter().map(|r| r.count(true)).sum();

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day02_test,
    2,
    example = ("1227775554", "4174379265"),
    input = ("35367539282", "45814076230")
);
