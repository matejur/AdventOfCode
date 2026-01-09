use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    steps: i64,
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<(i64, i64, i64)> {
        let (x1, y1, x2, y2) = (self.x1, self.y1, self.x2, self.y2);
        let (x3, y3, x4, y4) = (other.x1, other.y1, other.x2, other.y2);

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denom == 0 {
            return None;
        }

        let num_x = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let num_y = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        let px = num_x / denom;
        let py = num_y / denom;

        if px == 0 && py == 0 {
            return None;
        }

        if Self::on_segment(px, py, x1, y1, x2, y2) && Self::on_segment(px, py, x3, y3, x4, y4) {
            let steps1 = self.steps + (px - self.x1).abs() + (py - self.y1).abs();
            let steps2 = other.steps + (px - other.x1).abs() + (py - other.y1).abs();
            Some((px, py, steps1 + steps2))
        } else {
            None
        }
    }

    fn on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
        px >= x1.min(x2) && px <= x1.max(x2) && py >= y1.min(y2) && py <= y1.max(y2)
    }
}

fn wire_to_segments(wire: &str) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    let mut segments = Vec::new();
    for seg in wire.split(',') {
        let mut chars = seg.chars();
        let dir = chars.next().expect("Malformed input");
        let value = chars.as_str().parse::<i64>().expect("Malformed input");
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            c => panic!("Unrecognized: {c}"),
        };

        segments.push(Line {
            x1: x,
            y1: y,
            x2: x + dx * value,
            y2: y + dy * value,
            steps,
        });
        x += dx * value;
        y += dy * value;
        steps += dx.abs() * value + dy.abs() * value;
    }

    segments
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let (wire1, wire2) = input.split_once('\n').context("Malformed input")?;

    let wire1 = wire_to_segments(wire1);
    let wire2 = wire_to_segments(wire2);

    let mut part1 = i64::MAX;
    let mut part2 = i64::MAX;
    for seg1 in &wire1 {
        for seg2 in &wire2 {
            if let Some((x, y, steps)) = seg1.intersect(seg2) {
                part1 = part1.min(x.abs() + y.abs());
                part2 = part2.min(steps);
            }
        }
    }

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day03_test,
    3,
    example = ("159", "610"),
    input = ("855", "11238")
);
