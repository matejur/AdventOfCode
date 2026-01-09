use std::f64::consts::PI;

use anyhow::Result;

use crate::day_test;

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let old_b = b;
        b = a % b;
        a = old_b;
    }
    a
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let height = input.lines().count() as i32;
    let width = input.lines().next().map_or(0, str::len) as i32;

    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x as i32, y as i32)))
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut dirs = Vec::new();
    let mut best_asteroid = 0;
    let mut gcd_cache = vec![0; (height * width) as usize];
    for i in 0..asteroids.len() {
        let mut unique_dirs = vec![false; (height * width * 4) as usize];
        let aster1 = &asteroids[i];
        for (j, aster2) in asteroids.iter().enumerate() {
            if i == j {
                continue;
            }

            let mut dx = aster1.0 - aster2.0;
            let mut dy = aster1.1 - aster2.1;

            let gcd_key = (dy.abs() * width + dx.abs()) as usize;
            let g = if gcd_cache[gcd_key] > 0 {
                gcd_cache[gcd_key]
            } else {
                gcd_cache[gcd_key] = gcd(dx, dy);
                gcd_cache[gcd_key]
            };
            dx /= g;
            dy /= g;

            dx += width;
            dy += height;
            let idx = (2 * width * dy + dx) as usize;

            unique_dirs[idx] = true;
        }

        let sum = unique_dirs.iter().filter(|&&x| x).count();
        if part1 < sum {
            part1 = sum;
            best_asteroid = i;
            dirs = unique_dirs
                .iter()
                .enumerate()
                .filter_map(|(idx, visible)| {
                    visible.then_some({
                        let x = (idx as i32 % (2 * width)) - width;
                        let y = (idx as i32 / (2 * width)) - height;
                        (x, y)
                    })
                })
                .collect();
        }
    }

    let mut dirs = dirs
        .iter()
        .map(|&(x, y)| {
            let angle = (3.0 * PI / 2.0 - (-y as f64).atan2(x as f64)).rem_euclid(2.0 * PI);
            (angle, x, y)
        })
        .collect::<Vec<_>>();

    dirs.sort_by(|(a1, _, _), (a2, _, _)| a1.partial_cmp(a2).unwrap());

    let (x, y) = asteroids[best_asteroid];
    let (_, dx, dy) = dirs[199];
    let part2 = (x - dx) * 100 + y - dy;

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day10_test, 10, input = ("282", "1008"));
