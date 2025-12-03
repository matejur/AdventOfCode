use anyhow::{Result, bail};
use std::str::FromStr;

#[derive(Debug)]
struct Instruction(i32);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, rest) = s.split_at(1);

        let dist: i32 = rest
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid distance in '{s}'"))?;

        match dir {
            "L" => Ok(Self(-dist)),
            "R" => Ok(Self(dist)),
            other => bail!("Unknown direction '{other}' in '{s}'"),
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let instructions: Vec<Instruction> = input.lines().map(str::parse).collect::<Result<_>>()?;

    let mut count1 = 0;
    let mut count2 = 0;

    let mut dial: i32 = 50;

    for Instruction(value) in instructions {
        let prev = dial;

        let hundreds = value.abs() / 100;
        count2 += hundreds;

        let shift = value % 100;
        dial += shift;

        if (dial <= 0 || 99 < dial) && prev != 0 {
            count2 += 1;
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            count1 += 1;
        }
    }

    Ok((count1.to_string(), count2.to_string()))
}
