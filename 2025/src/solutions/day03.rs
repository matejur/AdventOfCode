use anyhow::Result;

fn joltage(bank: &[u32], wanted_on: usize) -> usize {
    let mut to_remove = bank.len() - wanted_on;
    let mut bank = Vec::from(bank);

    while to_remove > 0 {
        let len = bank.len();

        for i in 0..len - 1 {
            if bank[i] < bank[i + 1] {
                bank.remove(i);
                break;
            }
        }
        to_remove -= 1;
    }

    bank.iter()
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
