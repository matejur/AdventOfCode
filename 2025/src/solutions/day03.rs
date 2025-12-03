use anyhow::Result;

const SPACE_SIZE: usize = 100 * 12;

fn joltage(bank: &[u32], num_on: u32, wanted_on: u32, memo: &mut [Option<u64>; SPACE_SIZE]) -> u64 {
    if num_on == 0 || bank.len() == 0 {
        return 0;
    }

    let key = ((num_on - 1) * 100) as usize + bank.len() - 1;
    if let Some(m) = memo[key] {
        return m;
    }

    let power = 10_u64.pow(wanted_on - num_on);

    let on = bank[0] as u64 * power + joltage(&bank[1..], num_on - 1, wanted_on, memo);
    let off = joltage(&bank[1..], num_on, wanted_on, memo);

    let res = on.max(off);
    memo[key] = Some(res);
    res
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let banks = input.lines();

    let mut count1 = 0;
    let mut count2 = 0;

    for bank in banks {
        let bank: Vec<_> = bank.chars().filter_map(|c| c.to_digit(10)).rev().collect();

        let mut memo = [None; SPACE_SIZE];
        count1 += joltage(&bank, 2, 2, &mut memo);

        let mut memo = [None; SPACE_SIZE];
        count2 += joltage(&bank, 12, 12, &mut memo);
    }
    Ok((count1.to_string(), count2.to_string()))
}
