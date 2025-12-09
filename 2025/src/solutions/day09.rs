use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn new(line: &str) -> Result<Self> {
        let mut numbers = line.split(',');

        Ok(Self {
            x: numbers.next().context("Malformed input (x)")?.parse()?,
            y: numbers.next().context("Malformed input (y)")?.parse()?,
        })
    }

    fn area_between(&self, other: &Tile) -> i64 {
        let dx = self.x - other.x + 1;
        let dy = self.y - other.y + 1;

        (dx * dy).abs()
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let tiles = input.lines().map(Tile::new).collect::<Result<Vec<_>>>()?;

    let mut max = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            max = max.max(tiles[i].area_between(&tiles[j]));
        }
    }
    Ok((max.to_string(), "".to_string()))
}
day_test!(
    day09_test,
    9,
    example = ("50", ""),
    input = ("4760959496", "")
);
