use std::collections::VecDeque;

use anyhow::{Context, Result};
const MAX_PARAMS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_instruction(inst: i64) -> [ParameterMode; MAX_PARAMS] {
        let mut modes = [ParameterMode::Position; MAX_PARAMS];
        let mut inst = inst / 100;
        for i in 0..MAX_PARAMS {
            modes[i] = (inst % 10).into();
            inst /= 10;
        }

        modes
    }
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            v => panic!("Invalid parameter mode: {v}"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum StopReason {
    Halted,
    Output(i64),
    InputNeeded,
}

#[derive(Clone, Debug)]
pub struct Intcode {
    memory: Vec<i64>,
    input: VecDeque<i64>,
    ip: usize,
}

impl Intcode {
    pub fn new(input: &str) -> Result<Self> {
        Ok(Self {
            memory: input
                .split(',')
                .map(|s| {
                    s.parse::<i64>()
                        .with_context(|| format!("Invalid intcode value: {s}"))
                })
                .collect::<Result<Vec<_>>>()?,
            ip: 0,
            input: VecDeque::new(),
        })
    }

    pub fn run(&mut self) -> StopReason {
        loop {
            let instruction = self.memory[self.ip];
            let opcode = instruction % 100;
            let modes = ParameterMode::from_instruction(instruction);

            match opcode {
                1 => {
                    let v = self.load(0, modes) + self.load(1, modes);
                    self.store(2, v);
                    self.ip += 4;
                }
                2 => {
                    let v = self.load(0, modes) * self.load(1, modes);
                    self.store(2, v);
                    self.ip += 4;
                }
                3 => match self.input.pop_front() {
                    Some(value) => {
                        self.store(0, value);
                        self.ip += 2;
                    }
                    None => return StopReason::InputNeeded,
                },
                4 => {
                    let value = self.load(0, modes);
                    self.ip += 2;
                    return StopReason::Output(value);
                }
                5 => self.jump_if(modes, |v| v != 0),
                6 => self.jump_if(modes, |v| v == 0),
                7 => {
                    let v = (self.load(0, modes) < self.load(1, modes)) as i64;
                    self.store(2, v);
                    self.ip += 4;
                }
                8 => {
                    let v = (self.load(0, modes) == self.load(1, modes)) as i64;
                    self.store(2, v);
                    self.ip += 4;
                }
                99 => return StopReason::Halted,
                op => panic!("Unknown opcode: {op}"),
            }
        }
    }

    fn jump_if<F>(&mut self, modes: [ParameterMode; MAX_PARAMS], cond: F)
    where
        F: Fn(i64) -> bool,
    {
        let test = self.load(0, modes);

        if cond(test) {
            self.ip = self.load(1, modes) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn load(&self, param_idx: usize, modes: [ParameterMode; MAX_PARAMS]) -> i64 {
        let param = self.memory[self.ip + param_idx + 1];
        match modes[param_idx] {
            ParameterMode::Position => self.memory[param as usize],
            ParameterMode::Immediate => param,
        }
    }

    fn store(&mut self, param_idx: usize, value: i64) {
        // Writes always treat the parameter as an address
        let addr = self.memory[self.ip + param_idx + 1] as usize;
        self.memory[addr] = value;
    }

    pub fn push_input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn with_noun_verb(mut self, noun: i64, verb: i64) -> Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    pub fn get_result(&self) -> i64 {
        self.memory[0]
    }

    pub fn run_to_last_output(&mut self) -> Option<i64> {
        let mut last_output = None;

        loop {
            match self.run() {
                StopReason::Output(v) => last_output = Some(v),
                StopReason::InputNeeded => {
                    panic!("VM requested input, but none was provided");
                }
                StopReason::Halted => break,
            }
        }

        last_output
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_param_mode() -> Result<()> {
        let mut vm = Intcode::new("1002,4,3,4,33")?;
        vm.run();
        assert_eq!(vm.memory[4], 99);

        let mut vm = Intcode::new("1101,100,-1,4,0")?;
        vm.run();
        assert_eq!(vm.memory[4], 99);

        Ok(())
    }
    #[test]
    fn test_jumps_and_comparisons() -> Result<()> {
        let base = Intcode::new(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        )?;

        let mut vm = base.clone();
        vm.push_input(7);
        assert_eq!(vm.run(), StopReason::Output(999));

        let mut vm = base.clone();
        vm.push_input(8);
        assert_eq!(vm.run(), StopReason::Output(1000));

        let mut vm = base.clone();
        vm.push_input(9);
        assert_eq!(vm.run(), StopReason::Output(1001));

        Ok(())
    }
}
