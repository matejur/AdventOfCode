def predict(history):
    sequences = [history]

    last_sequence = history
    while not all([x == 0 for x in last_sequence]):
        last_sequence = [y - x for x, y in zip(last_sequence, last_sequence[1:])]
        sequences.append(last_sequence)

    sequences.reverse()
    next_value = 0
    for sequence in sequences[1:]:
        next_value = sequence[-1] + next_value

    return next_value


def part1(input):
    lines = input.splitlines()

    answer = 0
    for history in lines:
        history = list(map(int, history.split()))
        answer += predict(history)

    return answer


def part2(input):
    lines = input.splitlines()

    answer = 0
    for history in lines:
        history = list(map(int, history.split()))
        answer += predict(history[::-1])

    return answer
