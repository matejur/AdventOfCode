use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::day_test;

const OFF: [(i64, i64); 4] = [(1, 0), (0, -1), (0, 1), (-1, 0)];

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

    fn uncompressed_area(&self, other: &Tile, xs_map: &[i64], ys_map: &[i64]) -> i64 {
        let dx = (xs_map[self.x as usize] - xs_map[other.x as usize]).abs() + 1;
        let dy = (ys_map[self.y as usize] - ys_map[other.y as usize]).abs() + 1;

        dx * dy
    }

    fn compressed_area(&self, other: &Tile) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;

        dx * dy
    }
}

fn fill_grid(grid: &mut [char], tiles: &[Tile], width: usize) {
    let mut draw_line = |p1: &Tile, p2: &Tile| {
        let start_y = p1.y.min(p2.y) as usize;
        let end_y = p1.y.max(p2.y) as usize;

        let start_x = p1.x.min(p2.x) as usize;
        let end_x = p1.x.max(p2.x) as usize;
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                grid[y * width + x] = '#';
            }
        }
    };

    for pair in tiles.windows(2) {
        let start = &pair[0];
        let end = &pair[1];

        draw_line(start, end);
    }

    draw_line(tiles.first().unwrap(), tiles.last().unwrap());

    let mut sx = 0;
    let mut sy = 0;

    'outer: for y in 0..grid.len() / width {
        for x in 0..width {
            let idx = y * width + x;
            if grid[idx] == '#' && grid[idx + 1] == ' ' && grid[idx + 2] == '#' {
                sx = x + 1;
                sy = y;
                break 'outer;
            }
        }
    }

    let mut stack = vec![(sx, sy)];

    while let Some((x, y)) = stack.pop() {
        grid[y * width + x] = '#';

        for (dx, dy) in OFF {
            let nx = (x as i64 + dx) as usize;
            let ny = (y as i64 + dy) as usize;

            if grid[ny * width + nx] == ' ' {
                stack.push((nx, ny));
            }
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut tiles = input.lines().map(Tile::new).collect::<Result<Vec<_>>>()?;

    let mut xs = tiles.iter().map(|t| t.x).collect::<Vec<_>>();
    let mut ys = tiles.iter().map(|t| t.y).collect::<Vec<_>>();
    xs.push(i64::MIN);
    ys.push(i64::MIN);

    xs.push(i64::MAX);
    ys.push(i64::MAX);

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let xs_compress_map: HashMap<i64, i64> = xs
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, num)| (num, idx as i64))
        .collect();

    let ys_compress_map: HashMap<i64, i64> = ys
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, num)| (num, idx as i64))
        .collect();

    for tile in tiles.iter_mut() {
        tile.x = *xs_compress_map
            .get(&tile.x)
            .expect("This x should be in the map");
        tile.y = *ys_compress_map
            .get(&tile.y)
            .expect("This y should be in the map");
    }

    let width = xs.len();
    let height = ys.len();
    let mut grid = vec![' '; width * height];
    fill_grid(&mut grid, &tiles, width);

    let mut compressed_area_grid = vec![0; width * height];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let v = i64::from(grid[y * width + x] == '#');
            let up = compressed_area_grid[(y - 1) * width + x];
            let up_left = compressed_area_grid[(y - 1) * width + x - 1];
            let left = compressed_area_grid[y * width + x - 1];

            compressed_area_grid[y * width + x] = v + up + left - up_left;
        }
    }

    let mut count1 = 0;
    let mut count2 = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let c1 = &tiles[i];
            let c2 = &tiles[j];

            let area = c1.uncompressed_area(c2, &xs, &ys);
            count1 = count1.max(area);

            let x1 = (c1.x.min(c2.x)) as usize - 1;
            let x2 = (c1.x.max(c2.x)) as usize;

            let y1 = (c1.y.min(c2.y)) as usize - 1;
            let y2 = (c1.y.max(c2.y)) as usize;

            let actual_area = compressed_area_grid[y2 * width + x2]
                + compressed_area_grid[y1 * width + x1]
                - compressed_area_grid[y1 * width + x2]
                - compressed_area_grid[y2 * width + x1];

            if c1.compressed_area(c2) == actual_area && area > count2 {
                count2 = area;
            }
        }
    }

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day09_test,
    9,
    example = ("50", "24"),
    input = ("4760959496", "1343576598")
);
