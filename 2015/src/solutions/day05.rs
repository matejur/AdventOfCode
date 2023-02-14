use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../../inputs/in05.txt");
    println!("Solving day 5");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn is_nice1(line: &str) -> bool {
    let vowels = line
        .chars()
        .fold(0, |acc, c| if "aeiou".contains(c) { acc + 1 } else { acc });

    if vowels < 3 {
        return false;
    }

    let mut pairs = line.as_bytes().windows(2).filter(|pair| pair[0] == pair[1]);

    if let None = pairs.next() {
        return false;
    }

    !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy")
}

fn is_nice2(line: &str) -> bool {
    let mut pairs = HashMap::new();

    let mut nice = false;
    for (i, pair) in line.as_bytes().windows(2).enumerate() {
        let pair = (pair[0], pair[1]);

        if let Some(index) = pairs.get(&pair) {
            if pair.0 != pair.1 || i - index > 1 {
                nice = true;
                break;
            }
        }

        pairs.insert(pair, i);
    }

    if !nice {
        return false;
    }

    for triplet in line.as_bytes().windows(3) {
        if triplet[0] == triplet[2] {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice1(line))
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice2(line))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT1: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    const INPUT2: &str = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

    #[test]
    fn day05_part1() {
        assert_eq!(part1(INPUT1), "2");
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2(INPUT2), "2");
    }
}
