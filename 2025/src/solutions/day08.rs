use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]
struct Pair<'a> {
    first: &'a JunctionBox,
    second: &'a JunctionBox,
    distance: i64,
}

impl<'a> PartialEq for Pair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<'a> Eq for Pair<'a> {}

impl<'a> PartialOrd for Pair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Pair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circuit_idx: RefCell<usize>,
}

impl JunctionBox {
    fn new(id: usize, line: &str) -> Result<Self> {
        let mut numbers = line.split(',');

        Ok(Self {
            x: numbers.next().context("Malformed input")?.parse()?,
            y: numbers.next().context("Malformed input")?.parse()?,
            z: numbers.next().context("Malformed input")?.parse()?,
            circuit_idx: id.into(),
        })
    }

    fn dist_sqr(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let boxes = input
        .lines()
        .enumerate()
        .map(|(i, line)| JunctionBox::new(i, line))
        .collect::<Result<Vec<_>>>()?;

    let mut circuits: HashMap<usize, Vec<&JunctionBox>> = HashMap::new();
    for (i, box_) in boxes.iter().enumerate() {
        circuits.insert(i, vec![&box_]);
    }

    let mut heap = BinaryHeap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let first = &boxes[i];
            let second = &boxes[j];
            let distance = first.dist_sqr(second);

            heap.push(Pair {
                first,
                second,
                distance,
            });
        }
    }

    let mut connections_to_make = if boxes.len() == 20 { 10 } else { 1000 };
    while connections_to_make > 0 {
        let pair = heap.pop().context("Malformed input")?;
        connections_to_make -= 1;

        // Already in the same circuit
        if pair.first.circuit_idx == pair.second.circuit_idx {
            continue;
        }

        // Join them together
        let first_idx = *pair.first.circuit_idx.borrow();
        let second_idx = *pair.second.circuit_idx.borrow();

        let min_idx = first_idx.min(second_idx);
        let max_idx = first_idx.max(second_idx);

        let mut max_circuit_boxes = circuits.remove(&max_idx).context("Malformed")?;

        for b in &max_circuit_boxes {
            *b.circuit_idx.borrow_mut() = min_idx;
        }

        circuits
            .get_mut(&min_idx)
            .context("Malformed")?
            .append(&mut max_circuit_boxes);
    }

    let mut counts = circuits
        .iter()
        .map(|(_, circuit)| circuit.len())
        .collect::<Vec<_>>();
    counts.sort();

    let count1: usize = counts.iter().rev().take(3).product();
    let count2 = loop {
        let pair = heap.pop().context("Malformed input")?;

        if pair.first.circuit_idx == pair.second.circuit_idx {
            continue;
        }

        // Join them together
        let first_idx = *pair.first.circuit_idx.borrow();
        let second_idx = *pair.second.circuit_idx.borrow();

        let min_idx = first_idx.min(second_idx);
        let max_idx = first_idx.max(second_idx);

        let mut max_circuit_boxes = circuits.remove(&max_idx).context("Malformed")?;

        if circuits.len() == 1 {
            break pair.first.x * pair.second.x;
        }
        for b in &max_circuit_boxes {
            *b.circuit_idx.borrow_mut() = min_idx;
        }

        circuits
            .get_mut(&min_idx)
            .context("Malformed")?
            .append(&mut max_circuit_boxes);
    };

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day08_test,
    8,
    example = ("40", "25272"),
    input = ("63920", "1026594680")
);
