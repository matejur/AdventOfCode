class Monkey {
    constructor(items, test_div, operation, monkey1, monkey2) {
        this.items = items;
        this.test_div = test_div;
        this.operation = operation;
        this.monkey1 = monkey1;
        this.monkey2 = monkey2;
        this.items_inspected = 0;
        this.modulo = 0;
    }

    turn(monkeys) {
        while (this.items.length) {
            const old = this.items.shift();
            let worry = eval(this.operation);

            if (this.modulo) worry = worry % this.modulo;
            else worry = Math.floor(worry / 3);

            this.items_inspected++;

            if (worry % this.test_div === 0) {
                monkeys[this.monkey1].items.push(worry);
            } else {
                monkeys[this.monkey2].items.push(worry);
            }
        }
    }
}

function getScore(monkeys) {
    let a = 0;
    let b = 0;
    for (let monkey of monkeys) {
        const num = monkey.items_inspected;

        if (num > a) {
            b = a;
            a = num;
        } else if (num < a && num > b) {
            b = num;
        }
    }
    return a * b;
}

export function solve(input) {
    const monkeys1 = [];
    const monkeys2 = [];
    let modulo = 1;

    input.split("\n\n").forEach((monkey) => {
        let [_, items, operation, test, monkey1, monkey2] = monkey.split("\n");

        let items1 = items.slice(17).split(", ").map(Number);
        let items2 = items.slice(17).split(", ").map(Number);
        operation = operation.slice(19);
        test = parseInt(test.split(" ").pop());
        monkey1 = parseInt(monkey1.split(" ").pop());
        monkey2 = parseInt(monkey2.split(" ").pop());

        modulo *= test;

        monkeys1.push(new Monkey(items1, test, operation, monkey1, monkey2));
        monkeys2.push(new Monkey(items2, test, operation, monkey1, monkey2));
    });

    for (let i = 0; i < 20; i++) {
        monkeys1.forEach((monkey) => monkey.turn(monkeys1));
    }

    monkeys2.forEach((monkey) => (monkey.modulo = modulo));
    for (let i = 0; i < 10000; i++) {
        monkeys2.forEach((monkey) => monkey.turn(monkeys2));
    }

    console.log("Part 1: " + getScore(monkeys1));
    console.log("Part 2: " + getScore(monkeys2));
}
