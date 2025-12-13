use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::{Debug, Display},
    ops::{Div, Index, IndexMut, Mul, Sub},
};

use anyhow::{Context, Result};

use crate::day_test;

struct Machine {
    wanted_lights: u16,
    buttons_bitmask: Vec<u16>,
    equations: Matrix,
    max_button_presses: Vec<i64>,
}

impl Machine {
    fn new(line: &str) -> Result<Self> {
        let mut parts = line.split_ascii_whitespace();

        let light_diagram = parts
            .next()
            .context("Malformed input (light diagram)")?
            .as_bytes();

        let mut wanted_lights = 0;
        for (i, &item) in light_diagram
            .iter()
            .enumerate()
            .take(light_diagram.len() - 1)
            .skip(1)
        {
            wanted_lights |= ((item == b'#') as u16) << (i - 1);
        }

        let mut buttons_bitmask = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        for part in parts {
            let is_button = part.chars().nth(0).context("Malformed input")? == '(';
            let part = &part[1..part.len() - 1];
            let digits = part.split(',').map(|num| num.parse::<u16>());

            if is_button {
                let mut bitmask = 0;
                let mut button = Vec::new();
                for digit in digits {
                    let digit = digit.context("Malformed input (button number)")?;
                    bitmask |= 1 << digit;
                    button.push(digit);
                }
                buttons.push(button);
                buttons_bitmask.push(bitmask);
            } else {
                for digit in digits {
                    joltages.push(digit? as i64);
                }
            }
        }

        let mut max_button_presses = Vec::new();
        for button in &buttons {
            let mut max_presses = i64::MAX;
            for joltage_idx in button {
                max_presses = max_presses.min(joltages[*joltage_idx as usize]);
            }
            max_button_presses.push(max_presses);
        }

        let mut matrix = Matrix::new(joltages.len(), buttons.len() + 1);
        for (i, joltage) in joltages.iter().enumerate() {
            matrix[i][buttons.len()] = Fraction::from_int(*joltage);
        }

        for (i, button) in buttons.iter().enumerate() {
            for row in button {
                matrix[*row as usize][i] = Fraction::from_int(1);
            }
        }
        matrix.reduce_to_rref();

        Ok(Self {
            wanted_lights,
            buttons_bitmask,
            equations: matrix,
            max_button_presses,
        })
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

#[derive(Debug, Clone, Copy, Eq)]
struct Fraction {
    num: i64,
    den: i64,
}

impl Fraction {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "Denominator cannot be zero");

        let mut num = num;
        let mut den = den;

        // Keep denominator positive
        if den < 0 {
            num = -num;
            den = -den;
        }

        let g = gcd(num.abs(), den);
        Fraction {
            num: num / g,
            den: den / g,
        }
    }

    fn from_int(num: i64) -> Self {
        Self::new(num, 1)
    }

    fn zero() -> Self {
        Self::from_int(0)
    }

