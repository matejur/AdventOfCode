from enum import Enum
from collections import Counter
from functools import cmp_to_key


class HandType(Enum):
    HIGH_CARD = 1
    PAIR = 2
    TWO_PAIRS = 3
    THREE_OF_A_KIND = 4
    FULL_HOUSE = 5
    FOUR_OF_A_KIND = 6
    FIVE_OF_A_KIND = 7

    def __sub__(self, other):
        return self.value - other.value


def part1_type(hand):
    cards = Counter(hand)

    if max(cards.values()) == 5:
        return HandType.FIVE_OF_A_KIND

    first, second = cards.most_common(2)
    first = first[1]
    second = second[1]

    if first == 4:
        return HandType.FOUR_OF_A_KIND
    elif first == 3 and second == 2:
        return HandType.FULL_HOUSE
    elif first == 3:
        return HandType.THREE_OF_A_KIND
    elif first == 2 and second == 2:
        return HandType.TWO_PAIRS
    elif first == 2:
        return HandType.PAIR
    else:
        return HandType.HIGH_CARD


def part2_type(hand):
    cards = Counter(hand)

    jokers = cards["J"]
    if jokers == 0:
        return part1_type(hand)

    if jokers == 5:
        return HandType.FIVE_OF_A_KIND

    first, second = cards.most_common(2)

    if first[0] == "J" or second[0] == "J":
        value = first[1] + second[1]
        if value == 5:
            return HandType.FIVE_OF_A_KIND
        elif value == 4:
            return HandType.FOUR_OF_A_KIND
        elif value == 3:
            return HandType.THREE_OF_A_KIND
        elif value == 2:
            return HandType.PAIR

    a = first[1]
    b = second[1]
    if first[0] != "J":
        a += jokers
    elif second[0] != "J":
        b += jokers

    first, second = max(a, b), min(a, b)

    if first == 4:
        return HandType.FOUR_OF_A_KIND
    elif first == 3 and second == 2:
        return HandType.FULL_HOUSE
    elif first == 3:
        return HandType.THREE_OF_A_KIND
    elif first == 2 and second == 2:
        return HandType.TWO_PAIRS
    elif first == 2:
        return HandType.PAIR
    else:
        return HandType.HIGH_CARD


def hand_comparator(hand1, hand2, mapping, hand_type_fn):
    hand1_type = hand_type_fn(hand1)
    hand2_type = hand_type_fn(hand2)

    if hand1_type != hand2_type:
        return hand1_type - hand2_type

    for card1, card2 in zip(hand1, hand2):
        if card1 == card2:
            continue
        return mapping[card1] - mapping[card2]


def total_winnings(input, mapping, hand_type_fn):
    hands = [
        (hand, int(bid)) for line in input.splitlines() for hand, bid in [line.split()]
    ]

    hands.sort(
        key=cmp_to_key(lambda x, y: hand_comparator(x[0], y[0], mapping, hand_type_fn))
    )

    answer = 0
    for i, (_, bid) in enumerate(hands):
        answer += bid * (i + 1)

    return answer


def part1(input):
    mapping = {
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "T": 10,
        "J": 11,
        "Q": 12,
        "K": 13,
        "A": 14,
    }

    return total_winnings(input, mapping, part1_type)


def part2(input):
    mapping = {
        "J": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "T": 10,
        "Q": 11,
        "K": 12,
        "A": 13,
    }

    return total_winnings(input, mapping, part2_type)
