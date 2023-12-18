def calculate_area(instructions):
    vertices = []
    x, y = 0, 0
    boundary = 0
    for dir, meters in instructions:
        if dir == "R":
            x += meters
        elif dir == "L":
            x -= meters
        elif dir == "U":
            y -= meters
        elif dir == "D":
            y += meters

        boundary += meters

        vertices.append((x, y))

    area = 0
    for v1, v2 in zip(vertices, vertices[1:]):
        area += v1[0] * v2[1] - v1[1] * v2[0]
    area += vertices[-1][0] * vertices[0][1] - vertices[-1][1] * vertices[0][0]

    return int((area + boundary) / 2) + 1


def part1(input):
    return calculate_area(
        [(line.split()[0], int(line.split()[1])) for line in input.splitlines()]
    )


def part2(input):
    return calculate_area(
        [
            (
                ["R", "D", "L", "U"][int(line.split()[2][1:-1][-1])],
                int(line.split()[2][1:-1][:-1].replace("#", "0x"), 16),
            )
            for line in input.splitlines()
        ]
    )
