function isVisible(grid, y, x) {
    const treeHeight = grid[y][x];
    const width = grid[y].length;
    const height = grid.length;

    const dir = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];

    let scenicScore = 1;
    let visible = false;
    dir.forEach((d) => {
        const [dy, dx] = d;

        let nx = x;
        let ny = y;
        let multi = 0;
        while (true) {
            nx += dx;
            ny += dy;

            if (nx < 0 || nx >= width || ny < 0 || ny >= height) {
                visible = true;
                scenicScore *= multi;
                break;
            }

            multi += 1;
            if (grid[ny][nx] >= treeHeight) {
                scenicScore *= multi;
                break;
            }
        }
    });

    return [visible, scenicScore];
}

export function solve(input) {
    const grid = input
        .trim()
        .split("\n")
        .map((line) => line.split("").map(Number));

    let part1 = 0;
    let part2 = 0;
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const [visible, score] = isVisible(grid, i, j);
            part1 += visible;

            if (score > part2) {
                part2 = score;
            }
        }
    }

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
