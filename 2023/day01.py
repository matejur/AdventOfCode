def part1(input):
    lines = input.splitlines()

    calibration_sum = 0
    for line in lines:
        digits = [x for x in line if x.isnumeric()]
        calibration_sum += int(digits[0] + digits[-1])

    return calibration_sum


def part2(input):
    lines = input.splitlines()

    table = {
        "one": "1",
        "two": "2",
        "three": "3",
        "four": "4",
        "five": "5",
        "six": "6",
        "seven": "7",
        "eight": "8",
        "nine": "9",
    }

    calibration_sum = 0
    for line in lines:
        digits = []
        start = 0
        for i, c in enumerate(line):
            if c.isnumeric():
                digits.append(c)
                start = i
                continue

            for key in table:
                if key in line[start : i + 1]:
                    digits.append(table[key])
                    start = i
                    break

        calibration_sum += int(digits[0] + digits[-1])

    return calibration_sum
