#[cfg(feature = "breakout_display")]
use std::{fmt::Write, thread::sleep, time::Duration};

use anyhow::Result;

use crate::{
    day_test,
    intcode::{Computer, StopReason},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[94m";
pub const WHITE: &str = "\x1b[97m";
pub const HOME: &str = "\x1b[H";
pub const CLEAR: &str = "\x1b[J";

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            v => panic!("Invalid Tile id: {v}"),
        }
    }
}

const WIDTH: usize = 38;
const HEIGHT: usize = 22;

#[cfg(feature = "breakout_display")]
fn display(screen: &[Tile], score: i64) {
    let mut s = String::new();

    let _ = write!(s, "{BOLD}{WHITE}Score: {score}{RESET}");
    for (i, &tile) in screen.iter().enumerate() {
        if i % WIDTH == 0 {
            s.push('\n');
        }
        let _ = match tile {
            Tile::Empty => write!(s, " "),
            Tile::Wall => write!(s, "{GREEN}#{RESET}"),
            Tile::Block => write!(s, "{BLUE}@{RESET}"),
            Tile::Paddle => write!(s, "{WHITE}{BOLD}-{RESET}"),
            Tile::Ball => write!(s, "{RED}{BOLD}o{RESET}"),
        };
    }

    println!("{HOME}{CLEAR}{s}");
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut computer = Computer::new(input)?;
    let mut screen = [Tile::Empty; WIDTH * HEIGHT];
    computer.set(0, 2);

    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    loop {
        let Some(x) = computer.run_to_next_output() else {
            break;
        };

        let y = computer
            .run_to_next_output()
            .expect("Computer should output a 3-tuple");

        let tile_id = computer
            .run_to_next_output()
            .expect("Computer should output a 3-tuple");

        if x < 0 {
            break;
        }

        let tile = tile_id.into();
        if tile == Tile::Paddle {
            paddle = (x, y);
        }
        if tile == Tile::Ball {
            ball = (x, y);
        }

        let pixel = y as usize * WIDTH + x as usize;
        screen[pixel] = tile;
    }
    let part1 = screen.iter().filter(|&&t| t == Tile::Block).count();

    let mut score = 0;
    loop {
        match computer.run() {
            StopReason::Halted => break,
            StopReason::Output(x) => {
                let y = computer
                    .run_to_next_output()
                    .expect("Computer should output a 3-tuple");
                let v = computer
                    .run_to_next_output()
                    .expect("Computer should output a 3-tuple");

                if x < 0 {
                    score = v;
                    continue;
                }

                let tile: Tile = v.into();

                let pixel = y as usize * WIDTH + x as usize;
                if tile == Tile::Paddle {
                    let old_pixel = paddle.1 as usize * WIDTH + paddle.0 as usize;
                    screen[old_pixel] = Tile::Empty;
                    paddle = (x, y);
                }
                if tile == Tile::Ball {
                    let old_pixel = ball.1 as usize * WIDTH + ball.0 as usize;
                    screen[old_pixel] = Tile::Empty;
                    ball = (x, y);
                }
                screen[pixel] = tile;

                #[cfg(feature = "breakout_display")]
                {
                    display(&screen, score);
                    sleep(Duration::from_millis(20));
                }
            }
            StopReason::InputNeeded => computer.push_input((ball.0 - paddle.0).signum()),
        }
    }

    Ok((part1.to_string(), score.to_string()))
}

day_test!(day13_test, 13, input = ("255", "12338"));
