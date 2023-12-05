import os
import datetime
import requests
import importlib
import shutil
import argparse

SESSION = os.environ.get("AOC_SESSION")
YEAR = 2023


def download_input(day):
    res = requests.get(
        f"https://adventofcode.com/{YEAR}/day/{day}/input", cookies={"session": SESSION}
    )

    if res.status_code == 404:
        print(f"No input for day {day} available")
        exit()

    if res.status_code == 400:
        print("Please set AOC_SESSION env variable")
        exit()

    return res.text.strip()


def read_file(day):
    if not os.path.exists("inputs"):
        os.mkdir("inputs")

    if not os.path.exists(os.path.join("inputs", f"in{day:02}.txt")):
        puzzle_input = download_input(day)
        with open(os.path.join("inputs", f"in{day:02}.txt"), "w") as f:
            f.write(puzzle_input)

    with open(os.path.join("inputs", f"in{day:02}.txt")) as f:
        return f.read()


def read_example_stdin():
    lines = ""
    last_line = None
    print("Please paste the example:")
    while True:
        x = input()
        if x == "" and last_line == x:
            break
        last_line = x
        lines += x + "\n"
    return lines.strip()


def get_example(day):
    example_path = os.path.join("inputs", f"in{day:02}ex.txt")
    if not os.path.exists(example_path):
        example = read_example_stdin()
        with open(example_path, "w") as f:
            f.write(example)
            return example

    else:
        with open(example_path, "r") as f:
            return f.read()


def run_day(day, args):
    if args.example:
        input = get_example(day)
    else:
        input = read_file(day)

    if os.path.exists(f"day{day:02}.py"):
        module = importlib.import_module(f"day{day:02}")

        start = datetime.datetime.now()
        print("-" * 10 + f" Day {day} " + "-" * 10)
        print(f"Part 1: {module.part1(input)}")
        print(f"Part 2: {module.part2(input)}")

        if args.time:
            print(
                f"Execution time: {(datetime.datetime.now() - start).total_seconds() * 1000:.4f}ms"
            )
    else:
        shutil.copy("template.py", f"day{day:02}.py")
        print(f"Created python file: day{day:02}.py. Start coding :)")


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("-d", "--day", default=datetime.datetime.now().day, type=int)
    parser.add_argument("-e", "--example", action="store_true")
    parser.add_argument("-t", "--time", action="store_true")
    args = parser.parse_args()

    day = args.day

    if not 0 < day < 26:
        print(f"No puzzle for day {day}")
        exit()

    run_day(day, args)


if __name__ == "__main__":
    main()
