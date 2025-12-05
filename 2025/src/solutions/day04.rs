use anyhow::Result;

use crate::day_test;

const OFF: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_rolls(x: usize, y: usize, grid: &[u8], width: usize, height: usize) -> u32 {
    let x = x as isize;
    let y = y as isize;
    let w = width as isize;
    let h = height as isize;

    let mut count = 0;

    for &(dx, dy) in &OFF {
        let xx = x + dx;
        let yy = y + dy;

        if xx >= 0 && xx < w && yy >= 0 && yy < h {
            let idx = yy * w + xx;
            count += (grid[idx as usize] == b'@') as u32;
        }
    }

    count
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid: Vec<u8> = input.lines().flat_map(str::as_bytes).copied().collect();

    let mut next = grid.clone();

    let mut count1 = 0;
    let mut count2 = 0;
    let mut first = true;

    loop {
        let mut changed = false;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if grid[idx] == b'@' {
                    let rolls = count_rolls(x, y, &grid, width, height);

                    let stays_alive = rolls >= 4;
                    next[idx] = b'.' + (stays_alive as u8) * (b'@' - b'.');

                    if !stays_alive {
                        if first {
                            count1 += 1;
                        }
                        count2 += 1;

                        changed = true;
                    }
                } else {
                    next[idx] = b'.';
                }
            }
        }

        if !changed {
            break;
        }

        std::mem::swap(&mut grid, &mut next);
        first = false;
    }

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day04_test,
    4,
    example = ("13", "43"),
    input = ("1486", "9024")
);
