use std::{collections::HashMap, cmp::min, cmp::max};
use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../../inputs/in09.txt");
    println!("Solving day 9");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn get_distance_matrix(input: &str) -> Vec<Vec<i32>> {
    let mut name_map = HashMap::new();

    let mut idx = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let city1 = parts[0];
        let city2 = parts[2];

        if !name_map.contains_key(city1) {
            name_map.insert(city1, idx);
            idx += 1;
        }

        if !name_map.contains_key(city2) {
            name_map.insert(city2, idx);
            idx += 1;
        }
    }
    
    let mut distances = vec![vec![0; idx]; idx];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let city1 = parts[0];
        let city2 = parts[2];
        let distance: i32 = parts[4].parse().expect("Wrong input!");

        let idx1 = *name_map.get(city1).unwrap();
        let idx2 = *name_map.get(city2).unwrap();

        distances[idx1][idx2] = distance;
        distances[idx2][idx1] = distance;
    }

    distances
}

fn get_distance(distances: Vec<Vec<i32>>, f: &dyn Fn(i32, i32) -> i32, mut result: i32) -> i32 {
    let n = distances.len();
    let cities = 0..n;

    for path in cities.permutations(n) {
        let mut length = 0;

        for pair in path.windows(2) {
            let c1 = pair[0];
            let c2 = pair[1];
            length += distances[c1][c2];
        }
        result = f(length, result);
    }

    result
}

fn part1(input: &str) -> String {
    let distances = get_distance_matrix(input);
    let min_length = get_distance(distances, &min, i32::MAX);
    min_length.to_string()
}

fn part2(input: &str) -> String {
    let distances = get_distance_matrix(input);
    let max_length = get_distance(distances, &max, i32::MIN);
    max_length.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn day09_part1() {
        assert_eq!(part1(INPUT), "605");
    }

    #[test]
    fn day09_part2() {
        assert_eq!(part2(INPUT), "982");
    }
}
