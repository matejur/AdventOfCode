use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use crate::day_test;

fn quantum_count(
    (x, mut y): (usize, usize),
    grid: &[&[u8]],
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    loop {
        y += 1;
        if grid.len() == y {
            return 1;
        }

        if grid[y][x] == b'^' {
            let mut calc = |nx| match memo.get(&(nx, y)) {
                Some(v) => *v,
                None => {
                    let res = quantum_count((nx, y), grid, memo);
                    memo.insert((nx, y), res);
                    res
                }
            };
            return calc(x - 1) + calc(x + 1);
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let start = (
        grid[0]
            .iter()
            .position(|ch| *ch == b'S')
            .context("Malformed input")?,
        0usize,
    );

    let mut stack = vec![start];

    let mut count1 = 0;
    let mut seen_bean = HashSet::new();
    let mut seen_splitters = HashSet::new();
    while let Some((x, mut y)) = stack.pop() {
        loop {
            y += 1;
            if y == grid.len() {
                break;
            }
            if grid[y][x] == b'^' {
                if seen_bean.insert((x - 1, y)) {
                    stack.push((x - 1, y));
                }
                if seen_bean.insert((x + 1, y)) {
                    stack.push((x + 1, y));
                }
                if seen_splitters.insert((x, y)) {
                    count1 += 1;
                }
                break;
            }
        }
    }

    let mut memo = HashMap::new();
    let count2 = quantum_count(start, &grid, &mut memo);

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day07_test,
    7,
    example = ("21", "40"),
    input = ("1518", "25489586715621")
);
