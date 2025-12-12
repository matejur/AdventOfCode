use std::collections::HashMap;

use crate::day_test;
use anyhow::Result;

fn rec<'a>(
    start: &'a str,
    goal: &'a str,
    graph: &'a HashMap<String, Vec<String>>,
    memo: &mut HashMap<(&'a str, &'a str), i64>,
) -> i64 {
    if let Some(res) = memo.get(&(start, goal)) {
        return *res;
    }
    if start == goal {
        return 1;
    }

    let res = if let Some(connections) = graph.get(start) {
        connections
            .iter()
            .map(|conn| rec(conn, goal, graph, memo))
            .sum()
    } else {
        0
    };

    memo.insert((start, goal), res);
    res
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let graph: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| {
            let (start, rest) = line.split_once(": ").expect("Malformed input");
            (
                start.to_string(),
                rest.split_whitespace().map(str::to_string).collect(),
            )
        })
        .collect();

    let mut memo = HashMap::new();
    let count1 = rec("you", "out", &graph, &mut memo);

    let count2 = rec("svr", "fft", &graph, &mut memo)
        * rec("fft", "dac", &graph, &mut memo)
        * rec("dac", "out", &graph, &mut memo)
        + rec("svr", "dac", &graph, &mut memo)
            * rec("dac", "fft", &graph, &mut memo)
            * rec("fft", "out", &graph, &mut memo);

    Ok((count1.to_string(), count2.to_string()))
}

day_test!(
    day11_test,
    11,
    example = ("0", "2"), // Each part has it's own example, I am testing against part 2
    input = ("733", "290219757077250")
);
