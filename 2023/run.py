import os
import sys
import datetime
import requests
import importlib
import shutil

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


def run_day(day):
    if not os.path.exists("inputs"):
        os.mkdir("inputs")

    if not os.path.exists(os.path.join("inputs", f"in{day:02}.txt")):
        puzzle_input = download_input(day)
        with open(os.path.join("inputs", f"in{day:02}.txt"), "w") as f:
            f.write(puzzle_input)

    with open(os.path.join("inputs", f"in{day:02}.txt")) as f:
        input = f.read()

    if os.path.exists(f"day{day:02}.py"):
        module = importlib.import_module(f"day{day:02}")

        print("-" * 10 + f" Day {day} " + "-" * 10)
        print(f"Part 1: {module.part1(input)}")
        print(f"Part 2: {module.part2(input)}")
    else:
        shutil.copy("template.py", f"day{day:02}.py")
        print(f"Created python file: day{day:02}.py. Start coding :)")


def main():
    if len(sys.argv) != 2:
        date = datetime.datetime.now()

        if date.month != 12:
            print("Running without arguments works only in December")
            exit()

        day = date.day
    else:
        day = int(sys.argv[1])

    if not 0 < day < 26:
        print(f"No puzzle for day {day}")
        exit()

    run_day(day)


if __name__ == "__main__":
    main()
