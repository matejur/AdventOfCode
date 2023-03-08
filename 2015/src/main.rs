use std::env;

mod solutions;

fn main() {
    let mut args = env::args();
    args.next();

    let day: u8 = args
        .next()
        .expect("Specify which day to run!")
        .parse()
        .expect("Input a number!");

    match day {
        1 => solutions::day01::solve(),
        2 => solutions::day02::solve(),
        3 => solutions::day03::solve(),
        4 => solutions::day04::solve(),
        5 => solutions::day05::solve(),
        6 => solutions::day06::solve(),
        7 => solutions::day07::solve(),
        _ => println!("Day not implemented!"),
    }
}
