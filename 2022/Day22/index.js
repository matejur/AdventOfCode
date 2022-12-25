function wrap(grid, x, y, nx, ny, dx, dy) {
    if (ny >= grid.length || dy === 1 && grid[ny][nx] === " ") {
        for (let i = 0; i < grid.length; i++) {
            if (grid[i][nx] === ".") {
                ny = i;
                return [nx, ny, [dx, dy]];
            }
            if (grid[i][nx] === "#") {
                return [x, y, [dx, dy]];
            }
        }
    }

    if (ny < 0 || dy === -1 && grid[ny][nx] === " ") {
        for (let i = grid.length - 1; i >= 0; i--) {
            if (grid[i][nx] === ".") {
                ny = i;
                return [nx, ny, [dx, dy]];
            }
            if (grid[i][nx] === "#") {
                return [x, y, [dx, dy]];
            }
        }
    }

    if (nx >= grid[ny].length || dx === 1 && grid[ny][nx] === " ") {
        for (let i = 0; i < grid[ny].length; i++) {
            if (grid[y][i] === ".") {
                nx = i;
                return [nx, ny, [dx, dy]];
            }
            if (grid[y][i] === "#") {
                return [x, y, [dx, dy]];ny = nx + 100
            }
        }
    }

    if (nx < 0 || dx === -1 && grid[ny][nx] === " ") {
        for (let i = grid[ny].length - 1; i >= 0; i--) {
            if (grid[ny][i] === ".") {
                nx = i;
                return [nx, ny, [dx, dy]];
            }
            if (grid[ny][i] === "#") {
                return [x, y, [dx, dy]];
            }
        }
    }
}

function wrapCube(grid, x, y, nx, ny, dx, dy) {
    let cdy = dy;
    let cdx = dx;

    if (ny < 0 && 50 <= nx && nx < 100 && dy === -1) {
        dy = 0
        dx = 1
        ny = nx + 100
        nx = 0
    } else if (nx < 0 && 150 <= ny && ny < 200 && dx === -1) {
        dy = 1
        dx = 0
        nx = ny - 100
        ny = 0
    } else if (ny < 0 && 100 <= nx && nx < 150 && dy === -1) {
        ny = 199
        nx = nx - 100
    } else if (ny >= 200 && 0 <= nx && nx < 50 && dy === 1) {
        ny = 0
        nx = nx + 100
    } else if (nx >= 150 && 0 <= ny && ny < 50 && dx === 1) {
        dx = -1
        dy = 0
        ny = 149 - ny
        nx = 99
    } else if (nx === 100 && 100 <= ny && ny < 150 && dx === 1) {
        dx = -1
        dy = 0
        ny = 149 - ny
        nx = 149
    } else if (ny === 50 && 100 <= nx && nx < 150 && dy === 1) {
        dy = 0
        dx = -1
        ny = nx - 50
        nx = 99
    } else if (nx === 100 && 50 <= ny && ny < 100 && dx === 1) {
        dy = -1
        dx = 0
        nx = ny + 50
        ny = 49
    } else if (ny === 150 && 50 <= nx && nx < 100 && dy === 1) {
        dy = 0
        dx = -1
        ny = nx + 100
        nx = 49
    } else if (nx === 50 && 150 <= ny && ny < 200 && dx === 1) {
        dy = -1
        dx = 0
        nx = ny - 100
        ny = 149
    } else if (ny === 99 && 0 <= nx && nx < 50 && dy === -1) {
        dy = 0
        dx = 1
        ny = nx + 50
        nx = 50
    } else if (nx === 49 && 50 <= ny && ny < 100 && dx === -1) {
        dy = 1
        dx = 0
        nx = ny - 50
        ny = 100
    } else if (nx === 49 && 0 <= ny && ny < 50 && dx === -1) {
        dy = 0
        dx = 1
        ny = 149 - ny
        nx = 0
    } else if (nx < 0 && 100 <= ny && ny < 150 && dx === -1) {
        dy = 0
        dx = 1
        ny = 149 - ny
        nx = 50
    }

    if (grid[ny][nx] === "#") {
        return [x, y, [cdx, cdy]];
    } 

    return [nx, ny, [dx, dy]];
}
function move(grid, x, y, facing, part1) {
    const [dx, dy] = facing;
    let [nx, ny] = [x + dx, y + dy];

    if (part1) {
        let wrapped = wrap(grid, x, y, nx, ny, dx, dy);
        if (wrapped) return wrapped;
    } else {
        let wrapped = wrapCube(grid, x, y, nx, ny, dx, dy);
        if (wrapped) return wrapped;
    }

    if (grid[ny][nx] === "#") {
        return [x, y, facing];
    }

    return [nx, ny, facing];
}

// U, R, D, L
const facings = [[0, -1], [1, 0], [0, 1], [-1, 0]];
const values = [3, 0, 1, 2];

function findPassword(grid, instructions, x, y, part1) {
    const dists = instructions.match(/\d+/g).map(Number);
    const dirs = instructions.match(/[A-Z]/g);

    let facingId = 1;
    let facing = facings[facingId];

    for (let i = 0; i < dists.length; i++) {
        const dist = dists[i];
        const rotation = dirs[i];
        
        for (let j = 0; j < dist; j++) {
            [x, y, facing] = move(grid, x, y, facing, part1);
        }

        facingId = facings.findIndex(f => f[0] === facing[0] && f[1] === facing[1])
        if (rotation === "L") {
            facingId = (facingId + 3) % 4;
        }
        if (rotation === "R") {
            facingId = (facingId + 1) % 4;
        }
        facing = facings[facingId];
    }

    return (y + 1) * 1000 + (x + 1) * 4 + values[facingId];
}

export function solve(input) {
    const [map, instructions] = input.split("\n\n");
    const rows = map.split("\n");
    const height = rows.length;
    const width = Math.max(...rows.map(row => row.length));

    const grid = new Array(height).fill(0).map(() => new Array(width).fill(" "));

    let cx = 0;
    let cy = 0;
    let found = false;
    for (let y = 0; y < height; y++) {
        const row = rows[y];
        for (let x = 0; x < row.length; x++) {
            grid[y][x] = row[x];

            if (!found && y === 0 && row[x] === ".") {
                cx = x;
                found = true;
            }
        }
    }

    console.log("Part 1: " + findPassword(grid, instructions, cx, cy, true));
    console.log("Part 2: " + findPassword(grid, instructions, cx, cy, false));
}