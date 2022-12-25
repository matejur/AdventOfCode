package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type cave struct {
	name         string
	connections  []*cave
	big          bool
	visited      bool
	visitedTwice bool
}

func (cave *cave) addCave(other *cave) {
	cave.connections = append(cave.connections, other)
}

var caveMap = make(map[string]*cave)

func findPaths(cave *cave, twice bool) int {
	if cave.name == "end" {
		return 1
	}

	cave.visited = true
	paths := 0
	for i := range cave.connections {
		next := cave.connections[i]
		if next.name == "start" {
			continue
		}

		if next.big || !next.visited {
			paths += findPaths(next, twice)
		} else if !twice {
			next.visitedTwice = true
			paths += findPaths(next, true)
		}
	}

	if cave.visitedTwice {
		cave.visitedTwice = false
	} else {
		cave.visited = false
	}
	return paths
}

func main() {
	file, err := os.Open("in12.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "-")
		nameA := line[0]
		nameB := line[1]

		caveA, existsA := caveMap[nameA]
		caveB, existsB := caveMap[nameB]

		if !existsA {
			caveA = &cave{name: nameA, big: nameA[0] < 96}
			caveMap[nameA] = caveA
		}

		if !existsB {
			caveB = &cave{name: nameB, big: nameB[0] < 96}
			caveMap[nameB] = caveB
		}

		caveA.addCave(caveB)
		caveB.addCave(caveA)
	}

	start := caveMap["start"]
	fmt.Printf("Part 1: %d\n", findPaths(start, true))
	fmt.Printf("Part 2: %d\n", findPaths(start, false))
}
