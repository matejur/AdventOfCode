use std::cmp::{min, max};

pub fn solve() {
    let input = include_str!("../../inputs/in14.txt");
    println!("Solving day 14");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn get_distance(speed: i32, duration: i32, rest: i32, seconds: i32) -> i32{
    let time = duration + rest;

    let repeats = seconds / time;
    let remainder = min(seconds % time, duration);

    speed * (duration * repeats + remainder)
}

fn get_distance_vector(speed: i32, duration: i32, rest: i32, seconds: i32) -> Vec<i32> {
    let mut dists = vec![];
    for second in 1..=seconds {
        let dist = get_distance(speed, duration, rest, second);
        dists.push(dist);
    }

    dists
}

fn get_points(distances: Vec<Vec<i32>>) -> Vec<i32> {
    let mut points = vec![0; distances.len()];
    for second in 0..distances[0].len() {
        let mut best_dist = 0;
        for deer in 0..distances.len() {
            best_dist = max(best_dist, distances[deer][second]);
        }

        for deer in 0..distances.len() {
            if distances[deer][second] == best_dist {
                points[deer] += 1;
            }
        }
    }

    points
}

fn part1(input: &str) -> String {
    let mut max_distance = 0;

    for deer in input.lines() {
        let parts: Vec<&str> = deer.split_whitespace().collect();

        let speed = parts[3].parse().expect("Wrong input!");
        let duration = parts[6].parse().expect("Wrong input!");
        let rest = parts[13].parse().expect("Wrong input!");

        let distance = get_distance(speed, duration, rest, 2503);
        max_distance = max(max_distance, distance);
    }

    max_distance.to_string()
}

fn part2(input: &str) -> String {
    let seconds: i32 = 2503;
    let mut distances: Vec<Vec<i32>> = vec![];

    for deer in input.lines() {
        let parts: Vec<&str> = deer.split_whitespace().collect();

        let speed = parts[3].parse().expect("Wrong input!");
        let duration = parts[6].parse().expect("Wrong input!");
        let rest = parts[13].parse().expect("Wrong input!");

        distances.push(get_distance_vector(speed, duration, rest, seconds));
    }

    get_points(distances).iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::{get_distance, get_distance_vector, get_points};

    #[test]
    fn day14_part1() {
        assert_eq!(get_distance(14, 10, 127, 1000), 1120);
        assert_eq!(get_distance(16, 11, 162, 1000), 1056);
    }

    #[test]
    fn day14_part2() {
        let mut distances: Vec<Vec<i32>> = vec![];
        distances.push(get_distance_vector(14, 10, 127, 1000));
        distances.push(get_distance_vector(16, 11, 162, 1000));
        let points = get_points(distances);
        assert_eq!(points[0], 312);
        assert_eq!(points[1], 689);
    }
}
