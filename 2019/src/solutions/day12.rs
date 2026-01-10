use anyhow::{Context, Result};

use crate::day_test;

#[derive(Debug, Clone)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    pub fn new(line: &str) -> Result<Self> {
        let coords: Vec<i32> = line
            .trim_matches(['<', '>'])
            .split(", ")
            .map(|part| {
                part[2..]
                    .parse::<i32>()
                    .with_context(|| format!("invalid coordinate: {}", part))
            })
            .collect::<Result<_>>()?;

        let [x, y, z] = coords[..] else {
            anyhow::bail!("expected 3 coordinates, got {}", coords.len());
        };

        Ok(Self {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        })
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        let dx = get_gravity(self.x, other.x);
        let dy = get_gravity(self.y, other.y);
        let dz = get_gravity(self.z, other.z);

        self.vx += dx;
        self.vy += dy;
        self.vz += dz;

        other.vx -= dx;
        other.vy -= dy;
        other.vz -= dz;
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn energy(&self) -> u32 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.vx.abs() + self.vy.abs() + self.vz.abs();
        (kinetic * potential) as u32
    }
}

fn get_gravity(a: i32, b: i32) -> i32 {
    if a > b {
        -1
    } else if a < b {
        1
    } else {
        0
    }
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let (left, right) = moons.split_at_mut(i + 1);
        let moon1 = &mut left[i];
        for moon2 in right {
            moon1.apply_gravity(moon2);
        }
    }

    for moon in moons {
        moon.apply_velocity();
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut moons = input.lines().map(Moon::new).collect::<Result<Vec<_>>>()?;

    let mut loop_x = None;
    let mut loop_y = None;
    let mut loop_z = None;

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0.. {
        step(&mut moons);

        if i == 999 {
            part1 = moons.iter().map(Moon::energy).sum();
        }

        loop_x = loop_x.or(moons.iter().all(|m| m.vx == 0).then_some(i + 1));
        loop_y = loop_y.or(moons.iter().all(|m| m.vy == 0).then_some(i + 1));
        loop_z = loop_z.or(moons.iter().all(|m| m.vz == 0).then_some(i + 1));

        if let (Some(x), Some(y), Some(z)) = (loop_x, loop_y, loop_z) {
            part2 = 2 * lcm(&[x, y, z]);
            break;
        }
    }

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day12_test, 12, input = ("7471", "376243355967784"));
