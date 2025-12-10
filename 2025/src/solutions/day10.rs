use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    u32,
};

use anyhow::{Context, Result};

use crate::day_test;

struct Machine {
    wanted_lights: u16,
    buttons_bitmask: Vec<u16>,
    buttons: Vec<u16>,
    wanted_joltages: [u16; 10],
}

impl Machine {
    fn new(line: &str) -> Result<Self> {
        let mut parts = line.split_ascii_whitespace();

        let light_diagram = parts
            .next()
            .context("Malformed input (light diagram)")?
            .as_bytes();

        let mut wanted_lights = 0;
        for i in 1..light_diagram.len() - 1 {
            wanted_lights |= ((light_diagram[i] == b'#') as u16) << (i - 1);
        }

        let mut buttons_bitmask = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = [0; 10];
        while let Some(part) = parts.next() {
            let is_button = part.chars().nth(0).context("Malformed input")? == '(';
            let part = &part[1..part.len() - 1];
            let digits = part.split(',').map(|num| num.parse::<u16>());

            if is_button {
                let mut val = 0;
                for digit in digits {
                    let digit = digit.context("Malformed input (button number)")?;
                    val |= 1 << digit;
                    buttons.push(digit);
                }
                buttons_bitmask.push(val);
            } else {
                for (i, digit) in digits.enumerate() {
                    joltages[i] = digit?;
                }
            }
        }

        Ok(Self {
            wanted_lights,
            buttons,
            buttons_bitmask: buttons_bitmask,
            wanted_joltages: joltages,
        })
    }
}

fn part1(machine: &Machine) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let mut seen = HashSet::new();
    while let Some((num, light)) = queue.pop_front() {
        if light == machine.wanted_lights {
            return num;
        }

        for button in &machine.buttons_bitmask {
            if seen.insert(light ^ button) {
                queue.push_back((num + 1, light ^ button));
            }
        }
    }

    unreachable!();
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine:")?;
        writeln!(f, "Wanted: {:016b}", self.wanted_lights)?;
        write!(f, "Buttons: ")?;
        for button in &self.buttons_bitmask {
            write!(f, "{:016b} ", button)?;
        }
        writeln!(f, "\nJoltages: {:?}", self.wanted_joltages)
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let machines = input
        .lines()
        .map(Machine::new)
        .collect::<Result<Vec<_>>>()?;

    let count1: u32 = machines.iter().map(part1).sum();
    Ok((count1.to_string(), "".to_string()))
}

day_test!(day10_test, 10, example = ("7", ""), input = ("455", ""));
