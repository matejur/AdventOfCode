use anyhow::Result;

use crate::day_test;

fn joltage(bank: &[u32], wanted_on: usize) -> usize {
    let mut to_remove = bank.len() - wanted_on;
    let mut stack = Vec::new();

    for &batt in bank {
        while !stack.is_empty()
            && to_remove > 0
            && *stack.last().expect("Stack is not empty") < batt
        {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(batt);
    }

    stack
        .iter()
        .take(wanted_on)
        .enumerate()
        .map(|(i, bat)| *bat as usize * 10_usize.pow((wanted_on - i - 1) as u32))
        .sum()
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let banks = input.lines();

    let mut count1 = 0;
    let mut count2 = 0;

    for bank in banks {
        let bank: Vec<_> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        count1 += joltage(&bank, 2);
        count2 += joltage(&bank, 12);
    }
    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day03_test,
    3,
    example = ("357", "3121910778619"),
    input = ("17324", "171846613143331")
);
