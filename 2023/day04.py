from collections import defaultdict


def part1(input):
    points = 0

    for card in input.splitlines():
        winning_numbers, your_numbers = card.split(":")[1].split("|")
        winning_numbers = set(winning_numbers.split())
        your_numbers = set(your_numbers.split())
        intersection = len(your_numbers.intersection(winning_numbers))
        points += 1 << intersection >> 1

    return points


def part2(input):
    cards = defaultdict(lambda: 1)
    for i, card in enumerate(input.splitlines()):
        winning_numbers, your_numbers = card.split(":")[1].split("|")
        winning_numbers = set(winning_numbers.split())
        your_numbers = set(your_numbers.split())
        intersection = len(your_numbers.intersection(winning_numbers))

        current_cards = cards[i]
        for j in range(intersection):
            cards[i + j + 1] += current_cards

    return sum(cards.values())
