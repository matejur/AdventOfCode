use std::{collections::HashMap, str::FromStr};
use Gate::*;
use Variable::*;

pub fn solve() {
    let input = include_str!("../../inputs/in07.txt");
    println!("Solving day 7");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug, Clone)]
enum Variable {
    Num(u16),
    Name(String),
}

#[derive(Debug)]
struct ParseVariableError;

impl FromStr for Variable {
    type Err = ParseVariableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(num) => Ok(Self::Num(num)),
            Err(_) => Ok(Self::Name(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
enum Gate {
    AND(Variable, Variable),
    OR(Variable, Variable),
    LSHIFT(Variable, Variable),
    RSHIFT(Variable, Variable),
    NOT(Variable),
    CONST(Variable),
}

#[derive(Debug)]
struct ParseGateError;

impl FromStr for Gate {
    type Err = ParseGateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        match parts.len() {
            1 => Ok(CONST(parts[0].parse().unwrap())),
            2 => Ok(NOT(parts[1].parse().unwrap())),
            3 => {
                let left: Variable = parts[0].parse().unwrap();
                let right: Variable = parts[2].parse().unwrap();

                match parts[1] {
                    "AND" => Ok(AND(left, right)),
                    "OR" => Ok(OR(left, right)),
                    "LSHIFT" => Ok(LSHIFT(left, right)),
                    "RSHIFT" => Ok(RSHIFT(left, right)),
                    _ => panic!("Invalid gate"),
                }
            }
            _ => panic!("Wrong input!"),
        }
    }
}

fn first_second(a: Variable, b: Variable, circuit: &mut HashMap<String, Gate>) -> (u16, u16) {
    let first = match a {
        Num(n) => n,
        Name(n) => solve_circuit(&n, circuit),
    };

    let second = match b {
        Num(n) => n,
        Name(n) => solve_circuit(&n, circuit),
    };

    (first, second)
}

fn solve_circuit(wire: &String, circuit: &mut HashMap<String, Gate>) -> u16 {
    let gate = circuit.get(&*wire).unwrap().clone();

    let value = match gate {
        AND(a, b) => {
            let (a, b) = first_second(a, b, circuit);
            a & b
        }
        OR(a, b) => {
            let (a, b) = first_second(a, b, circuit);
            a | b
        }
        LSHIFT(a, b) => {
            let (a, b) = first_second(a, b, circuit);
            a << b
        }
        RSHIFT(a, b) => {
            let (a, b) = first_second(a, b, circuit);
            a >> b
        }
        NOT(n) => match n {
            Num(n) => !n,
            Name(name) => !solve_circuit(&name, circuit),
        },
        CONST(n) => match n {
            Num(n) => n,
            Name(n) => solve_circuit(&n, circuit),
        },
    };

    circuit.insert(wire.clone(), CONST(Num(value)));

    value
}

fn parse_circuit(input: &str) -> HashMap<String, Gate> {
    let mut circuit = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let wire = parts[1].to_string();
        let gate: Gate = parts[0].parse().unwrap();

        circuit.insert(wire, gate);
    }

    circuit
}

fn part1(input: &str) -> String {
    let mut circuit = parse_circuit(input);
    let result = solve_circuit(&"a".to_string(), &mut circuit);
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut circuit = parse_circuit(input);

    let wire_a = solve_circuit(&"a".to_string(), &mut circuit);

    let mut circuit = parse_circuit(input);
    circuit.insert("b".to_string(), CONST(Num(wire_a)));

    let result = solve_circuit(&"a".to_string(), &mut circuit);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::{parse_circuit, solve_circuit};

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn day07_part1() {
        let mut circuit = parse_circuit(INPUT);

        assert_eq!("72", solve_circuit(&"d".to_string(), &mut circuit).to_string());
        assert_eq!("507", solve_circuit(&"e".to_string(), &mut circuit).to_string());
        assert_eq!("492", solve_circuit(&"f".to_string(), &mut circuit).to_string());
        assert_eq!("114", solve_circuit(&"g".to_string(), &mut circuit).to_string());
        assert_eq!("65412", solve_circuit(&"h".to_string(), &mut circuit).to_string());
        assert_eq!("65079", solve_circuit(&"i".to_string(), &mut circuit).to_string());
    }

    #[test]
    fn day07_part2() {}
}
