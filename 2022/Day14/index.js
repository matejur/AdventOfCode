function drawLines(grid, points) {
    let prevX = points[0][0];
    let prevY = points[0][1];

    for (let i = 1; i < points.length; i++) {
        let currX = points[i][0];
        let currY = points[i][1];

        for (let x = Math.min(prevX, currX); x <= Math.max(prevX, currX); x++) {
            for (let y = Math.min(prevY, currY); y <= Math.max(prevY, currY); y++) {
                grid[y][x] = "#";
            }
        }
        prevX = currX;
        prevY = currY;
    }
}

function dropSand(grid) {
    let sandX = 500;
    let sandY = 0;

    while (sandY < grid.length - 1) {
        if (grid[sandY + 1][sandX] === ".") {
            sandY++;
            continue;
        }

        if (grid[sandY + 1][sandX - 1] === ".") {
            sandX--;
            sandY++;
            continue;
        }

        if (grid[sandY + 1][sandX + 1] === ".") {
            sandX++;
            sandY++;
            continue;
        }

        if (sandX === 500 && sandY === 0) {
            return false;
        }

        grid[sandY][sandX] = "O";
        return true;
    }

    return false;
}

export function solve(input) {
    const grid = new Array(200).fill(0).map(() => new Array(1000).fill("."));

    let maxY = 0;
    input.split("\n").forEach((line) => {
        const segments = line.split(" -> ").map((s) => s.split(",").map(Number));
        drawLines(grid, segments);

        segments.forEach((point) => {
            if (point[1] > maxY) maxY = point[1];
        });
    });

    let part1 = 0;
    while (dropSand(grid)) {
        part1++;
    }
    drawLines(grid, [[0, maxY + 2], [grid[0].length, maxY + 2]]);

    let part2 = part1 + 1;
    while (dropSand(grid)) {
        part2++;
    }

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
