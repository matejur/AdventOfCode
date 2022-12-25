// thanks to https://www.youtube.com/watch?v=H3PSODv4nf0 for some optimizations

function dfs(blueprint, maxSpend, cache, time, bots, resources) {
    if (time === 0) return resources[3];

    const key = `${time}-${bots}-${resources}`;
    if (cache[key]) return cache[key];

    // if we don't build any bots, this is the best we can get
    let best = resources[3] + bots[3] * time;

    // try to build each bot
    for (let robotId = 0; robotId < 4; robotId++) {
        const recipie = blueprint[robotId];

        // if we already have enough bots, dont build more
        if (robotId != 3 && bots[robotId] >= maxSpend[robotId]) continue;

        // how much time do we need to build this robot? can we build it at all?
        let wait = 0;
        let canMake = true;
        for (const [cost, resource] of recipie) {
            if (bots[resource] === 0) {
                canMake = false;
                break;
            }
            wait = Math.max(wait, Math.ceil((cost - resources[resource]) / bots[resource]));
        }

        // we can build it, so go ahead
        if (canMake) {
            const remainingTime = time - wait - 1;

            if (remainingTime <= 0) continue;

            const tmpBots = [...bots];
            const tmpResources = [...resources];

            // number of resources after building the robot
            for (let i = 0; i < 4; i++) {
                tmpResources[i] += bots[i] * (wait + 1);
            }
            
            for (const [cost, resource] of recipie) {
                tmpResources[resource] -= cost;
            }
            
            tmpBots[robotId]++;
            
            // if we have more resources that we need, we can cap them, so that the cache hits more often
            for(let i = 0; i < 3; i++) {
                tmpResources[i] = Math.min(maxSpend[i] * remainingTime, tmpResources[i]);
            }

            best = Math.max(best, dfs(blueprint, maxSpend, cache, remainingTime, tmpBots, tmpResources));
        }
    }

    cache[key] = best;
    return best;
}

export function solve(input) {
    let part1 = 0;
    let part2 = 1;
    input.split("\n").forEach((line) => {
        const [id, oreCost, clayCost, obsidianCostOre, obsidianCostClay, geodeCostOre, geodeCostObsidian] = line.match(/\d+/g).map(Number);
        
        const blueprint = [[[oreCost, 0]], [[clayCost, 0]], [[obsidianCostOre, 0], [obsidianCostClay, 1]], [[geodeCostOre, 0], [geodeCostObsidian, 2]]];
        const maxSpend = [Math.max(oreCost, clayCost, obsidianCostOre, geodeCostOre), obsidianCostClay, geodeCostObsidian]

        const value = dfs(blueprint, maxSpend, {}, 24, [1, 0, 0, 0], [0, 0, 0, 0])
        part1 += value * id;

        if (id < 4) {
            part2 *= dfs(blueprint, maxSpend, {}, 32, [1, 0, 0, 0], [0, 0, 0, 0])
        }
    });

    console.log("Part 1: " + part1);
    console.log("Part 2: " + part2);
}
