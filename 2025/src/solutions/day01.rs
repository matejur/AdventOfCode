use anyhow::{Result, bail};
use std::str::FromStr;

#[derive(Debug)]
struct Instruction(i32, u16);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, rest) = s.split_at(1);

        let dist: u16 = rest
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid distance in '{s}'"))?;

        match dir {
            "L" => Ok(Self(-1, dist)),
            "R" => Ok(Self(1, dist)),
            other => bail!("Unknown direction '{other}' in '{s}'"),
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let instructions: Vec<Instruction> = input.lines().map(str::parse).collect::<Result<_>>()?;

    let mut count1 = 0;
    let mut count2 = 0;

    let mut dial: i32 = 50;

    for Instruction(sign, value) in instructions {
        for _ in 0..value {
            dial += sign;
            dial = dial.rem_euclid(100);

            if dial == 0 {
                count2 += 1;
            }
        }
        if dial == 0 {
            count1 += 1;
        }
    }

    Ok((count1.to_string(), count2.to_string()))
}
