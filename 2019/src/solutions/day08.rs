use anyhow::Result;
use itertools::Itertools;

use crate::day_test;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
    Transparent,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Black,
            '1' => Self::White,
            '2' => Self::Transparent,
            v => panic!("Invalid color: {v}"),
        }
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut fewest_zeros = u32::MAX;
    let mut part1 = 0u32;
    let mut image = [Color::Transparent; WIDTH * HEIGHT];
    for layer in &input.chars().chunks(WIDTH * HEIGHT) {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;

        for (i, c) in layer.enumerate() {
            match c {
                '0' => zeros += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => (),
            }

            if let Color::Transparent = image[i] {
                image[i] = c.into();
            }
        }

        if zeros < fewest_zeros {
            fewest_zeros = zeros;
            part1 = ones * twos;
        }
    }

    let mut part2 = String::new();
    for (i, pixel) in image.iter().enumerate() {
        if i % WIDTH == 0 {
            part2.push('\n');
        }
        part2.push(match pixel {
            Color::White => '#',
            Color::Black => ' ',
            Color::Transparent => '?',
        });
    }

    Ok((part1.to_string(), part2))
}

day_test!(
    day08_test,
    8,
    input = (
        "1596",
        "
#    ###  ###   ##  #### 
#    #  # #  # #  # #    
#    ###  #  # #    ###  
#    #  # ###  #    #    
#    #  # # #  #  # #    
#### ###  #  #  ##  #### "
    )
);
