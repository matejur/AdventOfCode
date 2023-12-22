# My original implementation was very slow
# This is an adaptation of the solution from HyperNeutrino
from collections import deque


def xy_overlap(a, b):
    return max(a[0], b[0]) <= min(a[3], b[3]) and max(a[1], b[1]) <= min(a[4], b[4])


def settle(bricks):
    bricks.sort(key=lambda x: x[2])

    for i, brick in enumerate(bricks):
        max_z = 1
        for below in bricks[:i]:
            if xy_overlap(brick, below):
                max_z = max(max_z, below[5] + 1)
        brick[5] -= brick[2] - max_z
        brick[2] = max_z

    bricks.sort(key=lambda x: x[2])


def build_graph(bricks):
    supports = {i: set() for i in range(len(bricks))}
    supported_by = {i: set() for i in range(len(bricks))}

    for j, higher in enumerate(bricks):
        for i, lower in enumerate(bricks[:j]):
            if xy_overlap(lower, higher) and higher[2] == lower[5] + 1:
                supports[i].add(j)
                supported_by[j].add(i)

    return supports, supported_by


def part1(input):
    bricks = [
        list(map(int, line.replace("~", ",").split(","))) for line in input.splitlines()
    ]

    settle(bricks)
    supports, supported_by = build_graph(bricks)

    answer = 0
    for i in range(len(bricks)):
        if all(len(supported_by[j]) > 1 for j in supports[i]):
            answer += 1

    return answer


def part2(input):
    bricks = [
        list(map(int, line.replace("~", ",").split(","))) for line in input.splitlines()
    ]

    settle(bricks)
    supports, supported_by = build_graph(bricks)

    answer = 0

    for i in range(len(bricks)):
        queue = deque([j for j in supports[i] if len(supported_by[j]) == 1])
        falling = set(queue)
        falling.add(i)

        while queue:
            j = queue.popleft()
            for k in supports[j].difference(falling):
                if supported_by[k].issubset(falling):
                    queue.append(k)
                    falling.add(k)

        answer += len(falling) - 1

    return answer
