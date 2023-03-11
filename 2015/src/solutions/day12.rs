use regex::Regex;
use serde_json::Value;

pub fn solve() {
    let input = include_str!("../../inputs/in12.txt");
    println!("Solving day 12");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let number_regex = Regex::new(r"-?\d+").unwrap();

    let sum: i32 = number_regex.find_iter(input)
                    .map(|num| num.as_str().parse::<i32>().unwrap())
                    .sum();

    sum.to_string()
}

fn json_sum(json: Value) -> i64 {
    let mut sum = 0;

    if let Value::Object(ref o) = json {
        for val in o.values() {
            if let Value::String(s) = val {
                if *s == "red".to_string() {
                    return 0;
                }
            }
        }
    }

    match json {
        Value::Number(n) => sum += n.as_i64().unwrap(),
        Value::Array(arr) => {
            for val in arr {
                sum += json_sum(serde_json::from_value::<Value>(val).unwrap());
            }
        },
        Value::Object(obj) => {
            for val in obj.values() {
                sum += json_sum(serde_json::from_value::<Value>(val.clone()).unwrap());
            }
        },
        _ => (),
    }

    sum
}

fn part2(input: &str) -> String {
    let json: Value = serde_json::from_str(input).unwrap();
    json_sum(json).to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day12_part1() {
        let inp = "[1,2,3]";
        assert_eq!(part1(inp), "6");

        let inp = "{\"a\":2,\"b\":4}";
        assert_eq!(part1(inp), "6");

        let inp = "[[[3]]]";
        assert_eq!(part1(inp), "3");

        let inp = "{\"a\":{\"b\":4},\"c\":-1}";
        assert_eq!(part1(inp), "3");

        let inp = "{\"a\":[-1,1]}";
        assert_eq!(part1(inp), "0");

        let inp = "[-1,{\"a\":1}]";
        assert_eq!(part1(inp), "0");

        let inp = "[]";
        assert_eq!(part1(inp), "0");

        let inp = "{}";
        assert_eq!(part1(inp), "0");
    }

    #[test]
    fn day12_part2() {
        let inp = "[1,2,3]";
        assert_eq!(part2(inp), "6");

        let inp = "[1,{\"c\":\"red\",\"b\":2},3]";
        assert_eq!(part2(inp), "4");

        let inp = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
        assert_eq!(part2(inp), "0");

        let inp = "[1,\"red\",5]";
        assert_eq!(part2(inp), "6");
    }
}
