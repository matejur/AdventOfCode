use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

use crate::{day_test, intcode::Computer};

enum Dir {
    North = 1,
    South,
    West,
    East,
}

struct Movement {
    dir: Dir,
    back: Dir,
    dx: i32,
    dy: i32,
}

const MOVEMENTS: [Movement; 4] = [
    Movement {
        dir: Dir::North,
        back: Dir::South,
        dx: 0,
        dy: -1,
    },
    Movement {
        dir: Dir::South,
        back: Dir::North,
        dx: 0,
        dy: 1,
    },
    Movement {
        dir: Dir::West,
        back: Dir::East,
        dx: -1,
        dy: 0,
    },
    Movement {
        dir: Dir::East,
        back: Dir::West,
        dx: 1,
        dy: 0,
    },
];

#[derive(Debug)]
enum Tile {
    Empty,
    Goal,
}

fn explore(
    robot: &mut Computer,
    x: i32,
    y: i32,
    seen: &mut HashSet<(i32, i32)>,
    tiles: &mut HashMap<(i32, i32), Tile>,
) {
    for movement in MOVEMENTS {
        let nx = x + movement.dx;
        let ny = y + movement.dy;

        if !seen.insert((nx, ny)) {
            continue;
        }

        robot.push_input(movement.dir as i64);
        match robot
            .run_to_next_output()
            .expect("This robot should never halt")
        {
            1 => {
                tiles.insert((nx, ny), Tile::Empty);
                explore(robot, nx, ny, seen, tiles);
                robot.push_input(movement.back as i64);
                let _ = robot.run_to_next_output();
            }
            2 => {
                tiles.insert((nx, ny), Tile::Goal);
                explore(robot, nx, ny, seen, tiles);
                robot.push_input(movement.back as i64);
                let _ = robot.run_to_next_output();
            }
            _ => {}
        };

        seen.remove(&(nx, ny));
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut robot = Computer::new(input)?;
    let mut seen = HashSet::new();
    let mut tiles = HashMap::new();

    explore(&mut robot, 0, 0, &mut seen, &mut tiles);

    let (min_x, max_x, min_y, max_y) =
        tiles
            .iter()
            .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), ((x, y), _)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            });

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut board = vec!['#'; ((width + 2) * (height + 2)) as usize];

    let mut sx = 0;
    let mut sy = 0;
    for ((x, y), tile) in tiles {
        let x = x - min_x + 1;
        let y = y - min_y + 1;
        let idx = (y * (width + 2) + x) as usize;
        board[idx] = match tile {
            Tile::Empty => ' ',
            Tile::Goal => {
                sx = x;
                sy = y;
                'S'
            }
        }
    }

    let mut queue = VecDeque::from([(sx, sy, 0)]);
    let mut seen = vec![false; ((width + 2) * (height + 2)) as usize];

    let mut part1 = 0;
    let mut part2 = 0;
    while let Some((x, y, num)) = queue.pop_front() {
        if x == -min_x + 1 && y == -min_y + 1 {
            part1 = num;
        }
        part2 = num;

        for movement in MOVEMENTS {
            let nx = x + movement.dx;
            let ny = y + movement.dy;

            let idx = (ny * (width + 2) + nx) as usize;
            if board[idx] == '#' {
                continue;
            }

            if seen[idx] {
                continue;
            }
            seen[idx] = true;

            queue.push_back((nx, ny, num + 1));
        }
    }

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day15_test, 15, input = ("318", "390"));
