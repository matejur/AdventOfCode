use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Display},
    i64,
    iter::Sum,
    ops::{Add, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub, SubAssign},
    u32,
};

use anyhow::{Context, Result};

use crate::day_test;

struct Machine {
    wanted_lights: u16,
    buttons_bitmask: Vec<u16>,
    equations: Matrix,
}

impl Machine {
    fn new(line: &str) -> Result<Self> {
        let mut parts = line.split_ascii_whitespace();

        let light_diagram = parts
            .next()
            .context("Malformed input (light diagram)")?
            .as_bytes();

        let mut wanted_lights = 0;
        for i in 1..light_diagram.len() - 1 {
            wanted_lights |= ((light_diagram[i] == b'#') as u16) << (i - 1);
        }

        let mut buttons_bitmask = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        while let Some(part) = parts.next() {
            let is_button = part.chars().nth(0).context("Malformed input")? == '(';
            let part = &part[1..part.len() - 1];
            let digits = part.split(',').map(|num| num.parse::<u16>());

            if is_button {
                let mut val = 0;
                let mut button = Vec::new();
                for digit in digits {
                    let digit = digit.context("Malformed input (button number)")?;
                    val |= 1 << digit;
                    button.push(digit);
                }
                buttons.push(button);
                buttons_bitmask.push(val);
            } else {
                for digit in digits {
                    joltages.push(digit?);
                }
            }
        }

        let mut matrix = Matrix::new(joltages.len(), buttons.len() + 1);
        for (i, joltage) in joltages.iter().enumerate() {
            matrix[i][buttons.len()] = Fraction::from_int(*joltage as i64);
        }

        for (i, button) in buttons.iter().enumerate() {
            for row in button {
                matrix[*row as usize][i] = Fraction::from_int(1);
            }
        }
        matrix.to_kinda_rref();

        Ok(Self {
            wanted_lights,
            buttons_bitmask: buttons_bitmask,
            equations: matrix,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction {
    num: i64,
    denom: i64,
}

impl Default for Fraction {
    fn default() -> Self {
        Self { num: 0, denom: 1 }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl Fraction {
    pub fn new(num: i64, denom: i64) -> Self {
        assert!(denom != 0, "Denominator cannot be zero");
        let mut frac = Fraction { num, denom };
        frac.simplify();
        frac
    }

    pub fn from_int(num: i64) -> Self {
        Fraction::new(num, 1)
    }

    fn simplify(&mut self) {
        let gcd = gcd(self.num.abs(), self.denom.abs());
        self.num /= gcd;
        self.denom /= gcd;
        if self.denom < 0 {
            self.num = -self.num;
            self.denom = -self.denom;
        }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_one(&self) -> bool {
        self.num == self.denom
    }
}

impl Sub for Fraction {
    type Output = Fraction;
    fn sub(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.denom - other.num * self.denom,
            self.denom * other.denom,
        )
    }
}
impl Add for Fraction {
    type Output = Fraction;
    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.denom + other.num * self.denom,
            self.denom * other.denom,
        )
    }
}
impl Sum for Fraction {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fraction::from_int(0), |acc, x| acc + x)
    }
}
impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction::new(-self.num, self.denom)
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Fraction) {
        *self = *self - other;
    }
}

impl Mul for Fraction {
    type Output = Fraction;
    fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.denom * other.denom)
    }
}

