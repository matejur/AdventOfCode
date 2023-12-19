def parse_workflows(workflows):
    out = {}

    for workflow in workflows.splitlines():
        name, rules = workflow[:-1].split("{")
        out[name] = []
        for rule in rules.split(","):
            rule = rule.split(":")
            if len(rule) == 1:
                out[name].append(rule[0])
            else:
                out[name].append(tuple(rule))

    return out


def parse_parts(parts):
    out = []

    for part in parts.splitlines():
        part = part[1:-1].split(",")
        out.append({name: int(value) for name, value in [p.split("=") for p in part]})

    return out


def eval_part(part, workflows):
    name = "in"

    while True:
        if name == "A":
            return True
        if name == "R":
            return False

        current = workflows[name]
        for rule in current:
            if type(rule) == tuple:
                rule, dest = rule
                op = rule[1]
                value = int(rule[2:])
                for name, limit in part.items():
                    if name in rule:
                        if op == "<":
                            if limit < value:
                                name = dest
                                break
                        elif op == ">":
                            if limit > value:
                                name = dest
                                break
                else:
                    continue
                break
            else:
                name = rule


def part1(input):
    workflows, parts = input.split("\n\n")
    workflows = parse_workflows(workflows)
    parts = parse_parts(parts)

    answer = 0
    for part in parts:
        if eval_part(part, workflows):
            answer += sum(part.values())

    return answer


def num_accepted(parts, current, workflows):
    parts = parts.copy()

    if current == "A":
        answer = 1
        for s, e in parts:
            answer *= e - s + 1
        return answer

    if current == "R":
        return 0

    work = workflows[current]
    answer = 0
    for rule in work:
        if type(rule) == tuple:
            rule, dest = rule
            index = "xmas".index(rule[0])
            op = rule[1]
            value = int(rule[2:])

            start, end = parts[index]
            if op == "<":
                # all go to the next workflow
                if end < value:
                    return num_accepted(parts, dest, workflows)
                # all go to the next rule
                elif value < start:
                    continue
                # part goes to the next rule, rest goes to the next workflow
                else:
                    parts_to_continue = (value, end)
                    parts_to_next = (start, value - 1)
                    parts[index] = parts_to_next
                    answer += num_accepted(parts, dest, workflows)
                    parts[index] = parts_to_continue
            elif op == ">":
                # all go to the next workflow
                if start > value:
                    return num_accepted(parts, dest, workflows)
                # all go to the next rule
                elif value > end:
                    continue
                # part goes to the next rule, rest goes to the next workflow
                else:
                    parts_to_continue = (start, value)
                    parts_to_next = (value + 1, end)
                    parts[index] = parts_to_next
                    answer += num_accepted(parts, dest, workflows)
                    parts[index] = parts_to_continue
        else:
            answer += num_accepted(parts, rule, workflows)

    return answer


def part2(input):
    workflows, _ = input.split("\n\n")
    workflows = parse_workflows(workflows)

    return num_accepted([(1, 4000), (1, 4000), (1, 4000), (1, 4000)], "in", workflows)
