def dfs(graph, current, end, seen):
    if current == end:
        return 0

    seen.add(current)
    distances = [0]
    for i, dist in enumerate(graph[current]):
        if dist == 0 or i in seen:
            continue
        distances.append(dfs(graph, i, end, seen) + dist)
    seen.remove(current)

    return max(distances)


def build_graph(grid, start, end, directions):
    intersections = [start, end]

    for y in range(len(grid)):
        for x in range(len(grid[y])):
            ch = grid[y][x]
            if ch == "#":
                continue

            neighbors = 0
            for nx, ny in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]:
                if (
                    0 <= ny < len(grid)
                    and 0 <= nx < len(grid[ny])
                    and grid[ny][nx] != "#"
                ):
                    neighbors += 1

            if neighbors > 2:
                intersections.append((x, y))

    graph = [[0 for _ in range(len(intersections))] for _ in range(len(intersections))]

    for ix, iy in intersections:
        stack = [(ix, iy, 0)]
        seen = {(ix, iy)}

        while stack:
            x, y, dist = stack.pop()

            if dist != 0 and (x, y) in intersections:
                graph[intersections.index((ix, iy))][intersections.index((x, y))] = dist
                continue

            ch = grid[y][x]
            for dx, dy in directions[ch]:
                nx, ny = x + dx, y + dy
                if (nx, ny) in seen:
                    continue
                if (
                    0 <= ny < len(grid)
                    and 0 <= nx < len(grid[ny])
                    and grid[ny][nx] != "#"
                ):
                    seen.add((nx, ny))
                    stack.append((nx, ny, dist + 1))

    return graph, intersections


def solve(input, directions):
    grid = input.splitlines()
    sy, sx = (0, grid[0].index("."))
    ey, ex = (len(grid) - 1, grid[-1].index("."))

    graph, intersections = build_graph(grid, (sx, sy), (ex, ey), directions)

    start_index = intersections.index((sx, sy))
    end_index = intersections.index((ex, ey))

    return dfs(graph, start_index, end_index, set())


def part1(input):
    directions = {
        "^": [(0, -1)],
        "v": [(0, 1)],
        "<": [(-1, 0)],
        ">": [(1, 0)],
        ".": [(1, 0), (-1, 0), (0, 1), (0, -1)],
    }
    return solve(input, directions)


def part2(input):
    directions = {k: [(0, 1), (0, -1), (1, 0), (-1, 0)] for k in "^v<>."}
    return solve(input, directions)
