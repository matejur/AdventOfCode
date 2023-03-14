use std::{collections::HashMap, cmp::max};

use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../../inputs/in13.txt");
    println!("Solving day 13");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn check_map(name: &String, name_map: &mut HashMap<String, usize>, happines_matrix: &mut Vec<Vec<i32>>) {
    if !name_map.contains_key(name) {
        name_map.insert(name.clone(), name_map.len());

        for row in &mut *happines_matrix {
            row.push(0);
        }
        happines_matrix.push(vec![0; name_map.len()]);
    }
}

fn get_happines_matrix(input: &str) -> Vec<Vec<i32>> {
    let mut name_map = HashMap::new();
    let mut happines_matrix = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first_person = parts[0].to_string();
        let second_person = parts[10].to_string().replace(".", "");
        let change = parts[2];
        let mut amount = parts[3].parse().expect("Wrong input!");

        check_map(&first_person, &mut name_map, &mut happines_matrix);
        check_map(&second_person, &mut name_map, &mut happines_matrix);

        if change == "lose" {
            amount *= -1;
        }

        let first_person = name_map.get(&first_person).unwrap();
        let second_person = name_map.get(&second_person).unwrap();

        happines_matrix[*first_person][*second_person] = amount;
    }

    happines_matrix
}

fn calculate_happiness(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let order = 0..n;
    let mut max_happines = 0;

    for conf in order.permutations(n) {
        let mut happines = 0;

        for pair in conf.windows(2) {
            let first = *pair.first().unwrap();
            let second = *pair.last().unwrap();
            happines += matrix[first][second];
            happines += matrix[second][first];
        }

        let first = *conf.first().unwrap();
        let last = *conf.last().unwrap();
        happines += matrix[first][last];
        happines += matrix[last][first];

        max_happines = max(happines, max_happines);
    }

    max_happines
}

fn part1(input: &str) -> String {
    let matrix = get_happines_matrix(input);
    let happines = calculate_happiness(matrix);
    happines.to_string()
}

fn part2(input: &str) -> String {
    let mut matrix = get_happines_matrix(input);

    for row in &mut *matrix {
        row.push(0);
    }
    matrix.push(vec![0; matrix[0].len()]);

    let happines = calculate_happiness(matrix);
    happines.to_string()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn day13_part1() {
        let input = include_str!("../../inputs/in13ex.txt");
        assert_eq!(part1(input), "330");
    }
}
