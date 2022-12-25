export function solve(input) {
    const intervals = [];
    const row = 2000000;
    const lines1 = [];
    const lines2 = [];

    input.split("\n").forEach((line) => {
        const [sx, sy, bx, by] = line.match(/-?\d+/g).map(Number);
        const manhattan = Math.abs(sx - bx) + Math.abs(sy - by);

        if (Math.abs(sy - manhattan) < row) {
            const d = manhattan - Math.abs(sy - row);
            intervals.push([sx - d, sx + d]);
        }

        lines1.push(sx - sy - manhattan);
        lines1.push(sx - sy + manhattan);
        lines2.push(sx + sy + manhattan);
        lines2.push(sx + sy - manhattan);
    });

    intervals.sort((a, b) => (a[0] === b[0] ? a[1] - b[1] : a[0] - b[0]));

    const combined = [intervals[0]];
    for (let i = 1; i < intervals.length; i++) {
        const [a, b] = intervals[i];
        const [c, d] = combined[combined.length - 1];

        if (a > d + 1) {
            combined.push([a, b]);
            continue;
        }

        combined[combined.length - 1][1] = Math.max(b, d);
    }

    const part1 = combined.reduce((acc, [a, b]) => acc + b - a, 0);

    let first = 0;
    let second = 0;

    for (let i = 0; i < lines1.length; i++) {
        for (let j = i; j < lines1.length; j++) {
            let first1 = lines1[i];
            let first2 = lines1[j];

            if (Math.abs(first1 - first2) === 2)
                first = (first1 + first2) / 2;

            let second1 = lines2[i];
            let second2 = lines2[j];

            if (Math.abs(second1 - second2) === 2)
                second = (second1 + second2) / 2;
        }
    }

    const x = (first + second) / 2;
    const y = (second - first) / 2;
    console.log("Part 1: " + part1);
    console.log("Part 2: " + (x * 4000000 + y));
}
