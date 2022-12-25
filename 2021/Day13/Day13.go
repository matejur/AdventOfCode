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

type coord struct {
	x, y int
}

func exists(c coord, arr []coord) bool {
	for _, i := range arr {
		if c.x == i.x && c.y == i.y {
			return true
		}
	}
	return false
}

func foldY(coords []coord, num int) []coord {
	var newCoords []coord

	for _, c := range coords {
		var new coord
		if c.y < num {
			new = c
		} else {
			new = coord{x: c.x, y: 2*num - c.y}
		}
		if !exists(new, newCoords) {
			newCoords = append(newCoords, new)
		}
	}

	return newCoords
}

func foldX(coords []coord, num int) []coord {
	var newCoords []coord

	for _, c := range coords {
		var new coord
		if c.x < num {
			new = c
		} else {
			new = coord{x: 2*num - c.x, y: c.y}
		}
		if !exists(new, newCoords) {
			newCoords = append(newCoords, new)
		}
	}

	return newCoords
}

func printGrid(coords []coord, maxX, maxY int) {
	grid := make([][]string, maxY)
	for i := range grid {
		grid[i] = make([]string, maxX)
		for j := range grid[i] {
			grid[i][j] = "."
		}
	}

	for _, c := range coords {
		grid[c.y][c.x] = "#"
	}

	fmt.Println("Part 2:")
	for i := range grid {
		fmt.Println(grid[i])
	}
}

func main() {
	file, err := os.Open("in13.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var coords []coord
	maxX := 0
	maxY := 0
	instructions := false
	first := true
	for scanner.Scan() {
		line := scanner.Text()

		if !instructions {
			if line == "" {
				instructions = true
				continue
			}
			x, y := 0, 0
			fmt.Sscanf(line, "%d,%d", &x, &y)
			coords = append(coords, coord{x: x, y: y})
		} else {
			instruction := strings.Split(line, " ")[2]
			number := 0
			fmt.Sscanf(instruction[1:], "=%d", &number)

			if instruction[0] == 'x' {
				coords = foldX(coords, number)
				maxX = number
			} else {
				coords = foldY(coords, number)
				maxY = number
			}

			if first {
				first = false
				fmt.Printf("Part 1: %d\n", len(coords))
			}
		}
	}
	printGrid(coords, maxX, maxY)
}
