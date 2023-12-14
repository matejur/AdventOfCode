def tilt_north(platform):
    for y in range(len(platform)):
        for x in range(len(platform[y])):
            if platform[y][x] == "O":
                platform[y][x] = "."
                stop_y = y
                while stop_y > 0 and platform[stop_y - 1][x] == ".":
                    stop_y -= 1
                platform[stop_y][x] = "O"


def tilt_west(platform):
    for y in range(len(platform)):
        for x in range(len(platform[y])):
            if platform[y][x] == "O":
                platform[y][x] = "."
                stop_x = x
                while stop_x > 0 and platform[y][stop_x - 1] == ".":
                    stop_x -= 1
                platform[y][stop_x] = "O"


def tilt_east(platform):
    for y in range(len(platform)):
        for x in range(len(platform[y]) - 1, -1, -1):
            if platform[y][x] == "O":
                platform[y][x] = "."
                stop_x = x
                while stop_x < len(platform[y]) - 1 and platform[y][stop_x + 1] == ".":
                    stop_x += 1
                platform[y][stop_x] = "O"


def tilt_south(platform):
    for y in range(len(platform) - 1, -1, -1):
        for x in range(len(platform[y])):
            if platform[y][x] == "O":
                platform[y][x] = "."
                stop_y = y
                while stop_y < len(platform) - 1 and platform[stop_y + 1][x] == ".":
                    stop_y += 1
                platform[stop_y][x] = "O"


def cycle(platform):
    tilt_north(platform)
    tilt_west(platform)
    tilt_south(platform)
    tilt_east(platform)


def hash_platform(platform):
    return "".join(["".join(row) for row in platform])


def calculate_load(platform):
    answer = 0
    for i, row in enumerate(platform):
        rocks = row.count("O")
        answer += rocks * (len(platform) - i)
    return answer


def part1(input):
    platform = [list(row) for row in input.splitlines()]

    tilt_north(platform)

    return calculate_load(platform)


def part2(input):
    platform = [list(row) for row in input.splitlines()]

    platform_hashes = {}
    for i in range(1000000000):
        cycle(platform)
        platform_hash = hash_platform(platform)
        if platform_hash in platform_hashes:
            diff = i - platform_hashes[platform_hash]
            remaining = 1000000000 - i - 1
            remaining %= diff
            break

        platform_hashes[platform_hash] = i

    for i in range(remaining):
        cycle(platform)

    return calculate_load(platform)
