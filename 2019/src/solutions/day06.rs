use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Context, Result};

use crate::day_test;

fn count_orbits(object: &str, distance: i32, map: &HashMap<&str, Vec<&str>>) -> i32 {
    map.get(object).map_or(distance, |children| {
        distance
            + children
                .iter()
                .map(|child| count_orbits(child, distance + 1, map))
                .sum::<i32>()
    })
}

fn find_santa(
    orbit_children: &HashMap<&str, Vec<&str>>,
    orbit_parents: &HashMap<&str, &str>,
) -> i32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let start = orbit_parents
        .get("YOU")
        .expect("YOU is orbiting around an object");
    let target = orbit_parents
        .get("SAN")
        .expect("SAN is orbiting around an object");

    queue.push_back((start, 0));

    while let Some((object, transfers)) = queue.pop_front() {
        if object == target {
            return transfers;
        }

        if !seen.insert(object) {
            continue;
        }

        if let Some(parent) = orbit_parents.get(object) {
            queue.push_back((parent, transfers + 1));
        }

        if let Some(children) = orbit_children.get(object) {
            children
                .iter()
                .for_each(|child| queue.push_back((child, transfers + 1)));
        }
    }

    unreachable!()
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut orbit_children = HashMap::new();
    let mut orbit_parents = HashMap::new();

    for pair in input.lines() {
        let (first, second) = pair.split_once(")").context("Expected ) in each line")?;
        orbit_children
            .entry(first)
            .or_insert(Vec::new())
            .push(second);
        orbit_parents.insert(second, first);
    }

    Ok((
        count_orbits("COM", 0, &orbit_children).to_string(),
        find_santa(&orbit_children, &orbit_parents).to_string(),
    ))
}

day_test!(
    day06_test,
    6,
    example = ("54", "4"),
    input = ("139597", "286")
);
