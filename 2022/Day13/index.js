function rightOrder(left, right) {
    if (Number.isInteger(left) && Number.isInteger(right)) {
        if (left === right) return;
        return left < right;
    }

    if (Array.isArray(left) && Array.isArray(right)) {
        for (let i = 0; i < Math.min(left.length, right.length); i++) {
            const result = rightOrder(left[i], right[i]);
            if (result !== undefined) {
                return result;
            }
        }

        if (left.length === right.length) return;
        return left.length < right.length;
    }

    return Array.isArray(left)
        ? rightOrder(left, [right])
        : rightOrder([left], right);
}

export function solve(input) {
    let part1 = 0;
    const allPackets = [];
    input.split("\n\n").forEach((pair, i) => {
        const [left, right] = pair.split("\n").map(JSON.parse);
        allPackets.push(left, right);

        if (rightOrder(left, right)) {
            part1 += i + 1;
        }
    });

    allPackets.push([[2]], [[6]]);
    allPackets.sort((a, b) => rightOrder(a, b) ? -1 : 1);
    
    let part2 = 1;
    for (let i = 0; i < allPackets.length; i++) {
        const packet = allPackets[i];

        if (packet.length === 1 && packet[0].length === 1 && (packet[0][0] === 2 || packet[0][0] === 6)) {
            part2 *= i + 1;
        }
    }

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
