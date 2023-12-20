from collections import deque
from math import lcm


class Module:
    def __init__(self, type, dest, name):
        self.type = type
        self.name = name
        self.dest = dest
        self.output = False
        self.inputs = {}
        self.pulses = [0, 0]

    def pulse(self, module):
        self.pulses[module.output] += 1

        if self.type == "%":
            if not module.output:
                self.output = not self.output
            else:
                return False
        elif self.type == "&":
            self.inputs[module.name] = module.output
            self.output = not all(self.inputs.values())
        elif self.type == "o":
            return False
        return True

    def __repr__(self):
        return f"Module({self.name}, {self.type}, {self.dest}, {self.inputs})"


def parse_config(input):
    config = {}
    for line in input.splitlines():
        name, dest = line.split(" -> ")
        dest = dest.split(", ")

        for type in "%&":
            if type in name:
                config[name[1:]] = Module(type, dest, name[1:])
                break
        else:
            config[name] = Module("b", dest, name)

    output_modules = {}
    for module in config:
        for dest in config[module].dest:
            if dest not in config:
                output_modules[dest] = Module("o", [], dest)

    config.update(output_modules)

    return config


def press_button(config):
    queue = deque(["broadcaster"])
    while queue:
        module_name = queue.popleft()
        module = config[module_name]
        for dest in module.dest:
            # print(f"{module_name} -{module.output}-> {dest}")
            happened = config[dest].pulse(module)
            if happened:
                queue.append(dest)


def part1(input):
    config = parse_config(input)

    for module in config:
        for dest in config[module].dest:
            config[dest].inputs[module] = False

    for _ in range(1000):
        press_button(config)

    high = 0
    low = 1000
    for module in config.values():
        high += module.pulses[True]
        low += module.pulses[False]

    return high * low


def part2(input):
    config = parse_config(input)

    for module in config:
        for dest in config[module].dest:
            config[dest].inputs[module] = False

    rx_in = list(config["rx"].inputs.keys())[0]

    inputs = {n: 0 for n in config[rx_in].inputs}

    presses = 0
    while True:
        presses += 1
        queue = deque(["broadcaster"])
        while queue:
            module_name = queue.popleft()
            module = config[module_name]
            for dest in module.dest:
                # print(f"{module_name} -{module.output}-> {dest}")
                happened = config[dest].pulse(module)

                for m in inputs:
                    if m == dest and config[m].output and inputs[m] == 0:
                        inputs[m] = presses

                if all(x > 0 for x in inputs.values()):
                    return lcm(*inputs.values())

                if happened:
                    queue.append(dest)
