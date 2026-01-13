use anyhow::Result;

use crate::day_test;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

struct PatternIter {
    base_pattern: &'static [i32],
    repeats: usize,
    curr_idx: usize,
}

impl Iterator for PatternIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let pat_idx = (self.curr_idx / self.repeats) % self.base_pattern.len();
        self.curr_idx += 1;
        Some(self.base_pattern[pat_idx])
    }
}

fn fft(input: &[i32], output: &mut [i32]) {
    for out_idx in 0..input.len() {
        let pattern = PatternIter {
            base_pattern: &BASE_PATTERN,
            curr_idx: 1,
            repeats: out_idx + 1,
        };

        output[out_idx] = 0;
        for (in_idx, mult) in (0..input.len()).zip(pattern) {
            output[out_idx] += input[in_idx] * mult;
        }

        output[out_idx] = output[out_idx].abs() % 10;
    }
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let mut signal = input
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as i32)
        .collect::<Vec<_>>();
    let mut output = vec![0; signal.len()];

    for _ in 0..100 {
        fft(&signal, &mut output);
        std::mem::swap(&mut signal, &mut output);
    }

    let part1 = signal
        .iter()
        .take(8)
        .map(|&c| (c as u8 + b'0') as char)
        .collect();

    Ok((part1, "".to_string()))
}

day_test!(day00_test, 0, example = ("", ""), input = ("", ""));
