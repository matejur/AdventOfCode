export function solve(input) {
    const [drawing, moves] = input.split("\n\n");

    const num_stacks = drawing.match(/\d+/g).length;
    const stacks1 = new Array(num_stacks).fill(0).map((i) => []);
    const stacks2 = new Array(num_stacks).fill(0).map((i) => []);

    const rows = drawing.split("\n").reverse();
    for (let i = 1; i < rows.length; i++) {
        const row = rows[i];
        const letters = row.match(/.{1,4}/g);

        for (let j = 0; j < letters.length; j++) {
            const letter = letters[j].match(/[A-Z]/);
            if (letter) {
                stacks1[j].push(letter[0]);
                stacks2[j].push(letter[0]);
            }
        }
    }

    moves
        .trim()
        .split("\n")
        .forEach((element) => {
            const [num, from, to] = element.match(/\d+/g).map(Number);

            let string = "";
            for (let i = 0; i < num; i++) {
                stacks1[to - 1].push(stacks1[from - 1].pop());
                string = stacks2[from - 1].pop() + string;
            }
            stacks2[to - 1].push(...string.split(""));
        });

    let part1 = "";
    for (const stack of stacks1) {
        part1 += stack.pop();
    }

    let part2 = "";
    for (const stack of stacks2) {
        part2 += stack.pop();
    }

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
