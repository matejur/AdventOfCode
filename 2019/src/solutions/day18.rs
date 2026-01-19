use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    ops::BitOrAssign,
};

use anyhow::Result;

use crate::day_test;

fn start_pos(input: &str) -> (usize, usize) {
    for (y, row) in input.lines().enumerate() {
        if let Some(x) = row.as_bytes().iter().position(|&c| c == b'@') {
            return (x, y);
        }
    }

    unreachable!("There should be an @ in input");
}

#[derive(Debug)]
struct Key {
    x: usize,
    y: usize,
    c: char,
}

fn find_keys(input: &str) -> Vec<Key> {
    let mut keys = Vec::new();

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.as_bytes().iter().enumerate() {
            if c.is_ascii_lowercase() {
                keys.push(Key {
                    x,
                    y,
                    c: *c as char,
                });
            }
        }
    }

    keys
}

#[derive(Clone, Copy)]
struct Bitmap(u32);

impl BitOrAssign<u32> for Bitmap {
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs;
    }
}

impl Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;

        while n != 0 {
            let bit = n.trailing_zeros() as u8;
            write!(f, "{}", (bit + b'A') as char)?;
            n &= n - 1;
        }
        writeln!(f)?;

        Ok(())
    }
}

fn construct_graph(map: &str, keys: &[Key]) -> HashMap<char, Vec<(char, usize, Bitmap)>> {
    let mut graph = HashMap::new();
    let width = map.lines().next().unwrap().len();
    let idx = |x, y| (y * (width + 1)) + x;
    let map = map.as_bytes();

    for start_key in keys {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([(start_key.x, start_key.y, 0, Bitmap(0))]);

        while let Some((x, y, dist, mut required_keys)) = queue.pop_front() {
            let c = map[idx(x, y)] as char;
            if c.is_ascii_lowercase() && c != start_key.c {
                graph
                    .entry(start_key.c)
                    .or_insert(Vec::new())
                    .push((c, dist, required_keys));
                graph
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((start_key.c, dist, required_keys));
            }

            if c.is_ascii_uppercase() {
                required_keys |= 1 << (c as u8 - b'A')
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = ((x as i64) + dx) as usize;
                let y = ((y as i64) + dy) as usize;

                if !seen.insert((x, y)) {
                    continue;
                }

                if map[idx(x, y)] == b'#' {
                    continue;
                }

                queue.push_back((x, y, dist + 1, required_keys));
            }
        }
    }

    graph
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    c: char,
    steps: usize,
    keys: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.c.cmp(&other.c))
            .then_with(|| self.keys.cmp(&other.keys))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> usize {
    let (sx, sy) = start_pos(input);
    let mut all_keys = find_keys(input);
    all_keys.push(Key {
        x: sx,
        y: sy,
        c: '@',
    });

    let graph = construct_graph(input, &all_keys);
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        c: '@',
        steps: 0,
        keys: 0,
    }]);
    let mut visited = HashSet::new();

    while let Some(State { c, steps, keys }) = queue.pop() {
        if !visited.insert((c, keys)) {
            continue;
        }

        if keys.count_ones() as usize == all_keys.len() - 1 {
            return steps;
        }

        let edges = graph.get(&c).expect("All keys should be in here");
        for (next_key, dist, required_keys) in edges {
            if keys & (1 << (*next_key as u8 - b'a')) > 0 || *next_key == '@' {
                continue;
            }

            if (required_keys.0 & keys) == required_keys.0 {
                queue.push(State {
                    c: *next_key,
                    steps: steps + dist,
                    keys: keys | 1 << (*next_key as u8 - b'a'),
                });
            }
        }
    }

    unreachable!("There should be a solution!");
}
pub fn solve(input: &str) -> Result<(String, String)> {
    let part1 = part1(input);

    Ok((part1.to_string(), "".to_string()))
}

day_test!(day00_test, 0, example = ("", ""), input = ("", ""));
