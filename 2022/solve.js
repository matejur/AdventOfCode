// Very janky solver
import { readFileSync } from "fs";

async function solve_day(day) {
    const dayString = day.padStart(2, "0");

    let input;
    let solve;
    try {
        input = readFileSync(`./Day${dayString}/input.txt`, "utf8");
    } catch {
        return
    }
    ({ solve } = await import(`./Day${dayString}/index.js`));
    console.log(`------ Day ${day} ------`);
    if (day === "22") {
        solve(input);
    } else {
        solve(input.trim());
    }
}

const day = process.argv[2];

if (day) {
    solve_day(day);
} else {
    for (let i = 1; i < 26; i++) {
        await solve_day(i.toString());
    }
}
