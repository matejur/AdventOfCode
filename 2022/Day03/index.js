function toPriority(char) {
    const ascii = char.charCodeAt(0);
    return ascii > 96 ? ascii - 96 : ascii - 38;
}

export function solve(input) {
    
    let priorities1 = 0;
    input.split("\n").forEach((line) => {
        const n = line.length;
        const comp1 = line.slice(0, n / 2);
        const comp2 = line.slice(n / 2, n);

        for (let char of comp1) {
            if (comp2.includes(char)) {
                priorities1 += toPriority(char);
                break;
            }
        }
    });

    let priorities2 = 0;
    input.match(/.*\n.*\n.*\n?/g).forEach((group) => {
        let [r1, r2, r3] = group.split("\n");

        for (let char of r1) {
            if (r2.includes(char) && r3.includes(char)) {
                priorities2 += toPriority(char);
                break;
            }
        }
    });

    console.log("Part 1: " + priorities1);
    console.log("Part 2: " + priorities2);
}
