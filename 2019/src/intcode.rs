use std::collections::VecDeque;

use anyhow::{Context, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
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
pub struct Computer {
    memory: Vec<i64>,
    input: VecDeque<i64>,
    ip: usize,
    relative_base: i64,
}

impl Computer {
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
            relative_base: 0,
        })
    }

    pub fn run(&mut self) -> StopReason {
        loop {
            let instruction = self.memory[self.ip];
            let opcode = instruction % 100;

            let mut inst = instruction / 100;
            let m0 = (inst % 10).into();
            inst /= 10;
            let m1 = (inst % 10).into();
            inst /= 10;
            let m2 = (inst % 10).into();

            match opcode {
                1 => {
                    let v = self.load(0, m0) + self.load(1, m1);
                    self.store(2, v, m2);
                    self.ip += 4;
                }
                2 => {
                    let v = self.load(0, m0) * self.load(1, m1);
                    self.store(2, v, m2);
                    self.ip += 4;
                }
                3 => match self.input.pop_front() {
                    Some(value) => {
                        self.store(0, value, m0);
                        self.ip += 2;
                    }
                    None => return StopReason::InputNeeded,
                },
                4 => {
                    let value = self.load(0, m0);
                    self.ip += 2;
                    return StopReason::Output(value);
                }

                5 => {
                    if self.load(0, m0) != 0 {
                        self.ip = self.load(1, m1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.load(0, m0) == 0 {
                        self.ip = self.load(1, m1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let v = (self.load(0, m0) < self.load(1, m1)) as i64;
                    self.store(2, v, m2);
                    self.ip += 4;
                }
                8 => {
                    let v = (self.load(0, m0) == self.load(1, m1)) as i64;
                    self.store(2, v, m2);
                    self.ip += 4;
                }
                9 => {
                    self.relative_base += self.load(0, m0);
                    self.ip += 2;
                }
                99 => return StopReason::Halted,
                op => panic!("Unknown opcode: {op}"),
            }
        }
    }

    fn load(&self, param_idx: usize, mode: ParameterMode) -> i64 {
        let param = self.memory[self.ip + param_idx + 1];
        let addr = match mode {
            ParameterMode::Relative => (self.relative_base + param) as usize,
            ParameterMode::Position => param as usize,
            ParameterMode::Immediate => return param,
        };

        self.memory.get(addr).copied().unwrap_or(0)
    }

    fn store(&mut self, param_idx: usize, value: i64, mode: ParameterMode) {
        let mut addr = self.memory[self.ip + param_idx + 1];

        if let ParameterMode::Relative = mode {
            addr += self.relative_base;
        }

        let addr = addr as usize;

        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }

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

    pub fn run_to_next_output(&mut self) -> Option<i64> {
        match self.run() {
            StopReason::Output(v) => Some(v),
            StopReason::InputNeeded => {
                panic!("VM requested input, but none was provided");
            }
            StopReason::Halted => None,
        }
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_param_mode() -> Result<()> {
        let mut vm = Computer::new("1002,4,3,4,33")?;
        vm.run();
        assert_eq!(vm.memory[4], 99);

        let mut vm = Computer::new("1101,100,-1,4,0")?;
        vm.run();
        assert_eq!(vm.memory[4], 99);

        Ok(())
    }

    #[test]
    fn test_jumps_and_comparisons() -> Result<()> {
        let base = Computer::new(
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

    #[test]
    fn test_relative_mode_quine() -> Result<()> {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut vm = Computer::new(program)?;

        let mut output = Vec::new();
        loop {
            match vm.run() {
                StopReason::Output(v) => output.push(v),
                StopReason::Halted => break,
                StopReason::InputNeeded => panic!("Unexpected input request"),
            }
        }

        let expected: Vec<i64> = program.split(',').map(|s| s.parse().unwrap()).collect();

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_relative_mode_large_number() -> Result<()> {
        let mut vm = Computer::new("1102,34915192,34915192,7,4,7,99,0")?;

        match vm.run() {
            StopReason::Output(v) => {
                assert_eq!(v.to_string().len(), 16);
            }
            _ => panic!("Expected output"),
        }

        Ok(())
    }

    #[test]
    fn test_relative_mode_large_immediate() -> Result<()> {
        let mut vm = Computer::new("104,1125899906842624,99")?;

        match vm.run() {
            StopReason::Output(v) => {
                assert_eq!(v, 1125899906842624);
            }
            _ => panic!("Expected output"),
        }

        Ok(())
    }
}
