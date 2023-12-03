def extract_number(grid, x, y):
    row = grid[y]

    while row[x].isdigit():
        x -= 1
        if x == -1:
            break
    x += 1

    number = ""
    while row[x].isdigit():
        number += row[x]
        row[x] = "."

        x += 1
        if x == len(row):
            break

    return int(number)


def find_adjacent_numbers(grid, x, y):
    neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

    numbers = []
    for dx, dy in neighbors:
        nx, ny = x + dx, y + dy
        if grid[ny][nx].isdigit():
            numbers.append(extract_number(grid, nx, ny))

    return numbers


def part1(input):
    grid = [[c for c in line] for line in input.splitlines()]

    answer = 0
    for y, row in enumerate(grid):
        for x, char in enumerate(row):
            if char != "." and not char.isalnum():
                answer += sum(find_adjacent_numbers(grid, x, y))

    return answer


def part2(input):
    grid = [[c for c in line] for line in input.splitlines()]

    answer = 0
    for y, row in enumerate(grid):
        for x, char in enumerate(row):
            if char == "*":
                gears = find_adjacent_numbers(grid, x, y)
                if len(gears) == 2:
                    answer += gears[0] * gears[1]

    return answer