    fn one() -> Self {
        Self::from_int(1)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.num * other.den).partial_cmp(&(other.num * self.den))
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        assert!(rhs.num != 0, "Division by zero");
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

struct Matrix {
    data: Vec<Fraction>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![Fraction::zero(); rows * cols],
        }
    }

    fn reduce_to_rref(&mut self) {
        let mut lead = 0;

        // println!("BEFORE:\n{self}");
        'outer: for r in 0..self.rows {
            if self.cols <= lead {
                break 'outer;
            }

            let mut i = r;

            while self[i][lead] == Fraction::zero() {
                i += 1;

                if i == self.rows {
                    i = r;
                    lead += 1;

                    if self.cols == lead {
                        break 'outer;
                    }
                }
            }

            self.swap_rows(i, r);
            if self[r][lead] != Fraction::zero() {
                let div = self[r][lead];

                for c in 0..self.cols {
                    self[r][c] = self[r][c] / div;
                }
            }

            for i in 0..self.rows {
                if i != r {
                    let mult = self[i][lead];
                    for c in 0..self.cols {
                        let res = self[i][c] - self[r][c] * mult;
                        self[i][c] = res;
                    }
                }
            }
            lead += 1;
        }
        // println!("AFTER:\n{self}");
    }

    fn swap_rows(&mut self, idx1: usize, idx2: usize) {
        for x in 0..self.cols {
            self.data.swap(idx1 * self.cols + x, idx2 * self.cols + x);
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [Fraction];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{:>4} ", format!("{}", self[y][x]))?;

                if x == self.cols - 2 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(machine: &Machine) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let mut seen = HashSet::new();
    while let Some((num, light)) = queue.pop_front() {
        if light == machine.wanted_lights {
            return num;
        }

        for button in &machine.buttons_bitmask {
            if seen.insert(light ^ button) {
                queue.push_back((num + 1, light ^ button));
            }
        }
    }

    unreachable!();
}

fn part2(machine: &Machine) -> i64 {
    let matrix = &machine.equations;
    let r = matrix.rows;
    let c = matrix.cols - 1;

    #[allow(non_snake_case)]
    let mut A = vec![vec![Fraction::zero(); c]; r];
    let mut b = vec![Fraction::zero(); r];

    for (i, row) in A.iter_mut().enumerate().take(r) {
        row[..c].copy_from_slice(&matrix[i][..c]);
        b[i] = matrix[i][c];
    }

    let mut pivots = vec![None; c];
    for (i, row) in A.iter_mut().enumerate().take(r) {
        for j in 0..c {
            if row[j] == Fraction::one() && (0..j).all(|k| row[k] == Fraction::zero()) {
                pivots[j] = Some(i);
                break;
            }
        }
    }

    let basic: Vec<usize> = (0..c).filter(|&j| pivots[j].is_some()).collect();
    let free: Vec<usize> = (0..c).filter(|&j| pivots[j].is_none()).collect();

    if free.is_empty() {
        return b.iter().map(|f| f.num).sum();
    }

    let free_ranges: Vec<(i64, i64)> = free
        .iter()
        .map(|&button| (0, machine.max_button_presses[button]))
        .collect();

    let compute_x = |free_vals: &Vec<Fraction>| -> Vec<Fraction> {
        let mut x = vec![Fraction::zero(); c];
        for (&col, &val) in free.iter().zip(free_vals.iter()) {
            x[col] = val;
        }
        for &col in &basic {
            let row = pivots[col].expect("basic has only idx where pivot is_some");
            let mut rhs = b[row];
            for (idx, &f) in free.iter().enumerate() {
                rhs = rhs - A[row][f] * free_vals[idx];
            }
            x[col] = rhs;
        }

        x
    };

    let mut best_sum = i64::MAX;
    let mut best_x = vec![];

    fn search(
        idx: usize,
        free: &Vec<usize>,
        free_ranges: &Vec<(i64, i64)>,
        chosen: &mut Vec<Fraction>,
        compute_x: &dyn Fn(&Vec<Fraction>) -> Vec<Fraction>,
        best_sum: &mut i64,
        best_x: &mut Vec<Fraction>,
    ) {
        if idx == free.len() {
            let x = compute_x(chosen);
            if x.iter().all(|&v| v >= Fraction::zero() && v.den == 1) {
                let s: i64 = x.iter().map(|f| f.num).sum();
                if s < *best_sum {
                    *best_sum = s;
                    *best_x = x;
                }
            }
            return;
        }

        let (lo, hi) = free_ranges[idx];

        for t in lo..=hi {
            chosen[idx] = Fraction::from_int(t);
            search(
                idx + 1,
                free,
                free_ranges,
                chosen,
                compute_x,
                best_sum,
                best_x,
            );
        }
    }

    let mut chosen = vec![Fraction::zero(); free.len()];
    search(
        0,
        &free,
        &free_ranges,
        &mut chosen,
        &compute_x,
        &mut best_sum,
        &mut best_x,
    );

    best_sum
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine:")?;
        writeln!(f, "Wanted: {:016b}", self.wanted_lights)?;
        write!(f, "Buttons: ")?;
        for button in &self.buttons_bitmask {
            write!(f, "{:016b} ", button)?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let machines = input
        .lines()
        .map(Machine::new)
        .collect::<Result<Vec<_>>>()?;

    let count1: u32 = machines.iter().map(part1).sum();
    let count2: i64 = machines.iter().map(part2).sum();

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day10_test,
    10,
    example = ("7", "33"),
    input = ("455", "16978")
);
