export function solve(input) {
    let part1 = 0;
    let part2 = 0;

    input.split("\n").forEach((line) => {
        const [a, b, c, d] = line.split(/[-,]/).map(Number);
        if ((a <= c && b >= d) || (a >= c && b <= d)) {
            part1++;
        }

        if (a <= d && b >= c) {
            part2++;
        }
    });

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
