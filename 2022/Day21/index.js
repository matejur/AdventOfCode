function makeEqual(value, monkey, monkeys) {
    const currMonkey = monkeys.find((m) => m.name === monkey);

    if (monkey === "humn") {
        return value;
    }

    const equation = currMonkey.equation.split(" ");
    const op1 = monkeys.find((m) => m.name === equation[0]).value;
    const op2 = monkeys.find((m) => m.name === equation[2]).value;
    const operator = equation[1];

    let equal = 0;
    switch (operator) {
        case "+":
            if (op2) equal = value - op2;
            else equal = value - op1;
            break;
        case "*":
            if (op2) equal = value / op2;
            else equal = value / op1;
            break;
        case "-":
            if (op2) equal = value + op2;
            else equal = op1 - value;
            break;
        case "/":
            if (op2) equal = value * op2;
            else equal = op1 / value;
            break;
        default:
            throw new Error("Unknown operator: " + operator);
    }

    if (op2) return makeEqual(equal, equation[0], monkeys);
    else return makeEqual(equal, equation[2], monkeys);
}

function calculate(monkey, monkeys, part2 = false) {
    const currMonkey = monkeys.find((m) => m.name === monkey);

    if (part2 && monkey === "humn") {
        return NaN;
    }

    if (currMonkey.value) {
        return currMonkey.value;
    }

    const equation = currMonkey.equation.split(" ");
    const op1 = calculate(equation[0], monkeys, part2);
    const op2 = calculate(equation[2], monkeys, part2);
    const operator = equation[1];

    if (part2 && monkey === "root") {
        return makeEqual(op2, equation[0], monkeys);
    }

    switch (operator) {
        case "+":
            currMonkey.value = op1 + op2;
            break;
        case "*":
            currMonkey.value = op1 * op2;
            break;
        case "-":
            currMonkey.value = op1 - op2;
            break;
        case "/":
            currMonkey.value = op1 / op2;
            break;
        default:
            throw new Error("Unknown operator: " + operator);
    }

    return currMonkey.value;
}

function parseInput(input) {
    return input.split("\n").map((line) => {
        const split = line.split(": ");
        const name = split[0];
        const value = parseInt(split[1]);

        if (value) {
            return { name, value };
        } else {
            return { name, equation: split[1] };
        }
    });
}

export function solve(input) {
    let monkeys = parseInput(input);
    console.log("Part 1: " + calculate("root", monkeys));

    monkeys = parseInput(input);
    console.log("Part 2: " + calculate("root", monkeys, true));
}
