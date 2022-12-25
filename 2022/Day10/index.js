function checkCycle(cycle, X) {
    if (cycle % 40 === 20) {
        return cycle * X;
    }
    return 0;
}

function drawPixel(screen, cycle, X) {
    const pixel = cycle % 40;
    if (pixel < X + 2 && pixel > X - 2) {
        screen[cycle] = "#";
    }
}

export function solve(input) {
    let X = 1;
    let cycle = 0;
    let part1 = 0;

    const screen = new Array(240).fill(" ");

    input.split(/[\n ]/).forEach((op) => {
        drawPixel(screen, cycle, X);
        cycle += 1;
        part1 += checkCycle(cycle, X);

        let number = parseInt(op);
        if (number) {
            X += number;
        }
    });

    console.log("Part 1: " + part1);
    console.log("Part 2: ");
    for (let i = 0; i < screen.length; i += 40) {
        console.log(screen.slice(i, i + 40).join(""));
    }
}
