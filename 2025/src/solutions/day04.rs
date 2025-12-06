use crate::day_test;
use anyhow::Result;

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

pub fn solve(input: &str) -> Result<(String, String)> {
    let raw: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let h = raw.len();
    let w = raw[0].len();

    let pw = w + 2;
    let ph = h + 2;

    let mut pad = vec![0u8; pw * ph];

    for y in 0..h {
        for x in 0..w {
            if raw[y][x] == b'@' {
                pad[(y + 1) * pw + (x + 1)] = 1;
            }
        }
    }

    let mut count = vec![0u8; pw * ph];
    let mut to_remove = Vec::new();

    for y in 1..=h {
        for x in 1..=w {
            if pad[y * pw + x] == 1 {
                let mut c = 0;
                for (dx, dy) in OFF {
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;
                    c += pad[ny * pw + nx];
                }
                count[y * pw + x] = c;
                if c < 4 {
                    to_remove.push((x, y));
                }
            }
        }
    }

    let count1 = to_remove.len();
    let mut count2 = 0;

    while let Some((x, y)) = to_remove.pop() {
        count2 += 1;

        for (dx, dy) in OFF {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let idx = ny * pw + nx;

            if count[idx] == 4 {
                to_remove.push((nx, ny));
            }

            count[idx] = count[idx].saturating_sub(1);
        }
    }

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day04_test,
    4,
    example = ("13", "43"),
    input = ("1486", "9024")
);
