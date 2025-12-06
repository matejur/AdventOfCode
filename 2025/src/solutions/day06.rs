use anyhow::Result;

use crate::day_test;

fn transpose(input: &str) -> String {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut out = String::with_capacity(input.len());

    for i in 0..max_len {
        for line in &lines {
            if let Some(&b) = line.get(i) {
                match b {
                    b' ' | b'*' | b'+' => {}
                    _ => out.push(b as char),
                }
            }
        }
        out.push('\n');
    }

    out
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut problems: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, token) in line.split_ascii_whitespace().enumerate() {
            if row_idx == 0 {
                problems.push(Vec::new());
            }

            let c = token.as_bytes()[0] as char;
            if c == '*' || c == '+' {
                operations.push(c);
            } else {
                problems[col_idx].push(token.parse()?);
            }
        }
    }

    let count1: i64 = problems
        .iter()
        .zip(&operations)
        .map(|(nums, &op)| {
            nums.iter()
                .skip(1)
                .fold(nums[0], |acc, &n| if op == '*' { acc * n } else { acc + n })
        })
        .sum();

    let transposed = transpose(input);
    let count2: i64 = transposed
        .split("\n\n")
        .zip(operations)
        .map(|(block, op)| {
            let mut nums = block.lines().filter_map(|l| l.parse::<i64>().ok());

            let first = nums.next().expect("at least one value");
            nums.fold(first, |acc, n| if op == '*' { acc * n } else { acc + n })
        })
        .sum();

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day06_test,
    6,
    example = ("4277556", "3263827"),
    input = ("4412382293768", "7858808482092")
);
