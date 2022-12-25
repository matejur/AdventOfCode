const dirs = [
    [0, 0, 1],
    [0, 0, -1],
    [0, 1, 0],
    [0, -1, 0],
    [1, 0, 0],
    [-1, 0, 0],
];

function cubeArea(x, y, z, grid) {
    let area = 6;
    for (let dir of dirs) {
        const [dx, dy, dz] = dir;
        const [nx, ny, nz] = [x + dx, y + dy, z + dz];
        if (grid[nx] && grid[nx][ny] && grid[nx][ny][nz] === 1) {
            area--;
        }
    }
    return area;
}

function getArea(grid) {
    let area = 0;
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            for (let k = 0; k < grid[i][j].length; k++) {
                if (grid[i][j][k] === 1) {
                    area += cubeArea(i, j, k, grid);
                }
            }
        }
    }
    return area;
}

function floodFill(grid, visited) {
    let x = 0;
    let y = 0;
    let z = 0;
    let queue = [[x, y, z]];
    visited[x][y][z] = true;

    let sum = 0;
    while (queue.length) {
        const [x, y, z] = queue.shift();

        for (let dir of dirs) {
            const [dx, dy, dz] = dir;
            const [nx, ny, nz] = [x + dx, y + dy, z + dz];

            if (nx < 0 || ny < 0 || nz < 0) continue;
            if (
                nx >= grid.length ||
                ny >= grid[nx].length ||
                nz >= grid[nx][ny].length
            )
                continue;

            if (grid[nx][ny][nz] === 1) {
                sum += 1;
            } else {
                if (!visited[nx][ny][nz]) {
                    visited[nx][ny][nz] = true;
                    queue.push([nx, ny, nz]);
                }
            }
        }
    }

    return sum;
}

export function solve(input) {
    let n = 30;
    const grid = new Array(n)
        .fill(0)
        .map(() => new Array(n).fill(0).map(() => new Array(n).fill(0)));

    const visited = new Array(n)
        .fill(0)
        .map(() => new Array(n).fill(0).map(() => new Array(n).fill(false)));

    input.split("\n").forEach((cube) => {
        const [x, y, z] = cube.split(",").map(Number);

        grid[x + 1][y + 1][z + 1] = 1;
    });

    console.log("Part 1: " + getArea(grid));
    console.log("Part 2: " + floodFill(grid, visited));
}
