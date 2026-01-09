use anyhow::Result;

use crate::{day_test, intcode::Computer};

pub fn solve(input: &str) -> Result<(String, String)> {
    let base = Computer::new(input)?;

    // for _ in 0..10 {
    let mut vm = base.clone();
    vm.push_input(1);
    let part1 = vm.run_to_last_output().expect("Should provide an output");

    let mut vm = base.clone();
    vm.push_input(2);
    let part2 = vm.run_to_last_output().expect("Should provide an output");
    // }
    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day09_test, 9, input = ("2377080455", "74917"));
