use std::env;

mod solutions;

fn main() {
    let mut args = env::args();
    args.next();

    let input = args.next();

    if let Some(x) = input {
        let day: u8 = x.parse().expect("Input a number or leave it blank");

        match day {
            1 => solutions::day01::solve(),
            2 => solutions::day02::solve(),
            3 => solutions::day03::solve(),
            4 => solutions::day04::solve(),
            5 => solutions::day05::solve(),
            6 => solutions::day06::solve(),
            7 => solutions::day07::solve(),
            8 => solutions::day08::solve(),
            9 => solutions::day09::solve(),
            10 => solutions::day10::solve(),
            11 => solutions::day11::solve(),
            12 => solutions::day12::solve(),
            13 => solutions::day13::solve(),
            14 => solutions::day14::solve(),
            15 => solutions::day15::solve(),
            16 => solutions::day16::solve(),
            17 => solutions::day17::solve(),
            18 => solutions::day18::solve(),
            19 => solutions::day19::solve(),
            20 => solutions::day20::solve(),
            21 => solutions::day21::solve(),
            22 => solutions::day22::solve(),
            23 => solutions::day23::solve(),
            24 => solutions::day24::solve(),
            25 => solutions::day25::solve(),
            _ => println!("Day not implemented!"),
        }
    } else {
        solutions::day01::solve();
        solutions::day02::solve();
        solutions::day03::solve();
        solutions::day04::solve();
        solutions::day05::solve();
        solutions::day06::solve();
        solutions::day07::solve();
        solutions::day08::solve();
        solutions::day09::solve();
        solutions::day10::solve();
        solutions::day11::solve();
        solutions::day12::solve();
        solutions::day13::solve();
        solutions::day14::solve();
        solutions::day15::solve();
        solutions::day16::solve();
        solutions::day17::solve();
        solutions::day18::solve();
        solutions::day19::solve();
        solutions::day20::solve();
        solutions::day21::solve();
        solutions::day22::solve();
        solutions::day23::solve();
        solutions::day24::solve();
        solutions::day25::solve();
    }
}
