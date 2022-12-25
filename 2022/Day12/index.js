function findPath(grid, start, end) {
    const queue = [start];
    const dists = new Array(grid.length).fill(0).map(() => new Array(grid[0].length).fill(0));
    const dirs = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
    ];

    while (queue.length > 0) {
        const { x, y } = queue.shift();

        if (x === end.x && y === end.y) {
            return dists[y][x];
        }

        const curr = grid[y][x];
        for (let dir of dirs) {
            const [dx, dy] = dir;
            const nx = x + dx;
            const ny = y + dy;

            
            if (nx >= 0 && nx < grid[0].length && ny >= 0 && ny < grid.length) {
                if (dists[ny][nx] !== 0) {
                    continue;
                }

                const next = grid[ny][nx];
                if (next > curr + 1) {
                    continue;
                }

                queue.push({ x: nx, y: ny });
                dists[ny][nx] = dists[y][x] + 1;
            }
        }
    }
}

export function solve(input) {
    const grid = input
        .split("\n")
        .map((line) => line.split("").map((c) => c.charCodeAt()));

    let start;
    let end;
    let lowestPoints = [];
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            if (grid[y][x] === "a".charCodeAt()) {
                lowestPoints.push({ x, y });
            }
            
            if (grid[y][x] === "S".charCodeAt()) {
                start = { x, y };
                grid[y][x] = "a".charCodeAt();
            }

            if (grid[y][x] === "E".charCodeAt()) {
                end = { x, y };
                grid[y][x] = "z".charCodeAt();
            }
        }
    }

    console.log("Part 1: " + findPath(grid, start, end));

    const lengths = lowestPoints.map((point) => findPath(grid, point, end));
    console.log("Part 2: " + lengths.sort()[0]);
}
