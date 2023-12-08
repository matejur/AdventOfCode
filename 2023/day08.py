from math import lcm


def part1(input):
    instructions, nodes = input.split("\n\n")

    network = {}
    for node in nodes.splitlines():
        node, children = node.split(" = ")
        ch1, ch2 = children[1:-1].split(", ")
        network[node] = (ch1, ch2)

    current_node = "AAA"
    steps = 0
    while current_node != "ZZZ":
        instr = instructions[steps % len(instructions)]
        current_node = network[current_node][instr == "R"]
        steps += 1

    return steps


def part2(input):
    instructions, nodes = input.split("\n\n")

    network = {}
    for node in nodes.splitlines():
        node, children = node.split(" = ")
        ch1, ch2 = children[1:-1].split(", ")
        network[node] = (ch1, ch2)

    cycles = []
    starting_nodes = [node for node in network if node[-1] == "A"]
    for node in starting_nodes:
        current_node = node
        steps = 0
        while current_node[-1] != "Z":
            instr = instructions[steps % len(instructions)]
            current_node = network[current_node][instr == "R"]
            steps += 1
        cycles.append(steps)

    return lcm(*cycles)
