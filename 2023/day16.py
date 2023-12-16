from collections import deque

UP = 0
RIGHT = 1
DOWN = 2
LEFT = 3


def move(x, y, dir):
    if dir == UP:
        return x, y - 1, UP
    elif dir == RIGHT:
        return x + 1, y, RIGHT
    elif dir == DOWN:
        return x, y + 1, DOWN
    elif dir == LEFT:
        return x - 1, y, LEFT


reflections = {
    "/": {UP: RIGHT, RIGHT: UP, DOWN: LEFT, LEFT: DOWN},
    "\\": {UP: LEFT, RIGHT: DOWN, DOWN: RIGHT, LEFT: UP},
}


def count_energized(grid, x, y, dir):
    beams = deque([(x, y, dir)])
    energized_tiles = set()
    all_beams = set()

    while len(beams) > 0:
        x, y, dir = beams.popleft()

        if (x, y, dir) in all_beams:
            continue

        if not 0 <= x < len(grid[0]) or not 0 <= y < len(grid):
            continue

        energized_tiles.add((x, y))
        all_beams.add((x, y, dir))

        if grid[y][x] == ".":
            beams.append(move(x, y, dir))

        elif grid[y][x] in "/\\":
            dir = reflections[grid[y][x]][dir]
            beams.append(move(x, y, dir))

        elif grid[y][x] == "|":
            if dir == UP or dir == DOWN:
                beams.append(move(x, y, dir))
            else:
                beams.append(move(x, y, UP))
                beams.append(move(x, y, DOWN))

        elif grid[y][x] == "-":
            if dir == LEFT or dir == RIGHT:
                beams.append(move(x, y, dir))
            else:
                beams.append(move(x, y, LEFT))
                beams.append(move(x, y, RIGHT))

    return len(energized_tiles)


def part1(input):
    grid = [list(line) for line in input.splitlines()]

    return count_energized(grid, 0, 0, RIGHT)


def part2(input):
    grid = [list(line) for line in input.splitlines()]

    max_energized = 0
    for x in range(len(grid[0])):
        max_energized = max(max_energized, count_energized(grid, x, 0, DOWN))
        max_energized = max(max_energized, count_energized(grid, x, len(grid) - 1, UP))

    for y in range(len(grid)):
        max_energized = max(max_energized, count_energized(grid, 0, y, RIGHT))
        max_energized = max(
            max_energized, count_energized(grid, len(grid[0]) - 1, y, LEFT)
        )

    return max_energized
