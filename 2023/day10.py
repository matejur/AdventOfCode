def print_grid(grid):
    for line in grid:
        print("".join(line))


pipe_to_dirs = {
    "|": [(0, -1), (0, 1)],
    "-": [(-1, 0), (1, 0)],
    "L": [(0, -1), (1, 0)],
    "J": [(0, -1), (-1, 0)],
    "7": [(0, 1), (-1, 0)],
    "F": [(0, 1), (1, 0)],
}


def next_from_start(grid, sx, sy):
    for dx, dy in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
        nx, ny = sx + dx, sy + dy
        pipe = grid[ny][nx]
        if pipe == ".":
            continue
        dirs = pipe_to_dirs[pipe]
        for px, py in dirs:
            if nx + px == sx and ny + py == sy:
                return nx, ny


def loop_length(grid, sx, sy):
    x, y = next_from_start(grid, sx, sy)
    length = 0

    while True:
        pipe = grid[y][x]
        if pipe == "S":
            grid[y][x] = "#"
            return length + 1

        grid[y][x] = "#"
        dirs = pipe_to_dirs[pipe]
        for dx, dy in dirs:
            nx, ny = x + dx, y + dy
            next_pipe = grid[ny][nx]
            if next_pipe not in ".#":
                if length == 0 and next_pipe == "S":
                    continue
                x, y = nx, ny
                length += 1
                break


def part1(input):
    grid = [["."] + list(line) + ["."] for line in input.splitlines()]
    grid.insert(0, ["."] * len(grid[0]))
    grid.append(["."] * len(grid[0]))

    sx, sy = 0, 0
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                sx, sy = x, y
                break

    return loop_length(grid, sx, sy) // 2


dilute_map = {
    "|": [[".", "|", "."], [".", "|", "."], [".", "|", "."]],
    "-": [[".", ".", "."], ["-", "-", "-"], [".", ".", "."]],
    "L": [[".", "|", "."], [".", "L", "-"], [".", ".", "."]],
    "J": [[".", "|", "."], ["-", "J", "."], [".", ".", "."]],
    "7": [[".", ".", "."], ["-", "7", "."], [".", "|", "."]],
    "F": [[".", ".", "."], [".", "F", "-"], [".", "|", "."]],
    "S": [[".", "|", "."], ["-", "S", "-"], [".", "|", "."]],
    ".": [[".", ".", "."], [".", ".", "."], [".", ".", "."]],
}


def dilute(grid):
    diluted = []
    for y, row in enumerate(grid):
        diluted.extend([[], [], []])
        for char in row:
            diluted[3 * y].extend(dilute_map[char][0])
            diluted[3 * y + 1].extend(dilute_map[char][1])
            diluted[3 * y + 2].extend(dilute_map[char][2])

    return diluted


def flood_fill(grid, x, y):
    stack = [(x, y)]
    while stack:
        x, y = stack.pop()
        if grid[y][x] != ".":
            continue

        grid[y][x] = "#"
        for dx, dy in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(grid[0]) and 0 <= ny < len(grid) and grid[ny][nx] == ".":
                stack.append((nx, ny))


def count_empty(grid):
    count = 0
    for y in range(0, len(grid), 3):
        for x in range(0, len(grid[y]), 3):
            if all(grid[y + dy][x + dx] == "." for dx in range(3) for dy in range(3)):
                count += 1
    return count


def part2(input):
    grid = [["."] + list(line) + ["."] for line in input.splitlines()]
    grid.insert(0, ["."] * len(grid[0]))
    grid.append(["."] * len(grid[0]))

    sx, sy = 0, 0
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                sx, sy = x, y
                break

    filtered_grid = [row[:] for row in grid]

    # Find the loop
    loop_length(grid, sx, sy)
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] != "#":
                filtered_grid[y][x] = "."

    diluted = dilute(filtered_grid)
    flood_fill(diluted, 0, 0)

    return count_empty(diluted)
