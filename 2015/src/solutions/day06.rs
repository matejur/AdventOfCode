use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../../inputs/in06.txt");
    println!("Solving day 6");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

#[derive(Debug)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split([' ', ',']).collect();

        let a: usize;
        let b: usize;
        let c: usize;
        let d: usize;

        parts.reverse();
        d = parts[0].parse().unwrap();
        c = parts[3].parse().unwrap();
        b = parts[1].parse().unwrap();
        a = parts[4].parse().unwrap();

        if s.starts_with("turn on") {
            Ok(Instruction {
                action: Action::TurnOn,
                a,
                b,
                c,
                d,
            })
        } else if s.starts_with("turn off") {
            Ok(Instruction {
                action: Action::TurnOff,
                a,
                b,
                c,
                d,
            })
        } else {
            Ok(Instruction {
                action: Action::Toggle,
                a,
                b,
                c,
                d,
            })
        }
    }
}

fn part1(input: &str) -> String {
    let mut lights = [false; 1_000_000];

    for line in input.lines() {
        let instr: Instruction = line.parse().unwrap();

        for i in instr.a..=instr.b {
            for j in instr.c..=instr.d {
                let idx: usize = (i * 1000 + j).try_into().unwrap();
                match instr.action {
                    Action::TurnOn => lights[idx] = true,
                    Action::TurnOff => lights[idx] = false,
                    Action::Toggle => lights[idx] = !lights[idx],
                }
            }
        }
    }

    lights.iter().filter(|s| **s).count().to_string()
}

fn part2(input: &str) -> String {
    let mut lights = [0; 1_000_000];

    for line in input.lines() {
        let instr: Instruction = line.parse().unwrap();

        for i in instr.a..=instr.b {
            for j in instr.c..=instr.d {
                let idx: usize = (i * 1000 + j).try_into().unwrap();
                match instr.action {
                    Action::TurnOn => lights[idx] += 1,
                    Action::TurnOff => {
                        if lights[idx] > 0 {
                            lights[idx] -= 1;
                        }
                    }
                    Action::Toggle => lights[idx] += 2,
                }
            }
        }
    }

    lights.iter().sum::<i32>().to_string()
}
