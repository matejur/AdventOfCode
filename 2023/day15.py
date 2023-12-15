def hash_step(step):
    hash = 0
    for c in step:
        hash += ord(c)
        hash *= 17
        hash %= 256

    return hash


def part1(input):
    return sum(hash_step(step) for step in input.split(","))


def part2(input):
    boxes = [[] for _ in range(256)]

    for step in input.split(","):
        if "-" in step:
            label = step[:-1]
            hash = hash_step(label)
            boxes[hash] = list(filter(lambda box: box[0] != label, boxes[hash]))
        else:
            label = step[:-2]
            focal = int(step[-1])
            hash = hash_step(label)

            for lens in boxes[hash]:
                if lens[0] == label:
                    lens[1] = focal
                    break
            else:
                boxes[hash].append([label, focal])

    answer = 0
    for i, box in enumerate(boxes):
        for j, lens in enumerate(box):
            answer += (i + 1) * (j + 1) * lens[1]

    return answer
