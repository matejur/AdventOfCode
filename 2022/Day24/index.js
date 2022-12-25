function gcd(a, b) {
    if (!b) {
        return a;
    }

    return gcd(b, a % b);
}

export function solve(input) {
    const blizzards = new Array(4).fill(0).map(i => new Set());

    input.split("\n").forEach((row, i) => {
        if (i === 0) return;
        row.split("").forEach((cell, j) => {
            if (j === 0) return;

            if ("<>^v".includes(cell)) {
                blizzards["<>^v".indexOf(cell)].add(`${i-1},${j-1}`);
            }
        });
    });

    const rows = input.split("\n").length - 2;
    const cols = input.split("\n")[0].length - 2;

    const targets = [[rows, cols - 1], [-1, 0]];
    const seen = new Set();
    const lcm = rows * cols / gcd(rows, cols);
    const queue = [[0, -1, 0, 0]]

    let part1 = false;

    while (queue.length) {
        let [time, cr, cc, stage] = queue.shift();

        let nstage = stage
        time += 1;

        for (const [dr, dc] of [[0, 1], [0, -1], [-1, 0], [1, 0], [0, 0]]) {
            const [nr, nc] = [cr + dr, cc + dc];

            const target = targets[stage % 2];
            if (nr === target[0] && nc === target[1]) {
                if (stage === 2) {
                    console.log("Part 2: " + time);
                    return;
                }
                if (stage === 0 && !part1) {
                    part1 = true;
                    console.log("Part 1: " + time);
                }
                nstage += 1;
            }

            if ((nr < 0 || nr >= rows || nc < 0 || nc >= cols) && !targets.some(t => t[0] === nr && t[1] === nc)) {
                continue;
            }

            let blocked = false;

            if (!targets.some(t => t[0] === nr && t[1] === nc)) {
                for (const [i, tr, tc] of [[0, 0, -1], [1, 0, 1], [2, -1, 0], [3, 1, 0]]) {
                    const r = ((nr - tr * time) % rows + rows) % rows;
                    const c = ((nc - tc * time) % cols + cols) % cols;
    
                    if (blizzards[i].has(`${r},${c}`)) {
                        blocked = true;
                        break;
                    }
                }
            }

            if (!blocked) {
                const key = `${nr},${nc},${stage},${time % lcm}`;

                if (seen.has(key)) {
                    continue;
                }

                seen.add(key);
                queue.push([time, nr, nc, nstage]);
            }
        }
    }

    console.log("Part 2: ");
}