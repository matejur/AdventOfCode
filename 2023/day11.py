import itertools


def solve_expansion(space, expansion):
    galaxies = [
        (y, x)
        for y in range(len(space))
        for x in range(len(space[0]))
        if space[y][x] == "#"
    ]

    empty_rows = [i for i, row in enumerate(space) if all(x == "." for x in row)]
    empty_columns = [
        i for i, column in enumerate(zip(*space)) if all(x == "." for x in column)
    ]

    answer = 0
    for a, b in itertools.combinations(galaxies, 2):
        empty_rows_between = sum(
            1 for i in range(min(a[0], b[0]) + 1, max(a[0], b[0])) if i in empty_rows
        )

        empty_columns_between = sum(
            1 for i in range(min(a[1], b[1]) + 1, max(a[1], b[1])) if i in empty_columns
        )

        answer += (
            abs(a[0] - b[0])
            + abs(a[1] - b[1])
            + empty_rows_between * (expansion - 1)
            + empty_columns_between * (expansion - 1)
        )

    return answer


def part1(input):
    space = [list(x) for x in input.splitlines()]

    return solve_expansion(space, 2)


def part2(input):
    space = [list(x) for x in input.splitlines()]

    return solve_expansion(space, 1000000)
