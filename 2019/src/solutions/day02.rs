use anyhow::Result;

use crate::{day_test, intcode::Intcode};

pub fn solve(input: &str) -> Result<(String, String)> {
    let base = Intcode::new(input)?;

    let mut p1 = base.clone().with_noun_verb(12, 2);
    p1.run();
    let part1 = p1.get_result();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut vm = base.clone().with_noun_verb(noun, verb);
            vm.run();
            if vm.get_result() == 19_690_720 {
                return Ok((part1.to_string(), (noun * 100 + verb).to_string()));
            }
        }
    }

    unreachable!()
}

day_test!(day02_test, 2, input = ("5305097", "4925"));
