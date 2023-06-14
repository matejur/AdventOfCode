pub fn solve() {
    let coords = (2981, 3075);
    println!("Solving day 25");
    println!("Part1: {}", part1(coords));
    println!("Part2: {}", part2());
}

fn part1(coords: (i32, i32)) -> String {
    let mut num: i64 = 20151125;
    let mut x = 1;
    let mut y = 1;

    loop {
        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            x += 1;
            y -= 1;
        }

        num = (num * 252533) % 33554393;

        if y == coords.0 && x == coords.1 {
            return num.to_string();
        }
    }
}

fn part2() -> String {
    "FREEBIE".to_string()
}
