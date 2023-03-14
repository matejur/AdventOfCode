use std::cmp::max;

pub fn solve() {
    let input = include_str!("../../inputs/in15.txt");
    println!("Solving day 15");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn calculate_score(p: Vec<Vec<i32>>, part2: bool) -> i32 {
    let mut best_score = 0;

    for a in 0..100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                let d = 100 - a - b - c;

                let cal = a * p[0][4] + b * p[1][4] + c * p[2][4] + d * p[3][4];

                if part2 && cal != 500 {
                    continue;
                }

                let cap = a * p[0][0] + b * p[1][0] + c * p[2][0] + d * p[3][0];
                let dur = a * p[0][1] + b * p[1][1] + c * p[2][1] + d * p[3][1];
                let fla = a * p[0][2] + b * p[1][2] + c * p[2][2] + d * p[3][2];
                let tex = a * p[0][3] + b * p[1][3] + c * p[2][3] + d * p[3][3];

                if cap > 0 && dur > 0 && fla > 0 && tex > 0 {
                    best_score = max(best_score, cap * dur * fla * tex);
                }
            }
        }
    }

    best_score
}

fn part1(input: &str) -> String {
    let mut properties = vec![];

    for ingredient in input.lines() {
        let parts: Vec<&str> = ingredient.split_whitespace().collect();
        
        let capacity = parts[2].replace(",", "").parse().expect("Wrong input!");
        let durability = parts[4].replace(",", "").parse().expect("Wrong input!");
        let flavor = parts[6].replace(",", "").parse().expect("Wrong input!");
        let texture = parts[8].replace(",", "").parse().expect("Wrong input!");
        let calories = parts[10].replace(",", "").parse().expect("Wrong input!");

        properties.push(vec![capacity, durability, flavor, texture, calories]);
    }

    calculate_score(properties, false).to_string()
}

fn part2(input: &str) -> String {
    let mut properties = vec![];

    for ingredient in input.lines() {
        let parts: Vec<&str> = ingredient.split_whitespace().collect();
        
        let capacity = parts[2].replace(",", "").parse().expect("Wrong input!");
        let durability = parts[4].replace(",", "").parse().expect("Wrong input!");
        let flavor = parts[6].replace(",", "").parse().expect("Wrong input!");
        let texture = parts[8].replace(",", "").parse().expect("Wrong input!");
        let calories = parts[10].replace(",", "").parse().expect("Wrong input!");

        properties.push(vec![capacity, durability, flavor, texture, calories]);
    }

    calculate_score(properties, true).to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day15_part1() {
        let input = include_str!("../../inputs/in15ex.txt");
        assert_eq!(part1(input), "62842880");
    }

    #[test]
    fn day15_part2() {
        let input = include_str!("../../inputs/in15ex.txt");
        assert_eq!(part2(input), "57600000");
    }
}
