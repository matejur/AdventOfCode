function findStart(signal, n) {
    for (let i = n; i < signal.length; i++) {
        const substring = signal.substring(i - n, i);
        if (new Set(substring).size === n) {
            return i;
        }
    }
}

export function solve(input) {
    console.log("Part 1: " + findStart(input, 4));
    console.log("Part 2: " + findStart(input, 14));
}