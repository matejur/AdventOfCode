use anyhow::Result;

use crate::{day_test, intcode::Computer};

fn alignment(map: &[u8], width: usize, height: usize) -> usize {
    let mut res = 0;
    let idx = |x: usize, y: usize| y * width + x;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if map[idx(x, y)] == b'#'
                && map[idx(x, y - 1)] == b'#'
                && map[idx(x, y + 1)] == b'#'
                && map[idx(x - 1, y)] == b'#'
                && map[idx(x + 1, y)] == b'#'
            {
                res += x * y;
            }
        }
    }

    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Straight,
    Left,
    Right,
    Back,
}

impl Turn {
    fn to_char(self) -> char {
        match self {
            Turn::Left => 'L',
            Turn::Right => 'R',
            _ => unreachable!("Robot should not have to turn back"),
        }
    }
}

impl Dir {
    fn all() -> [Dir; 4] {
        [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn relative_to(self, other: Dir) -> Turn {
        use Dir::*;
        match (self, other) {
            (a, b) if a == b => Turn::Straight,
            (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => Turn::Right,
            (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up) => Turn::Left,
            _ => Turn::Back,
        }
    }
}

fn get_next_dir(map: &[u8], robot_dir: Dir, robot_idx: usize, width: usize) -> Option<(Turn, Dir)> {
    for dir in Dir::all() {
        let next_idx = match dir {
            Dir::Up => robot_idx.checked_sub(width),
            Dir::Down => Some(robot_idx + width),
            Dir::Left => robot_idx.checked_sub(1),
            Dir::Right => Some(robot_idx + 1),
        };

        if let Some(idx) = next_idx
            && map.get(idx) == Some(&b'#')
        {
            let turn = robot_dir.relative_to(dir);

            if turn == Turn::Right || turn == Turn::Left {
                return Some((robot_dir.relative_to(dir), dir));
            }
        }
    }
    None
}

fn get_path(map: &mut [u8], width: usize, _height: usize) -> String {
    let mut path = String::new();

    let (robot_idx, &robot_char) = map
        .iter()
        .enumerate()
        .find(|(_, c)| matches!(**c, b'^' | b'v' | b'<' | b'>'))
        .expect("There should be a robot somewhere");

    let mut robot_x = (robot_idx % width) as i32;
    let mut robot_y = (robot_idx / width) as i32;

    let mut robot_dir = match robot_char {
        b'^' => Dir::Up,
        b'>' => Dir::Right,
        b'v' => Dir::Down,
        b'<' => Dir::Left,
        _ => unreachable!(),
    };

    map[robot_idx] = b'#';

    let idx = |x: i32, y: i32| (y * width as i32 + x) as usize;

    let (turn, dir) =
        get_next_dir(map, robot_dir, robot_idx, width).expect("Robot must have a path");
    robot_dir = dir;
    path.push(turn.to_char());

    loop {
        let (dx, dy) = robot_dir.delta();
        let mut steps = 0;

        let mut x = robot_x + dx;
        let mut y = robot_y + dy;

        while let Some(c) = map.get(idx(x, y))
            && (*c == b'#')
        {
            steps += 1;
            x += dx;
            y += dy;
        }

        robot_x = x - dx;
        robot_y = y - dy;

        path.push(',');
        path.push_str(&steps.to_string());

        let cur_idx = idx(robot_x, robot_y);
        if let Some((turn, dir)) = get_next_dir(map, robot_dir, cur_idx, width) {
            robot_dir = dir;
            path.push(',');
            path.push(turn.to_char());
        } else {
            break;
        }
    }

    path
}

fn compress(path: String) -> (String, String, String, String) {
    for size_a in (3..=21).rev() {
        let a = &path[..size_a];
        if !a.ends_with(",") {
            continue;
        }

        let a = &a[..a.len() - 1];
        let p1 = path.replace(a, "A");
        let leading_as = p1.split(",").take_while(|&t| t == "A").count();
        for size_b in (3..=21).rev() {
            let b = &path[leading_as * size_a..leading_as * size_a + size_b];
            if !b.ends_with(",") {
                continue;
            }

            let b = &b[..b.len() - 1];
            let p2 = p1.replace(b, "B");
            let leading_bs = p2[leading_as * 2..]
                .split(",")
                .take_while(|&t| t == "B")
                .count();
            for size_c in (3..=21).rev() {
                let c = &path[leading_as * size_a + leading_bs * size_b
                    ..leading_as * size_a + leading_bs * size_b + size_c];
                if !c.ends_with(",") {
                    continue;
                }

                let c = &c[..c.len() - 1];
                let p3 = p2.replace(c, "C");

                if p3.chars().all(|c| "ABC,".contains(c)) {
                    return (p3, a.to_string(), b.to_string(), c.to_string());
                }
            }
        }
    }

    unreachable!("There should be a solution!")
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut computer = Computer::new(input)?;
    computer.set(0, 2);

    let mut map = Vec::new();
    let mut height = 0;
    let mut width = 0;
    let mut first_line = true;
    while let Some(c) = computer.run_to_next_output() {
        if c as u8 == b'\n' {
            height += 1;

            if first_line {
                first_line = false;
            }
            continue;
        }

        if first_line {
            width += 1;
        }

        map.push(c as u8);
    }

    // After the map is drawn, robot says \nMain:\n
    height -= 2;

    let part1 = alignment(&map, width, height);

    let path = get_path(&mut map, width, height);
    let (main, a, b, c) = compress(path);

    for c in main.chars() {
        computer.push_input(c as i64);
    }
    computer.push_input('\n' as i64);

    for c in a.chars() {
        computer.push_input(c as i64);
    }
    computer.push_input('\n' as i64);

    for c in b.chars() {
        computer.push_input(c as i64);
    }
    computer.push_input('\n' as i64);

    for c in c.chars() {
        computer.push_input(c as i64);
    }
    computer.push_input('\n' as i64);

    computer.push_input('n' as i64);
    computer.push_input('\n' as i64);

    let part2 = computer.run_to_last_output().unwrap();

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day17_test, 17, input = ("7584", "1016738"));
