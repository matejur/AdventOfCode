def possible(group, reds, greens, blues):
    for color in group.split(", "):
        number = int(color.split()[0])

        if "red" in color and number > reds:
            return False
        if "green" in color and number > greens:
            return False
        if "blue" in color and number > blues:
            return False

    return True


def part1(input):
    sum = 0
    for game in input.splitlines():
        for groups in game.split(": ")[1].split("; "):
            if not possible(groups, 12, 13, 14):
                break
        else:
            sum += int(game.split(":")[0][5:])

    return sum


def set_power(groups):
    min_reds = 0
    min_greens = 0
    min_blues = 0

    for group in groups:
        for color in group.split(", "):
            number = int(color.split()[0])

            if "red" in color:
                min_reds = max(min_reds, number)
            elif "green" in color:
                min_greens = max(min_greens, number)
            elif "blue" in color:
                min_blues = max(min_blues, number)

    return min_blues * min_greens * min_reds


def part2(input):
    sum = 0
    for game in input.splitlines():
        sum += set_power(game.split(": ")[1].split("; "))

    return sum
