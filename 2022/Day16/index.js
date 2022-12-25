class Node {
    constructor(name, flowRate) {
        this.name = name;
        this.flowRate = flowRate;
        this.index = 0;
    }

    addNext(next) {
        this.next = next.map((node) => [node, 1]);
    }
}

const cache = {};
let nodeMap;
function search(node, remainingTime, opened, elephant) {
    if (remainingTime <= 0) {
        if (elephant) {
            return search(nodeMap.get("AA"), 26, opened, false);
        }
        return 0;
    }

    const key = `${node.name}${elephant}${remainingTime}-${opened}`;
    if (key in cache) return cache[key];

    let best = 0;
    for (let next of node.next) {
        const [node, dist] = next;
        best = Math.max(
            best,
            search(node, remainingTime - dist, opened, elephant)
        );
    }

    if ((opened & node.index) === 0 && node.flowRate > 0) {
        opened |= node.index;
        remainingTime--;

        const pressure = node.flowRate * remainingTime;

        for (let next of node.next) {
            const [node, dist] = next;
            best = Math.max(
                best,
                pressure + search(node, remainingTime - dist, opened, elephant)
            );
        }
    }

    cache[key] = best;
    return best;
}

function reduceNodes(nodes) {
    while (nodes.some((node) => node.flowRate === 0 && node.name !== "AA")) {
        const currNode = nodes.find(
            (node) => node.flowRate === 0 && node.name !== "AA"
        );

        for (let next1 of currNode.next) {
            const [name, dist] = next1;
            const nextNode = nodes.find((node) => node.name === name);

            if (!nextNode) continue;

            for (let next2 of currNode.next) {
                const [name2, dist2] = next2;

                if (name === name2) continue;

                const neighbour = nextNode.next.find(
                    (node) => node[0] === name2
                );
                if (neighbour) {
                    if (neighbour[1] > dist + dist2) {
                        neighbour[1] = dist + dist2;
                    }
                } else {
                    nextNode.next.push([name2, dist + dist2]);
                }
            }
        }
        nodes.splice(nodes.indexOf(currNode), 1);
    }

    nodeMap = new Map();
    nodes.forEach((node) => {
        nodeMap.set(node.name, node);
    });

    nodes.forEach((node, i) => {
        node.next = node.next.filter((next) => nodeMap.has(next[0]));
        node.next = node.next.map((next) => [nodeMap.get(next[0]), next[1]]);
        node.index = 1 << i;
    });

    return nodeMap;
}

export function solve(input) {
    const nodes = [];

    input.split("\n").forEach((line, i) => {
        const valves = line.match(/[A-Z]{2}/g);
        const flowRate = parseInt(line.match(/\d+/g)[0]);

        const name = valves.shift();
        const node = new Node(name, flowRate);
        node.addNext(valves);
        nodes.push(node);
    });

    const nodeMap = reduceNodes(nodes);
    console.log("Part 1: " + search(nodeMap.get("AA"), 30, 0, false));
    console.log("Part 2: " + search(nodeMap.get("AA"), 26, 0, true));
}
