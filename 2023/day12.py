MEMO = {}


def number_of_arrangements(
    springs, groups, current_index, current_group, springs_in_group
):
    key = (current_index, current_group, springs_in_group)

    if key in MEMO:
        return MEMO[key]

    if current_index == len(springs):
        # Accounting for . at the end .###..., however .###.#. should fail (when only one group of 3 is needed)
        if current_group == len(groups) and springs_in_group == 0:
            return 1

        # Cases like ..###
        if current_group == len(groups) - 1 and springs_in_group == groups[-1]:
            return 1

        return 0

    answer = 0
    cur = springs[current_index]
    if cur == "?":
        # If we have empty group, we can add one more .
        if springs_in_group == 0:
            answer += number_of_arrangements(
                springs, groups, current_index + 1, current_group, 0
            )

        # If we have enough springs in group, we can start new group
        if current_group < len(groups) and groups[current_group] == springs_in_group:
            answer += number_of_arrangements(
                springs, groups, current_index + 1, current_group + 1, 0
            )

        # If the group has some more space for springs, we can add one more
        if current_group < len(groups) and groups[current_group] > springs_in_group:
            answer += number_of_arrangements(
                springs, groups, current_index + 1, current_group, springs_in_group + 1
            )

    elif cur == ".":
        if springs_in_group == 0:
            answer += number_of_arrangements(
                springs, groups, current_index + 1, current_group, 0
            )
        elif current_group < len(groups) and groups[current_group] == springs_in_group:
            answer += number_of_arrangements(
                springs, groups, current_index + 1, current_group + 1, 0
            )

    elif cur == "#":
        answer += number_of_arrangements(
            springs, groups, current_index + 1, current_group, springs_in_group + 1
        )

    MEMO[key] = answer
    return answer


def part1(input):
    records = [record.split() for record in input.splitlines()]
    records = [(record[0], tuple(map(int, record[1].split(",")))) for record in records]

    answer = 0
    for springs, groups in records:
        answer += number_of_arrangements(springs, groups, 0, 0, 0)
        MEMO.clear()

    return answer


def part2(input):
    records = [record.split() for record in input.splitlines()]
    records = [(record[0], tuple(map(int, record[1].split(",")))) for record in records]

    answer = 0
    for springs, groups in records:
        springs = "?".join([springs, springs, springs, springs, springs])
        groups = 5 * groups
        answer += number_of_arrangements(springs, groups, 0, 0, 0)
        MEMO.clear()

    return answer
