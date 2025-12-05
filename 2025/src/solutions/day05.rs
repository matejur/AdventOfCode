use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(inp: &str) -> Result<Self> {
        let (start, end) = inp.split_once('-').context("Malformed input")?;

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }

    fn includes(&self, id: u64) -> bool {
        self.start <= id && id <= self.end
    }
}

fn count_fresh(ranges: &mut Vec<Range>) -> u64 {
    ranges.sort_unstable();

    let mut res = ranges[0].end - ranges[0].start + 1;
    let mut prev_end = ranges[0].end;

    for r in ranges.iter().skip(1) {
        let (mut start, end) = (r.start, r.end);

        if start <= prev_end {
            start = prev_end + 1;
        }

        if start > end {
            continue;
        }

        res += end - start + 1;
        prev_end = end;
    }

    res
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut ranges = Vec::new();
    let mut reading_ranges = true;

    let mut count1 = 0;
    let mut count2 = 0;

    for line in input.lines() {
        if line == "" {
            count2 = count_fresh(&mut ranges);
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            ranges.push(Range::new(line)?);
            continue;
        }

        for range in &ranges {
            if range.includes(line.parse()?) {
                count1 += 1;
                break;
            }
        }
    }

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day05_test,
    5,
    example = ("3", "14"),
    input = ("611", "345995423801866")
);
