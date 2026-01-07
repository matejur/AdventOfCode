use anyhow::Result;
use itertools::Itertools;

use crate::{
    day_test,
    intcode::{Intcode, StopReason},
};

const AMP_COUNT: usize = 5;

pub fn solve(input: &str) -> Result<(String, String)> {
    let base = Intcode::new(input)?;
    let part1 = (0..=4)
        .permutations(AMP_COUNT)
        .map(|settings| {
            let mut signal = 0;
            for setting in settings {
                let mut vm = base.clone();
                vm.push_input(setting);
                vm.push_input(signal);
                signal = vm
                    .run_to_last_output()
                    .expect("VM should produce an output");
            }
            signal
        })
        .max()
        .expect("There should be at least one maximum");

    let part2 = (5..=9)
        .permutations(AMP_COUNT)
        .map(|settings| {
            let mut vms: [Intcode; AMP_COUNT] = core::array::from_fn(|i| {
                let mut vm = base.clone();
                vm.push_input(settings[i]);
                vm
            });

            let mut signal = 0;
            let mut vm_idx = 0;
            loop {
                let vm = &mut vms[vm_idx % vms.len()];

                loop {
                    match vm.run() {
                        StopReason::Halted => return signal,
                        StopReason::Output(s) => {
                            signal = s;
                            break;
                        }
                        StopReason::InputNeeded => {
                            vm.push_input(signal);
                        }
                    }
                }
                vm_idx += 1;
            }
        })
        .max()
        .expect("There should be at least one maximum");

    Ok((part1.to_string(), part2.to_string()))
}

day_test!(day07_test, 7, input = ("20413", "3321777"));
