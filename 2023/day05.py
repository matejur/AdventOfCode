import math


def parse_maps(strings):
    maps = []
    for map_str in strings:
        lines = map_str.splitlines()[1:]
        map = []
        for line in lines:
            x, y, z = line.split()
            map.append((int(x), int(y), int(z)))
        maps.append(map)

    return maps


def apply_map_range(ranges, maps):
    maps.sort(key=lambda x: x[1])

    output_ranges = []
    for seed_start, seed_length in ranges:
        seed_end = seed_start + seed_length
        start = seed_start
        for map_dest, map_start, map_length in maps:
            map_end = map_start + map_length

            if map_start < start < map_end:
                length = min(map_end - start, seed_end - start)
                output_ranges.append((map_dest + seed_start - map_start, length))
                start = map_end
            elif start <= map_start <= seed_end:
                length = map_start - start
                output_ranges.append((start, length))  # a gap
                output_ranges.append((map_dest, min(map_length, seed_end - map_start)))
                start = map_end

        if start < seed_end:
            output_ranges.append((start, seed_end - start))

    return output_ranges


def part1(input):
    parts = input.split("\n\n")
    seeds, maps = map(int, parts[0].split()[1:]), parse_maps(parts[1:])

    min_location = math.inf
    for seed in seeds:
        seed = [[seed, 1]]
        for seed_map in maps:
            seed = apply_map_range(seed, seed_map)
        min_location = min(min_location, seed[0][0])

    return min_location


def part2(input):
    parts = input.split("\n\n")
    seeds, maps = list(map(int, parts[0].split()[1:])), parse_maps(parts[1:])

    min_location = math.inf
    for i in range(0, len(seeds), 2):
        ranges = [seeds[i : i + 2]]
        for seed_map in maps:
            ranges = apply_map_range(ranges, seed_map)

        min_location = min(sorted(ranges)[0][0], min_location)

    return min_location
