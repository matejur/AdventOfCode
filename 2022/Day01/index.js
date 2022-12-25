export function solve(input) {
    let elves = input.split("\n\n").map((elf) => {
        return elf.split("\n").map((item) => {
            return parseInt(item, 10);
        });
    });

    let calories = elves.map((elf) => {
        return elf.reduce((a, b) => a + b);
    });

    calories.sort((a, b) => b - a);

    console.log("Part 1: " + calories[0]);
    console.log("Part 2: " + (calories[0] + calories[1] + calories[2]));
}
