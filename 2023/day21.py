from collections import deque


def count_plots(grid, sx, sy, steps):
    queue = deque([(sy, sx, steps)])
    seen = set((sy, sx))
    ans = set()

    while queue:
        y, x, remaining = queue.popleft()

        if remaining % 2 == 0:
            ans.add((y, x))

        if remaining == 0:
            continue

        for dy, dx in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            ny, nx = y + dy, x + dx
            if ny < 0 or ny >= len(grid) or nx < 0 or nx >= len(grid[0]):
                continue
            if grid[ny][nx] == "#" or (ny, nx) in seen:
                continue

            seen.add((ny, nx))
            queue.append((ny, nx, remaining - 1))

    return len(ans)


def part1(input):
    grid = input.splitlines()

    sy, sx = next(
        (y, x) for y, row in enumerate(grid) for x, c in enumerate(row) if c == "S"
    )

    return count_plots(grid, sx, sy, 64)


def determinant(x1, x2, x3, y1, y2, y3, z1, z2, z3):
    return (
        x1 * y2 * z3
        + x2 * y3 * z1
        + x3 * y1 * z2
        - x3 * y2 * z1
        - x2 * y1 * z3
        - x1 * y3 * z2
    )


def part2(input):
    grid = input.replace("S", ".").splitlines()

    expansion_factor = 5
    expanded_grid = []
    for row in grid:
        expanded_grid.append(row * expansion_factor)
    grid = expanded_grid * expansion_factor

    sx, sy = len(grid) // 2, len(grid) // 2

    y1 = count_plots(grid, sx, sy, 65 + 0 * 131)
    y2 = count_plots(grid, sx, sy, 65 + 1 * 131)
    y3 = count_plots(grid, sx, sy, 65 + 2 * 131)

    # y1 = 0a + 0b + c
    # y2 = 1a + 1b + c
    # y3 = 4a + 2b + c

    # Cramer's rule
    D = determinant(0, 0, 1, 1, 1, 1, 4, 2, 1)
    Da = determinant(y1, 0, 1, y2, 1, 1, y3, 2, 1)
    Db = determinant(0, y1, 1, 1, y2, 1, 4, y3, 1)
    Dc = determinant(0, 0, y1, 1, 1, y2, 4, 2, y3)

    a = Da // D
    b = Db // D
    c = Dc // D

    x = 202300

    return a * x * x + b * x + c
