use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]

struct Pair {
    first: usize,
    second: usize,
    distance: i64,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    idx: usize,
}

impl JunctionBox {
    fn new(id: usize, line: &str) -> Result<Self> {
        let mut numbers = line.split(',');

        Ok(Self {
            x: numbers.next().context("Malformed input")?.parse()?,
            y: numbers.next().context("Malformed input")?.parse()?,
            z: numbers.next().context("Malformed input")?.parse()?,
            idx: id,
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
    let mut box_to_circuit: Vec<usize> = (0..boxes.len()).collect();
    for (i, box_) in boxes.iter().enumerate() {
        circuits.insert(i, vec![&box_]);
    }

    let mut vec = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let first = &boxes[i];
            let second = &boxes[j];
            vec.push(Pair {
                distance: first.dist_sqr(second),
                first: i,
                second: j,
            });
        }
    }

    let mut heap = BinaryHeap::from(vec);

    let mut connections_to_make = if boxes.len() == 20 { 10 } else { 1000 };
    let mut count1 = 0;
    let count2;

    loop {
        if connections_to_make == 0 {
            let mut counts = circuits
                .iter()
                .map(|(_, circuit)| circuit.len())
                .collect::<Vec<_>>();
            counts.sort();

            count1 = counts.iter().rev().take(3).product();
        }

        let pair = heap.pop().context("Malformed input")?;
        connections_to_make -= 1;

        let box1 = &boxes[pair.first];
        let box2 = &boxes[pair.second];

        let first_box_circuit_idx = box_to_circuit[box1.idx];
        let second_box_citcuit_idx = box_to_circuit[box2.idx];
        // Already in the same circuit

        if first_box_circuit_idx == second_box_citcuit_idx {
            continue;
        }

        // Join them together
        let min_idx = first_box_circuit_idx.min(second_box_citcuit_idx);
        let max_idx = first_box_circuit_idx.max(second_box_citcuit_idx);

        let mut max_circuit_boxes = circuits.remove(&max_idx).context("Malformed")?;

        if circuits.len() == 1 {
            count2 = box1.x * box2.x;
            break;
        }

        for b in &max_circuit_boxes {
            box_to_circuit[b.idx] = min_idx;
        }

        circuits
            .get_mut(&min_idx)
            .context("Malformed")?
            .append(&mut max_circuit_boxes);
    }

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day08_test,
    8,
    example = ("40", "25272"),
    input = ("63920", "1026594680")
);
