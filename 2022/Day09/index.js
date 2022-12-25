function getDiff(Hx, Hy, Tx, Ty) {
    const xDiff = Hx - Tx;
    const yDiff = Hy - Ty;

    const mX = xDiff === 0 ? 0 : xDiff / Math.abs(xDiff);
    const mY = yDiff === 0 ? 0 : yDiff / Math.abs(yDiff);

    return [mX, mY];
}

function solveSnake(segments, input) {
    const tailPositions = new Set();
    const snake = new Array(segments).fill(null).map((_) => [0, 0]);
    const dirs = {
        R: [0, 1],
        L: [0, -1],
        D: [1, 0],
        U: [-1, 0],
    };

    input.split("\n").forEach((line) => {
        const [dir, length] = [line[0], parseInt(line.slice(1))];

        const [dx, dy] = dirs[dir];
        for (let i = 0; i < length; i++) {
            snake[0][0] += dx;
            snake[0][1] += dy;

            for (let seg = 1; seg < segments; seg++) {
                const [Hx, Hy] = snake[seg - 1];
                const [Tx, Ty] = snake[seg];

                const manhattan = Math.abs(Hx - Tx) + Math.abs(Hy - Ty);
                if (
                    (Hx === Tx && Math.abs(Hy - Ty) > 1) ||
                    (Hy === Ty && Math.abs(Hx - Tx) > 1) ||
                    manhattan > 2
                ) {
                    const [mX, mY] = getDiff(Hx, Hy, Tx, Ty);
                    snake[seg] = [Tx + mX, Ty + mY];
                }
            }
            tailPositions.add(snake[segments - 1].join(","));
        }
    });

    return tailPositions.size;
}

export function solve(input) {
    console.log("Part 1: " + solveSnake(2, input));
    console.log("Part 2: " + solveSnake(10, input));
}
