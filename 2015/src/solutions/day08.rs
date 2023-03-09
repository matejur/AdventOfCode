pub fn solve() {
    let input = include_str!("../../inputs/in08.txt");
    println!("Solving day 8");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn count(str: &str) -> usize {
    let code_chars = str.chars().count();
    let mut memory_chars = code_chars - 2;

    let mut escaped = false;
    for char in str.chars() {
        if escaped {
            match char {
                '\\' | '"' => memory_chars -= 1,
                'x' => memory_chars -= 3,
                _ => ()
            }
            escaped = false;
            continue;
        }
        if char == '\\' {
            escaped = true;
        }
    }

    code_chars - memory_chars
}

fn encode(str: &str) -> usize {
    let code_chars = str.chars().count();
    let mut encoded_chars = code_chars + 2;

    for char in str.chars() {
        match char {
            '\\' | '"' => encoded_chars += 1,
            _ => (),
        }
    };

    encoded_chars - code_chars
}

fn part1(input: &str) -> String {
    let mut result = 0;

    for line in input.lines() {
        result += count(line);
    }

    result.to_string()
}

fn part2(input: &str) -> String {
    let mut result = 0;

    for line in input.lines() {
        result += encode(line);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day08_part1() {
        let input = include_str!("../../inputs/in08ex.txt");
        assert_eq!(part1(input), "12");
    }

    #[test]
    fn day08_part2() {
        let input = include_str!("../../inputs/in08ex.txt");
        assert_eq!(part2(input), "19");
    }
}
