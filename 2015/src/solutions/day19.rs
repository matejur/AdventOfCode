use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("../../inputs/in19.txt");
    println!("Solving day 19");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut molecule_set = HashSet::new();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let replacements_str = parts[0].split("\n").collect::<Vec<&str>>();
    let molecule = parts[1];

    for repl in replacements_str {
        let parts = repl.split(" => ").collect::<Vec<&str>>();
        let from = parts[0];
        let to = parts[1];

        let positions = molecule
            .match_indices(from)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        for pos in positions {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(pos..pos + from.len(), to);
            molecule_set.insert(new_molecule);
        }
    }

    molecule_set.len().to_string()
}

fn part2(input: &str) -> String {
    let mut all_replacements = HashMap::new();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let replacements_str = parts[0].split("\n").collect::<Vec<&str>>();
    let mut molecule = parts[1].trim().to_string();

    for repl in replacements_str {
        let parts = repl.split(" => ").collect::<Vec<&str>>();
        let from = parts[1];
        let to = parts[0];

        all_replacements.insert(from, to);
    }

    // I know, that this algorithm doesn't find the shortest sequence for every possible input
    let mut count = 0;
    let mut replacements = all_replacements.clone();
    while molecule != "e" {
        let longest_replacement = replacements.iter().max_by(|a, b| a.0.len().cmp(&b.0.len()));

        match longest_replacement {
            Some((from, to)) => {
                let from = from.clone();
                let to = to.clone();
                let position = molecule
                    .match_indices(from)
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();

                // To satisfy the tests :)
                if to == "e" && molecule.len() > from.len() {
                    replacements.remove(from);
                    continue;
                }

                if position.len() > 0 {
                    let pos = position[0];
                    molecule.replace_range(pos..pos + from.len(), to);

                    count += 1;
                }
                replacements.remove(from);
            }
            None => {
                replacements = all_replacements.clone();
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "H => HO
H => OH
O => HH
e => H
e => O

HOHOHO";

    #[test]
    fn day19_part1() {
        assert_eq!(part1(INPUT), "7");
    }

    #[test]
    fn day19_part2() {
        assert_eq!(part2(INPUT), "6");
    }
}
