pub fn solve() {
    let input = include_str!("../../inputs/in16.txt");
    println!("Solving day 16");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

const COMPOUNDS: [&str; 10] = [
    "children",
    "cats",
    "samoyeds",
    "pomeranians",
    "akitas",
    "vizslas",
    "goldfish",
    "trees",
    "cars",
    "perfumes",
];

const CATS_INDEX: usize = 1;
const TREES_INDEX: usize = 7;
const POMERANIANS_INDEX: usize = 3;
const GOLDFISH_INDEX: usize = 6;

const SUE: [i32; 10] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

fn add_compound(aunt: &mut [i32; 10], compound: &str, amount: &str) {
    let compound = compound.replace(":", "");
    let amount = amount.replace(",", "").parse().unwrap();
    for (i, c) in COMPOUNDS.iter().enumerate() {
        if *c == compound {
            aunt[i] = amount;
            break;
        }
    }
}

fn part1_compare(aunt: [i32; 10]) -> i32 {
    let mut result = 0;

    for (i, s) in SUE.iter().enumerate() {
        if *s == aunt[i] {
            result += 1;
        }
    }

    result
}

fn part2_compare(aunt: [i32; 10]) -> i32 {
    let mut result = 0;

    for (i, s) in SUE.iter().enumerate() {
        if i == CATS_INDEX || i == TREES_INDEX {
            if *s < aunt[i] {
                result += 1;
            }
            continue;
        }

        if i == POMERANIANS_INDEX || i == GOLDFISH_INDEX {
            if *s > aunt[i] {
                result += 1;
            }
            continue;
        }

        if *s == aunt[i] {
            result += 1;
        }
    }

    result
}

fn get_aunt_sue(input: &str, counter_fn: &dyn Fn([i32; 10]) -> i32) -> usize {
    let mut most_same = 0;
    let mut sue_number = 0;

    for (i, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut curr_aunt = [0; 10];

        add_compound(&mut curr_aunt, parts[2], parts[3]);
        add_compound(&mut curr_aunt, parts[4], parts[5]);
        add_compound(&mut curr_aunt, parts[6], parts[7]);

        let same_compounds = counter_fn(curr_aunt);

        if same_compounds >= most_same {
            most_same = same_compounds;
            sue_number = i + 1;
        }
    }

    sue_number
}

fn part1(input: &str) -> String {
    get_aunt_sue(input, &part1_compare).to_string()
}

fn part2(input: &str) -> String {
    get_aunt_sue(input, &part2_compare).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn day16_part1() {}

    #[test]
    fn day16_part2() {}
}
