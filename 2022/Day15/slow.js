function checkPoint(x, y, sensors) {
    if (x < 0 || x > 4000000 || y < 0 || y > 4000000) {
        return false;
    }

    for (let sensor of sensors) {
        const [sx, sy, d] = sensor;

        if (Math.abs(sx - x) + Math.abs(sy - y) <= d) {
            return false;
        }
    }

    return true;
}

function part2(sensors) {
    for (let sensor of sensors) {
        let [x, y, d] = sensor;
        d++;

        x -= d;

        for (let i = 0; i < d; i++) {
            if (checkPoint(x, y, sensors)) {
                return x * 4000000 + y;
            }
            x++;
            y--;
        }

        for (let i = 0; i < d; i++) {
            if (checkPoint(x, y, sensors)) {
                return x * 4000000 + y;
            }
            x++;
            y++;
        }

        for (let i = 0; i < d; i++) {
            if (checkPoint(x, y, sensors)) {
                return x * 4000000 + y;
            }
            x--;
            y++;
        }

        for (let i = 0; i < d; i++) {
            if (checkPoint(x, y, sensors)) {
                return x * 4000000 + y;
            }
            x--;
            y--;
        }
    }
}

export function solve(input) {
    const row = 2000000;
    const set = new Set();
    const sensors = [];

    input.split("\n").forEach((line) => {
        const [sx, sy, bx, by] = line.match(/-?\d+/g).map(Number);

        const manhattan = Math.abs(sx - bx) + Math.abs(sy - by);
        sensors.push([sx, sy, manhattan]);

        if (Math.abs(sy - manhattan) < row) {
            const d = manhattan - Math.abs(sy - row);

            for (let i = sx - d; i < sx + d; i++) {
                set.add(i);
            }
        }
    });

    console.log("Part 1: " + set.size);
    console.log("Part 2: " + part2(sensors));
}
