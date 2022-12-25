const caveHeight = 20000;
const grid = new Array(caveHeight).fill(0).map(() => new Array(7).fill("."));

const shapes = [
    ["....".split(""), "....".split(""), "....".split(""), "####".split("")],
    ["....".split(""), ".#..".split(""), "###.".split(""), ".#..".split("")],
    ["....".split(""), "..#.".split(""), "..#.".split(""), "###.".split("")],
    ["#...".split(""), "#...".split(""), "#...".split(""), "#...".split("")],
    ["....".split(""), "....".split(""), "##..".split(""), "##..".split("")],
];

function colide(shape, x, y) {
    for (let i = 0; i < shape.length; i++) {
        for (let j = 0; j < shape[i].length; j++) {
            if (shape[3 - i][j] === "#" && grid[y - i][x + j] === "#") {
                return true;
            }
        }
    }
    return false;
}

function placeShape(shape, x, y) {
    for (let i = 0; i < shape.length; i++) {
        for (let j = 0; j < shape[i].length; j++) {
            if (shape[3 - i][j] === "#") {
                grid[y - i][x + j] = "#";
            }
        }
    }
}

const hashes = new Map();
export function solve(jets) {
    let shapeIndex = 0;
    let jetIndex = 0;
    const shapeHeight = [1, 3, 3, 4, 2];
    const shapeWidth = [4, 3, 3, 1, 2];

    let maxHeight = caveHeight - 1;
    let simulationHeight = 0;
    let gotLoop = false;

    const loops = 4000;
    let part1 = false;

    for (let i = 0; i < loops; i++) {
        const shape = shapes[shapeIndex];
        const shapeH = shapeHeight[shapeIndex];
        const shapeW = shapeWidth[shapeIndex];
        let shapeX = 2;
        let shapeY = maxHeight - 3;

        while (true) {
            const jet = jets[jetIndex];
            jetIndex = (jetIndex + 1) % jets.length;
            if (
                jet === "<" &&
                shapeX > 0 &&
                !colide(shape, shapeX - 1, shapeY)
            ) {
                shapeX--;
            } else if (
                jet === ">" &&
                shapeX + shapeW < 7 &&
                !colide(shape, shapeX + 1, shapeY)
            ) {
                shapeX++;
            }

            if (shapeY < caveHeight - 1 && !colide(shape, shapeX, shapeY + 1)) {
                shapeY++;
            } else {
                placeShape(shape, shapeX, shapeY);
                maxHeight = Math.min(maxHeight, shapeY - shapeH);
                break;
            }
        }
        shapeIndex = (shapeIndex + 1) % shapes.length;

        if (i === 2021 && !part1) {
            console.log("Part 1: " + (caveHeight - maxHeight - 1));
            part1 = true;
        }

        if (!gotLoop && maxHeight < caveHeight - 2) {
            const key = `${shapeIndex},${jetIndex},${grid[maxHeight + 1].join(
                ""
            )}`;
            if (hashes.has(key)) {
                const [prevHeight, prevRocks] = hashes.get(key);
                const rocksDiff = i - prevRocks;
                const heightDiff = prevHeight - maxHeight;

                // naredil sem že 1 loop, do zdej sm porabil i kamnou
                const remainingLoops = (1000000000000 - i) / rocksDiff - 1;
                simulationHeight = Math.floor(remainingLoops) * heightDiff;

                // tolk kamnou je padlo med loopom
                const rocksDuringSim = Math.floor(remainingLoops) * rocksDiff;

                // tolk kamnou mi ostane
                const remainingRocks = 1000000000000 - i - rocksDuringSim;

                // smh, nastavljam i znotraj for loopa, Fürst bi rekel FUJ!
                i = loops - remainingRocks;

                gotLoop = true;
            } else {
                hashes.set(key, [maxHeight, i]);
            }
        }
    }

    console.log("Part 2: " + (caveHeight - maxHeight - 1 + simulationHeight));
}
