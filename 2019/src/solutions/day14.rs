use std::collections::{HashMap, VecDeque};

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
    let mut incoming = HashMap::new();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for reaction in reactions.values() {
        incoming.insert(reaction.output.name, reaction.input.len());
        for inp in &reaction.input {
            graph
                .entry(inp.name)
                .or_default()
                .push(reaction.output.name);
        }
    }

    let mut queue = VecDeque::from(["ORE"]);
    let mut order = Vec::new();

    while let Some(n) = queue.pop_front() {
        order.push(n);
        if let Some(outputs) = graph.get(n) {
            for &m in outputs {
                let v = incoming.get_mut(m).unwrap();
                *v -= 1;
                if *v == 0 {
                    queue.push_back(m);
                }
            }
        }
    }

    order
}

fn pop_highest_priority<'a>(
    map: &mut HashMap<&'a str, usize>,
    order_index: &HashMap<&'a str, usize>,
) -> Option<(&'a str, usize)> {
    let (&name, _) = map.iter().max_by_key(|(k, _)| order_index[*k])?;
    let amount = map.remove(name).unwrap();
    Some((name, amount))
}

fn ore_for_n_fuel<'a>(
    n: usize,
    reactions: &HashMap<&'a str, Reaction<'a>>,
    order_index: &HashMap<&'a str, usize>,
) -> Result<usize> {
    let mut to_make = HashMap::new();
    to_make.insert("FUEL", n);

    let mut ore_needed = 0;
    while let Some((name, amount)) = pop_highest_priority(&mut to_make, order_index) {
        if name == "ORE" {
            ore_needed += amount;
            continue;
        }

        let reaction = &reactions[name];
        let times = div_ceil(amount, reaction.output.amount);

        for input in &reaction.input {
            *to_make.entry(input.name).or_insert(0) += input.amount * times;
        }
    }

    Ok(ore_needed)
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let reactions = parse(input)?;
    let order = topological(&reactions);
    let order_index: HashMap<&str, usize> =
        order.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let part1 = ore_for_n_fuel(1, &reactions, &order_index)?;

    let threshold: usize = 1_000_000_000_000;
    let mut min = 1;
    let mut max = threshold;

    let part2 = loop {
        let mid = (min + max).div_ceil(2);
        let ore = ore_for_n_fuel(mid, &reactions, &order_index)?;

        if ore < threshold {
            min = mid;
        } else {
            max = mid - 1;
        }

        if min >= max {
            break min;
        }
    };

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day_14,
    14,
    example = ("2210736", "460664"),
    input = ("628586", "3209254")
);
