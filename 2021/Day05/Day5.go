package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func drawLine(floor [][]int, line [4]int) {
	if line[0] == line[2] {
		y1, y2 := line[1], line[3]
		if line[1] > line[3] {
			y1, y2 = line[3], line[1]
		} 
		for y := y1; y <= y2; y++ {
			floor[y][line[0]]++
		}
	}

	if line[1] == line[3] {
		x1, x2 := line[0], line[2]
		if line[0] > line[2] {
			x1, x2 = line[2], line[0]
		} 
		for x := x1; x <= x2; x++ {
			floor[line[1]][x]++
		}
	}
}

func newFloor(x int, y int) [][]int {
	floor := make([][]int, y+1)
	for i := range floor {
		for j := 0; j <= x; j++ {
			floor[i] = append(floor[i], 0)
		}
	}
	return floor
}

func drawLineDiagonal(floor [][]int, line [4]int) {
	if line[0] == line[2] || line[1] == line[3] {
		drawLine(floor, line)
		return
	}
	slope := (line[3] - line[1]) / (line[2] - line[0])

	x1, x2 := 0, 0
	y := 0
	if line[0] < line[2] {
		x1, x2 = line[0], line[2]
		y = line[1]
	} else {
		x1, x2 = line[2], line[0]
		y = line[3]
	}

	for x := x1; x <= x2; x++ {
		floor[y][x]++
		y += slope
	}
}

func count(floor [][]int) int {
	sum := 0
	for _, row := range floor {
		for _, spot := range row {
			if spot > 1 {
				sum++
			}
		}
	}
	return sum
}

func main() {
	file, err := os.Open("in5.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var lines [][4]int
	maxX := 0
	maxY := 0
	for scanner.Scan() {
		points := scanner.Text()
		x1, y1, x2, y2 := 0, 0, 0, 0
		fmt.Sscanf(points, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2)
		if x1 > maxX {
			maxX = x1
		}
		if x2 > maxX {
			maxX = x2
		}
		if y1 > maxY {
			maxY = y1
		}
		if y2 > maxY {
			maxY = y2
		}
		point := [4]int{x1, y1, x2, y2}
		lines = append(lines, point)
	}

	floor1 := newFloor(maxX, maxY)
	floor2 := newFloor(maxX, maxY)
	for _, line := range lines {
		drawLine(floor1, line)
		drawLineDiagonal(floor2, line)
	}
	fmt.Printf("Part 1: %d\n", count(floor1))
	fmt.Printf("Part 2: %d\n", count(floor2))
}
