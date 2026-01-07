use divan::Bencher;
use std::fs;

macro_rules! bench_day {
    ($day:ident) => {
        use aoc_2019::solutions::$day;

        #[divan::bench]
        fn $day(b: Bencher) {
            let input = fs::read_to_string(concat!("inputs/", stringify!($day), ".txt"))
                .expect("Failed to read input file");
            b.bench(|| {
                $day::solve(&input.trim()).expect("solve failed");
            });
        }
    };
}

fn main() {
    divan::main();
}

bench_day!(day01);
bench_day!(day02);
bench_day!(day03);
bench_day!(day04);
bench_day!(day05);
bench_day!(day06);
bench_day!(day07);
bench_day!(day08);
bench_day!(day09);
