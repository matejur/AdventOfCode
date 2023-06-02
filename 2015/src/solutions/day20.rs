pub fn solve() {
    let input = include_str!("../../inputs/in20.txt");
    println!("Solving day 20");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn calculate_house_presents_part1(house: i32) -> i32 {
    let end = (house as f64).sqrt().ceil() as i32;
    let mut presents = 0;

    if end * end == house {
        presents = 10 * end;
    }

    for elf in 1..end {
        if house % elf == 0 {
            presents += 10 * elf;
            presents += 10 * house / elf;
        }
    }

    presents
}

fn evaluate(elf: i32, house: i32) -> i32 {
    if house / elf > 50 {
        0
    } else {
        11 * elf
    }
}

fn calculate_house_presents_part2(house: i32) -> i32 {
    let end = (house as f64).sqrt().ceil() as i32;
    let mut presents = 0;

    if end * end == house {
        presents = evaluate(end, house);
    }

    for elf in 1..end {
        if house % elf == 0 {
            presents += evaluate(elf, house);
            presents += evaluate(house / elf, house);
        }
    }

    presents
}

fn part1(input: &str) -> String {
    let input: i32 = input.parse().unwrap();

    for i in 1..input {
        if calculate_house_presents_part1(i) > input {
            return i.to_string();
        }
    }

    "not_found".to_string()
}

fn part2(input: &str) -> String {
    let input: i32 = input.parse().unwrap();

    for i in 1..input {
        if calculate_house_presents_part2(i) > input {
            return i.to_string();
        }
    }

    "not_found".to_string()
}