impl Div for Fraction {
    type Output = Fraction;
    fn div(self, other: Fraction) -> Fraction {
        assert!(other.num != 0, "Cannot divide by zero");
        Fraction::new(self.num * other.denom, self.denom * other.num)
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Fraction) {
        *self = *self / other;
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num * other.denom).partial_cmp(&(other.num * self.denom))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.denom).cmp(&(other.num * self.denom))
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
            data: vec![Fraction::default(); rows * cols],
        }
    }

    fn to_kinda_rref(&mut self) {
        let mut lead = 0;

        // println!("BEFORE:\n{self}");
        'outer: for r in 0..self.rows {
            if self.cols <= lead {
                break 'outer;
            }

            let mut i = r;

            while self[i][lead].is_zero() {
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
            if !self[r][lead].is_zero() {
                let div = self[r][lead];
                for c in 0..self.cols {
                    self[r][c] /= div;
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
    let m = matrix.rows;
    let n = matrix.cols - 1;

    #[allow(non_snake_case)]
    let mut A = vec![vec![Fraction::default(); n]; m];
    let mut b = vec![Fraction::default(); m];

    for i in 0..m {
        for j in 0..n {
            A[i][j] = matrix[i][j];
        }
        b[i] = matrix[i][n];
    }

    let mut pivot_of = vec![None; n]; // pivot_of[j] = Some(row)
    for i in 0..m {
        for j in 0..n {
            if A[i][j].is_one() {
                let zero_left = (0..j).all(|k| A[i][k].is_zero());
                if zero_left {
                    pivot_of[j] = Some(i);
                    break;
                }
            }
        }
    }

    let basic: Vec<usize> = (0..n).filter(|&j| pivot_of[j].is_some()).collect();
    let free: Vec<usize> = (0..n).filter(|&j| pivot_of[j].is_none()).collect();

    if free.is_empty() {
        return b.iter().map(|f| f.num).sum();
    }

    let mut free_ranges: Vec<(Fraction, Fraction)> =
        vec![(Fraction::from_int(0), Fraction::from_int(300)); free.len()];

    println!("{matrix}");
    dbg!(&free_ranges);

    for (idx_f, &f) in free.iter().enumerate() {
        let mut low = Fraction::from_int(0);
        let mut high = Fraction::from_int(300);
        for &col in &basic {
            let row = pivot_of[col].unwrap();
            let coeff = A[row][f];
            if coeff.is_zero() {
                continue;
            }
            let rhs = b[row];
            if coeff > Fraction::from_int(0) {
                let val = rhs / coeff;
                if val < high {
                    high = val;
                }
            } else {
                let val = (-rhs) / (-coeff);
                if val > low {
                    low = val;
                }
            }
        }
        free_ranges[idx_f] = (low, high);
    }

    let compute_x = |free_vals: &Vec<Fraction>| -> Vec<Fraction> {
        let mut x = vec![Fraction::from_int(0); n];
        for (val, &col) in free_vals.iter().zip(free.iter()) {
            x[col] = *val;
        }
        for &col in &basic {
            let row = pivot_of[col].unwrap();
            let mut rhs = b[row];
            for (idx_f, &f) in free.iter().enumerate() {
                rhs -= A[row][f] * free_vals[idx_f];
            }
            x[col] = rhs;
        }
        x
    };

    // Recursive search
    fn search(
        idx: usize,
        free: &Vec<usize>,
        free_ranges: &Vec<(Fraction, Fraction)>,
        chosen: &mut Vec<Fraction>,
        compute_x: &dyn Fn(&Vec<Fraction>) -> Vec<Fraction>,
        best_sum: &mut Fraction,
        best_x: &mut Vec<Fraction>,
    ) {
        if idx == free.len() {
            let x = compute_x(chosen);
            if x.iter().all(|&v| v >= Fraction::from_int(0)) {
                let s: Fraction = x.iter().copied().sum();
                if s < *best_sum {
                    *best_sum = s;
                    *best_x = x;
                }
            }
            return;
        }

        let (lo, hi) = free_ranges[idx];
        // Simple integer step (assuming fractions represent integer values)
        let start = lo.num / lo.denom;
        let end = hi.num / hi.denom;
        for t in start..=end {
            chosen[idx] = Fraction::new(t, 1);
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

    let mut chosen = vec![Fraction::from_int(0); free.len()];
    let mut best_sum = Fraction::new(i64::MAX, 1);
    let mut best_x = vec![];
    search(
        0,
        &free,
        &free_ranges,
        &mut chosen,
        &compute_x,
        &mut best_sum,
        &mut best_x,
    );

    dbg!(best_sum).num
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

day_test!(day10_test, 10, example = ("7", ""), input = ("455", ""));
