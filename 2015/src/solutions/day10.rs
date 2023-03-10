pub fn solve() {
    let input = include_str!("../../inputs/in10.txt");
    println!("Solving day 10");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn step(seq: &str) -> String {
    let mut result = String::new();
    
    let mut iter = seq.chars();

    let mut count = 1;
    let mut curr_char = iter.next().unwrap();

    for char in iter {
        if char == curr_char {
            count += 1;
        } else {
            result += &format!("{count}{curr_char}");
            curr_char = char;
            count = 1;
        }
    }

    result += &format!("{count}{curr_char}");
    result
}

fn part1(input: &str) -> String {
    let mut result = input.to_string();

    for _ in 0..40 {
        result = step(&result);
    }

    result.len().to_string()
}

fn part2(input: &str) -> String {
    let mut result = input.to_string();

    for _ in 0..50 {
        result = step(&result);
    }

    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::step;

    #[test]
    fn day10_part1() {
        assert_eq!(step("1"), "11");
        assert_eq!(step("11"), "21");
        assert_eq!(step("21"), "1211");
        assert_eq!(step("1211"), "111221");
        assert_eq!(step("111221"), "312211");
    }

    #[test]
    #[ignore]
    fn day10_part2() {

    }
}
