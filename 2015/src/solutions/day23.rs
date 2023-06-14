use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../../inputs/in23.txt");
    println!("Solving day 23");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

#[derive(Debug)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        let instr = parts[0];
        let param = parts[1];

        if parts.len() > 2 {
            let offset: i32 = parts[2].parse().unwrap();
            let param = &param[0..param.len() - 1];

            return match instr {
                "jie" => Ok(Instruction::Jie(param.parse().unwrap(), offset)),
                "jio" => Ok(Instruction::Jio(param.parse().unwrap(), offset)),
                _ => panic!("Wrong input"),
            };
        }

        return match instr {
            "hlf" => Ok(Instruction::Hlf(param.parse().unwrap())),
            "tpl" => Ok(Instruction::Tpl(param.parse().unwrap())),
            "inc" => Ok(Instruction::Inc(param.parse().unwrap())),
            "jmp" => Ok(Instruction::Jmp(param.parse().unwrap())),
            _ => panic!("Wrong input"),
        };
    }
}

fn part1(input: &str) -> String {
    let instructions: Vec<Instruction> = input
        .trim()
        .split("\n")
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut pc: i32 = 0;
    let mut reg_a = 0;
    let mut reg_b = 0;

    while pc < instructions.len().try_into().unwrap() {
        let instr = &instructions[pc as usize];

        match instr {
            Instruction::Hlf(reg) => {
                if *reg == 'a' {
                    reg_a /= 2;
                } else {
                    reg_b /= 2;
                }
                pc += 1;
            }
            Instruction::Tpl(reg) => {
                if *reg == 'a' {
                    reg_a *= 3;
                } else {
                    reg_b *= 3;
                }
                pc += 1;
            }
            Instruction::Inc(reg) => {
                if *reg == 'a' {
                    reg_a += 1;
                } else {
                    reg_b += 1;
                }
                pc += 1;
            }
            Instruction::Jmp(offset) => {
                pc += *offset;
            }
            Instruction::Jie(reg, offset) => {
                if *reg == 'a' && reg_a % 2 == 0 {
                    pc += *offset;
                } else if *reg == 'b' && reg_b % 2 == 0 {
                    pc += *offset;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(reg, offset) => {
                if *reg == 'a' && reg_a == 1 {
                    pc += *offset;
                } else if *reg == 'b' && reg_b == 1 {
                    pc += *offset;
                } else {
                    pc += 1;
                }
            }
        }
    }

    reg_b.to_string()
}

fn part2(input: &str) -> String {
    let instructions: Vec<Instruction> = input
        .trim()
        .split("\n")
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut pc: i32 = 0;
    let mut reg_a = 1;
    let mut reg_b = 0;

    while pc < instructions.len().try_into().unwrap() {
        let instr = &instructions[pc as usize];

        match instr {
            Instruction::Hlf(reg) => {
                if *reg == 'a' {
                    reg_a /= 2;
                } else {
                    reg_b /= 2;
                }
                pc += 1;
            }
            Instruction::Tpl(reg) => {
                if *reg == 'a' {
                    reg_a *= 3;
                } else {
                    reg_b *= 3;
                }
                pc += 1;
            }
            Instruction::Inc(reg) => {
                if *reg == 'a' {
                    reg_a += 1;
                } else {
                    reg_b += 1;
                }
                pc += 1;
            }
            Instruction::Jmp(offset) => {
                pc += *offset;
            }
            Instruction::Jie(reg, offset) => {
                if *reg == 'a' && reg_a % 2 == 0 {
                    pc += *offset;
                } else if *reg == 'b' && reg_b % 2 == 0 {
                    pc += *offset;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(reg, offset) => {
                if *reg == 'a' && reg_a == 1 {
                    pc += *offset;
                } else if *reg == 'b' && reg_b == 1 {
                    pc += *offset;
                } else {
                    pc += 1;
                }
            }
        }
    }

    reg_b.to_string()
}
