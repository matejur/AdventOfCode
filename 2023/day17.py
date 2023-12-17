from queue import PriorityQueue


def calculate_heat_loss(grid, part2):
    queue = PriorityQueue()
    queue.put((0, (0, 0, 0, 1, 0)))
    queue.put((0, (0, 0, 1, 0, 0)))
    visited = set()

    while not queue.empty():
        heat_loss, (x, y, dx, dy, streight) = queue.get()

        if x == len(grid[0]) - 1 and y == len(grid) - 1:
            if part2 and streight < 3:
                continue
            return heat_loss

        if x < 0 or x >= len(grid[0]) or y < 0 or y >= len(grid):
            continue

        if (x, y, dx, dy, streight) in visited:
            continue

        visited.add((x, y, dx, dy, streight))

        if part2 and streight < 10 or not part2 and streight < 3:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(grid[0]) and 0 <= ny < len(grid):
                queue.put((heat_loss + grid[ny][nx], (nx, ny, dx, dy, streight + 1)))

        if not part2 or streight > 3:
            for ndx, ndy in ((dy, dx), (-dy, -dx)):
                nx, ny = x + ndx, y + ndy
                if 0 <= nx < len(grid[0]) and 0 <= ny < len(grid):
                    queue.put((heat_loss + grid[ny][nx], (nx, ny, ndx, ndy, 1)))


def part1(input):
    grid = [list(map(int, line)) for line in input.splitlines()]
    return calculate_heat_loss(grid, False)


def part2(input):
    grid = [list(map(int, line)) for line in input.splitlines()]
    return calculate_heat_loss(grid, True)
