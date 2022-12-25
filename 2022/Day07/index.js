class Directory {
    constructor(name) {
        this.name = name;
        this.dirs = [];
        this.size = 0;
        this.parrent = null;
    }

    addFile(size) {
        this.size += size;

        if (this.parent) {
            this.parent.addFile(size);
        }
    }

    addDir(name) {
        let newDir = new Directory(name);
        newDir.parent = this;
        this.dirs.push(newDir);
    }

    changeDir(name) {
        if (name === "..") return this.parent;

        return this.dirs.filter((dir) => dir.name === name)[0];
    }

    getSum(atMost) {
        let sum = 0;
        if (this.size <= atMost) sum += this.size;

        for (let dir of this.dirs) {
            sum += dir.getSum(atMost);
        }

        return sum;
    }

    getCandidates(needAtleast) {
        let candidates = [];
        if (this.size >= needAtleast) candidates.push(this.size);

        for (let dir of this.dirs) {
            candidates.push(...dir.getCandidates(needAtleast));
        }

        return candidates;
    }
}

class FileSystem {
    constructor() {
        this.root = new Directory("/");
        this.current = this.root;
    }

    addFile(size) {
        this.current.addFile(size);
    }

    addDirectory(name) {
        this.current.addDir(name);
    }

    changeDir(name) {
        if (name === "/") this.current = this.root;
        else this.current = this.current.changeDir(name);
    }

    getSum(atMost) {
        return this.root.getSum(atMost);
    }

    getCandidates(needAtleast) {
        return this.root.getCandidates(needAtleast);
    }
}

export function solve(input) {
    const fs = new FileSystem();

    input
        .trim()
        .split("\n")
        .forEach((line) => {
            if (line[0] === "$") {
                const cmd = line.split(" ");

                if (cmd[1] === "cd") {
                    fs.changeDir(cmd[2]);
                }
            } else {
                if (line[0] === "d") {
                    fs.addDirectory(line.split(" ")[1]);
                } else {
                    fs.addFile(parseInt(line.split(" ")[0]));
                }
            }
        });

    const totalSize = 70000000;
    const requiredSpace = 30000000;
    const remainingSpace = totalSize - fs.root.size;
    const needAtleast = requiredSpace - remainingSpace;

    const candidates = fs.getCandidates(needAtleast);

    console.log("Part 1: " + fs.getSum(100000));
    console.log("Part 2: " + Math.min(...candidates));
}
