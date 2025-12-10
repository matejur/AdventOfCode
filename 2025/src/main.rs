use std::{env, fs, io};

use anyhow::{Context, Result, bail};
use clap::Parser;
use time::OffsetDateTime;

use aoc_2025::solutions::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10};

#[derive(Parser, Debug)]
pub struct Args {
    pub day: Option<u8>,
    #[arg(short, long)]
    pub example: bool,
    #[arg(short, long)]
    pub all: bool,
}

fn download_input(day: u8, path: &str) -> Result<String> {
    let session = env::var("AOC_SESSION").context("Set AOC_SESSION env variable")?;

    let url = format!("https://adventofcode.com/2025/day/{day}/input");

    let client = reqwest::blocking::Client::new();
    let text = client
        .get(&url)
        .header("Cookie", format!("session={session}"))
        .header("User-Agent", "matej.urbas00@gmail.com")
        .send()?
        .text()?;

    fs::create_dir_all("inputs")?;
    fs::write(path, &text)?;

    Ok(text)
}

fn get_input(day: u8) -> Result<String> {
    let path = format!("inputs/day{day:02}.txt");

    if fs::exists(&path)? {
        return fs::read_to_string(&path).context("Failed to read input file");
    };

    download_input(day, &path)
}

fn get_example(day: u8) -> Result<String> {
    let path = format!("examples/day{day:02}.txt");

    if fs::exists(&path)? {
        return fs::read_to_string(&path).context("Failed to read example file");
    };

    println!("Please paste the example:");

    let example = io::read_to_string(io::stdin())?;

    fs::create_dir_all("examples")?;
    fs::write(path, &example)?;

    Ok(example)
}

fn run_day(day: u8, example: bool) -> Result<()> {
    let input = if example {
        get_example(day)?
    } else {
        get_input(day)?
    };
    let input = input.trim();

    let (part1, part2) = match day {
        1 => day01::solve(input)?,
        2 => day02::solve(input)?,
        3 => day03::solve(input)?,
        4 => day04::solve(input)?,
        5 => day05::solve(input)?,
        6 => day06::solve(input)?,
        7 => day07::solve(input)?,
        8 => day08::solve(input)?,
        9 => day09::solve(input)?,
        10 => day10::solve(input)?,
        _ => bail!("Day {day} not implemented"),
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.all {
        for day in 1..=12 {
            println!("========== Day {:02} ==========", day);
            run_day(day, args.example)?;
        }
    } else {
        let day = match args.day {
            Some(day) => day,
            None => {
                let day = OffsetDateTime::now_utc().day();
                eprintln!("Day not provided, running day {day}");
                day
            }
        };
        run_day(day, args.example)?;
    }

    Ok(())
}
