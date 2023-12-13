def vertical_reflection(grid, find_smudges=False):
    for x in range(len(grid[0]) - 1):
        smudges = 0
        for y in range(len(grid)):
            for offset in range(min(x + 1, len(grid[0]) - x - 1)):
                if grid[y][x - offset] != grid[y][x + offset + 1]:
                    smudges += 1
            if not find_smudges and smudges > 0:
                break
        if not find_smudges and smudges == 0:
            return x + 1
        if find_smudges and smudges == 1:
            return x + 1
    return 0


def horizontal_reflection(grid, find_smudges=False):
    for y in range(len(grid) - 1):
        smudges = 0
        for x in range(len(grid[0])):
            for offset in range(min(y + 1, len(grid) - y - 1)):
                if grid[y - offset][x] != grid[y + offset + 1][x]:
                    smudges += 1
            if not find_smudges and smudges > 0:
                break
        if not find_smudges and smudges == 0:
            return y + 1
        if find_smudges and smudges == 1:
            return y + 1
    return 0


def part1(input):
    grids = input.split("\n\n")
    grids = [[list(row) for row in grid.splitlines()] for grid in grids]

    answer = 0
    for grid in grids:
        answer += vertical_reflection(grid)
        answer += 100 * horizontal_reflection(grid)

    return answer


def part2(input):
    grids = input.split("\n\n")
    grids = [[list(row) for row in grid.splitlines()] for grid in grids]

    answer = 0
    for grid in grids:
        answer += vertical_reflection(grid, find_smudges=True)
        answer += 100 * horizontal_reflection(grid, find_smudges=True)

    return answer
