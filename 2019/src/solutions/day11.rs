use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::{day_test, intcode::Computer};

#[repr(i64)]
#[derive(Copy, Clone, Debug)]
enum Panel {
    Black = 0,
    White = 1,
}

impl TryFrom<i64> for Panel {
    type Error = anyhow::Error;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Panel::Black),
            1 => Ok(Panel::White),
            _ => bail!("Invalid color integer"),
        }
    }
}

fn run_robot(mut robot: Computer, panels: &mut HashMap<(i32, i32), Panel>) {
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1);

    loop {
        let panel = panels.entry((x, y)).or_insert(Panel::Black);

        robot.push_input(*panel as i64);

        let Some(paint_color) = robot.run_to_next_output() else {
            break;
        };

        let turn = robot
            .run_to_next_output()
            .expect("Robot should always output 2 things!");

        panels.insert(
            (x, y),
            paint_color
                .try_into()
                .expect("Robot should output only 0 or 1"),
        );

        if turn == 0 {
            (dx, dy) = (dy, -dx);
        } else {
            (dx, dy) = (-dy, dx);
        }

        x += dx;
        y += dy;
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let robot1 = Computer::new(input)?;
    let robot2 = robot1.clone();

    let mut panels = HashMap::new();
    run_robot(robot1, &mut panels);
    let part1 = panels.len();

    let mut panels = HashMap::new();
    panels.insert((0, 0), Panel::White);
    run_robot(robot2, &mut panels);

    let (min_x, max_x, min_y, max_y) = panels.keys().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
        },
    );

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut plate = vec![Panel::Black; width * height];
    panels.iter().for_each(|(&(x, y), &color)| {
        plate[(y - min_y) as usize * width + (x - min_x) as usize] = color;
    });

    let mut part2 = String::new();
    for (i, c) in plate.iter().enumerate() {
        if i % width == 0 {
            part2.push('\n');
        }
        part2.push(match c {
            Panel::Black => ' ',
            Panel::White => '#',
        });
    }

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(
    day11_test,
    11,
    input = (
        "2129",
        "
 ###  ####  ##  #  # ###   ##  #### #      
 #  # #    #  # # #  #  # #  #    # #      
 #  # ###  #    ##   #  # #      #  #      
 ###  #    #    # #  ###  # ##  #   #      
 #    #    #  # # #  # #  #  # #    #      
 #    ####  ##  #  # #  #  ### #### ####   "
    )
);
