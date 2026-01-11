use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]
struct Chemical<'a> {
    name: &'a str,
    amount: usize,
}

#[derive(Debug)]
struct Reaction<'a> {
    input: Vec<Chemical<'a>>,
    output: Chemical<'a>,
}

impl<'a> Chemical<'a> {
    fn new(inp: &'a str) -> Result<Self> {
        let (num, name) = inp.split_once(' ').context("There should be a space")?;
        let num = num.parse()?;

        Ok(Self { name, amount: num })
    }
}

fn parse<'a>(input: &'a str) -> Result<HashMap<&'a str, Reaction<'a>>> {
    let mut reactions = HashMap::new();

    for reaction in input.lines() {
        let (left, right) = reaction
            .split_once(" => ")
            .context("There should be an =>")?;
        let result = Chemical::new(right)?;
        let ingredients = left
            .split(", ")
            .map(Chemical::new)
            .collect::<Result<Vec<_>>>()?;

        reactions.insert(
            result.name,
            Reaction {
                input: ingredients,
                output: result,
            },
        );
    }

    Ok(reactions)
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn topological<'a>(reactions: &HashMap<&'a str, Reaction<'a>>) -> Vec<&'a str> {
    let mut sorted = Vec::new();
    let mut set = HashSet::from(["ORE"]);

    let mut inp_to_out: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut num_incoming = HashMap::new();
    for reaction in reactions.values() {
        num_incoming.insert(reaction.output.name, reaction.input.len() as i32);
        for inp in &reaction.input {
            inp_to_out
                .entry(inp.name)
                .or_default()
                .push(reaction.output.name);
        }
    }

    while let Some(&n) = set.iter().next() {
        set.remove(n);
        sorted.push(n);

        for m in inp_to_out.get(n).unwrap_or(&vec![]) {
            let v = num_incoming.get_mut(m).expect("Should contain inner");
            *v -= 1;
            if *v == 0 {
                set.insert(m);
            }
        }
    }

    sorted
}

fn ore_for_n_fuel<'a>(
    n: usize,
    reactions: &HashMap<&'a str, Reaction<'a>>,
    order: &Vec<&str>,
) -> Result<usize> {
    let mut to_make = vec![Chemical {
        name: "FUEL",
        amount: n,
    }];

    let mut ore_needed = 0;
    while let Some(chemical) = to_make.pop() {
        if chemical.name == "ORE" {
            ore_needed += chemical.amount;
            continue;
        }

        let reaction = reactions
            .get(chemical.name)
            .context("All chemicals except ORE should be here")?;

        let needed = chemical.amount;
        let produced = reaction.output.amount;
        let num_reactions = div_ceil(needed, produced);

        for input in &reaction.input {
            if let Some(r) = to_make.iter_mut().filter(|c| c.name == input.name).next() {
                r.amount += input.amount * num_reactions;
            } else {
                to_make.push(Chemical {
                    name: input.name,
                    amount: input.amount * num_reactions,
                });
            }
        }
        to_make.sort_unstable_by_key(|i| order.iter().position(|v| *v == i.name).unwrap());
    }
    Ok(ore_needed)
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut reactions = parse(input)?;
    let order = topological(&reactions);
    let _ = reactions
        .values_mut()
        .map(|r| {
            r.input
                .sort_unstable_by_key(|i| order.iter().position(|v| *v == i.name).unwrap());
            r.input.reverse();
        })
        .collect::<Vec<_>>();

    let part1 = ore_for_n_fuel(1, &reactions, &order)?;

    let mut min = 1;
    let mut max = 10_000_000;

    let part2 = loop {
        let mid = (min + max) / 2;
        let ore = ore_for_n_fuel(mid, &reactions, &order)?;

        if ore < 1000000000000 {
            min = mid + 1;
        } else {
            max = mid;
        }

        if min >= max {
            if ore > 1000000000000 {
                break mid - 1;
            }
            break mid;
        }
    };

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day_00, 0, example = ("", ""), input = ("", ""));
