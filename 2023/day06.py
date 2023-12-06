import math


def number_of_ways(time, distance):
    # return bruteforce(time, distance)
    # return binary_search(time, distance)
    return equation(time, distance)


def bruteforce(time, distance):
    # Part 2 in ~5s
    result = 0
    for i in range(time):
        result += (time - i) * i > distance
    return result


def binary_search(time, distance):
    start, end = 0, time
    while start < end:
        mid = (start + end) // 2
        if (time - mid) * mid > distance:
            start = mid + 1
        else:
            end = mid
    high = end

    start, end = 0, time
    while start < end:
        mid = (start + end) // 2
        if (time - mid) * mid > distance:
            end = mid
        else:
            start = mid + 1
    low = end

    return high - low


def equation(time, distance):
    high = (time + (time**2 - 4 * distance) ** (1 / 2)) / 2
    low = (time - (time**2 - 4 * distance) ** (1 / 2)) / 2

    return math.ceil(high) - math.floor(low) - 1


def part1(input):
    times, distances = input.split("\n")
    times = map(int, times.split(":")[1].split())
    distances = map(int, distances.split(":")[1].split())

    answer = 1
    for time, dist in zip(times, distances):
        answer *= number_of_ways(time, dist)

    return answer


def part2(input):
    times, distances = input.split("\n")
    time = int(times.split(":")[1].replace(" ", ""))
    distance = int(distances.split(":")[1].replace(" ", ""))

    return number_of_ways(time, distance)
