use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Intcode {
    memory: Vec<i64>,
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
        })
    }

    pub fn run(&mut self) {
        loop {
            match self.memory[self.ip] {
                1 => self.add(),
                2 => self.mul(),
                99 => break,
                op => panic!("Unknown opcode: {op}"),
            }
        }
    }

    fn add(&mut self) {
        let (a, b, out) = (
            self.memory[self.ip + 1] as usize,
            self.memory[self.ip + 2] as usize,
            self.memory[self.ip + 3] as usize,
        );
        self.memory[out] = self.memory[a] + self.memory[b];
        self.ip += 4;
    }

    fn mul(&mut self) {
        let (a, b, out) = (
            self.memory[self.ip + 1] as usize,
            self.memory[self.ip + 2] as usize,
            self.memory[self.ip + 3] as usize,
        );
        self.memory[out] = self.memory[a] * self.memory[b];
        self.ip += 4;
    }

    pub fn with_noun_verb(mut self, noun: i64, verb: i64) -> Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    pub fn get_result(&self) -> i64 {
        self.memory[0]
    }
}
