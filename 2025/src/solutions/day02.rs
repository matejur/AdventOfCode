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

    fn count1(&self) -> i64 {
        let mut res = 0;

        for val in self.start..=self.end {
            let digits = val.ilog10() + 1;

            if digits % 2 == 1 {
                continue;
            }

            let half = digits / 2;
            let pow = 10_i64.pow(half);

            let left = val / pow;
            let right = val % pow;

            if left == right {
                res += val;
            }
        }

        res
    }

    fn count2(&self) -> i64 {
        let mut res = 0;
        for val in self.start..=self.end {
            let digits = val.ilog10() + 1;
            let pow_cache = |k| 10_i64.pow(k);

            'divisors: for d in divisors(digits) {
                let block = val % pow_cache(d);
                let blocks = digits / d;

                for i in 1..blocks {
                    let part = (val / pow_cache(i * d)) % pow_cache(d);
                    if part != block {
                        continue 'divisors;
                    }
                }

                res += val;
                break;
            }
        }
        res
    }
}

fn divisors(n: u32) -> Vec<u32> {
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

    let part1: i64 = ranges.iter().map(Range::count1).sum();
    let part2: i64 = ranges.iter().map(Range::count2).sum();

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day02_test,
    2,
    example = ("1227775554", "4174379265"),
    input = ("35367539282", "45814076230")
);
