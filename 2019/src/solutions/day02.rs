use anyhow::Result;

use crate::{day_test, intcode::Computer};

pub fn solve(input: &str) -> Result<(String, String)> {
    let base = Computer::new(input)?;

    let mut p1 = base.clone().with_noun_verb(12, 2);
    p1.run();
    let part1 = p1.get_result();

    let target = 19_690_720;
    let mut low = 0;
    let mut high = 99 * 100 + 99; // 9999

    while low <= high {
        let mid = (low + high) / 2;

        let noun = mid / 100;
        let verb = mid % 100;

        let mut vm = base.clone().with_noun_verb(noun, verb);
        vm.run();
        let result = vm.get_result();

        if result == target {
            return Ok((part1.to_string(), mid.to_string()));
        } else if result < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    unreachable!()
}

day_test!(day02_test, 2, input = ("5305097", "4925"));
