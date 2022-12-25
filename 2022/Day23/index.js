function drawGrid(grid) {
    const xs = [];
    const ys = [];

    grid.forEach((coord) => {
        const [x, y] = coord.split(",").map(Number);
        xs.push(x);
        ys.push(y);
    });

    const minX = Math.min(...xs);
    const maxX = Math.max(...xs);
    const minY = Math.min(...ys);
    const maxY = Math.max(...ys);

    for (let y = minY; y <= maxY; y++) {
        let line = "";
        for (let x = minX; x <= maxX; x++) {
            line += grid.has(`${x},${y}`) ? "#" : ".";
        }
        console.log(line);
    }
    console.log();
}

function calculateEmpty(grid) {
    const xs = [];
    const ys = [];

    grid.forEach((coord) => {
        const [x, y] = coord.split(",").map(Number);
        xs.push(x);
        ys.push(y);
    });

    const minX = Math.min(...xs);
    const maxX = Math.max(...xs);
    const minY = Math.min(...ys);
    const maxY = Math.max(...ys);

    return (maxX - minX + 1) * (maxY - minY + 1) - grid.size;
}

function checkNeighbours(grid, x, y) {
    let neighbours = 0;
    for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
            if (i === 0 && j === 0) continue;
            if (grid.has(`${x + i},${y + j}`)) neighbours++;
        }
    }
    return neighbours;
}

const directions = [
    {
        check: [
            [-1, -1],
            [0, -1],
            [1, -1],
        ],
        go: [0, -1],
    },
    {
        check: [
            [-1, 1],
            [0, 1],
            [1, 1],
        ],
        go: [0, 1],
    },
    {
        check: [
            [-1, -1],
            [-1, 0],
            [-1, 1],
        ],
        go: [-1, 0],
    },
    {
        check: [
            [1, -1],
            [1, 0],
            [1, 1],
        ],
        go: [1, 0],
    },
];

function changed(s1, s2) {
    if (s1.size !== s2.size) return true;
    return ![...s1].every((coord) => s2.has(coord));
}

export function solve(input) {
    let grid = new Set();

    input.split("\n").forEach((line, y) => {
        line.split("").forEach((char, x) => {
            if (char === "#") {
                grid.add(`${x},${y}`);
            }
        });
    });
    
    let round = 0;
    while (true) {
        round++;
        const proposed = {};
        const newGrid = new Set();
        
        for (let elf of grid) {
            const [x, y] = elf.split(",").map(Number);
            if (checkNeighbours(grid, x, y)) {
                for (const {check, go} of directions) {
                    if (check.some(([dx, dy]) => grid.has(`${x + dx},${y + dy}`))) continue;
                    const [dx, dy] = go;
                    const key = `${x + dx},${y + dy}`;
                    if (proposed[key]) {
                        proposed[key]++;
                    } else {
                        proposed[key] = 1;
                    }
                    break;
                }
            }
        }

        for (let elf of grid) {
            const [x, y] = elf.split(",").map(Number);
            if (checkNeighbours(grid, x, y)) {
                let added = false;
                for (const {check, go} of directions) {
                    if (check.some(([dx, dy]) => grid.has(`${x + dx},${y + dy}`))) continue;
                    const [dx, dy] = go;
                    const key = `${x + dx},${y + dy}`;
                    if (proposed[key] === 1) {
                        newGrid.add(key);
                        added = true;
                    }
                    break;
                }
                if (!added) {
                    newGrid.add(elf);
                }
            } else {
                newGrid.add(elf);
            }
        }
        directions.push(directions.shift());

        if (!changed(grid, newGrid)) {
            console.log("Part 2: " + round);
            break;
        }

        grid = newGrid;
        
        if (round === 10) {
            console.log("Part 1: " + calculateEmpty(grid));
        }
    }
}
