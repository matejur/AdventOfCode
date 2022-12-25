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

func flash(arr [][]rune, flashed [][]bool, x, y int) int {
	if flashed[x][y] {
		return 0
	}

	sum := 0
	if arr[x][y] > 9 {
		sum++
		flashed[x][y] = true
		for i := -1; i < 2; i++ {
			for j := -1; j < 2; j++ {
				newx := x + i
				newY := y + j
				if !(newx < 0 || newx > len(arr)-1 || newY < 0 || newY > len(arr[x])-1) {
					arr[newx][newY]++
					sum += flash(arr, flashed, newx, newY)
				}
			}
		}
	}
	return sum
}

func notAllFlashed(arr [][]bool) bool {
	for i := range arr {
		for j := range arr[i] {
			if !arr[i][j] {
				return true
			}
		}
	}
	return false
}

func reset(arr [][]rune, flashed [][]bool) {
	for i := range arr {
		for j := range arr {
			if flashed[i][j] {
				arr[i][j] = 0
			}
		}
	}
}

func step(arr [][]rune) {
	for i := range arr {
		for j := range arr[i] {
			arr[i][j]++
		}
	}
}

func main() {
	file, err := os.Open("in11.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var octopuses [][]rune
	for scanner.Scan() {
		line := scanner.Text()
		var row []rune
		for _, i := range line {
			row = append(row, i-'0')
		}
		octopuses = append(octopuses, row)
	}

	flashed := make([][]bool, len(octopuses))
	for i := range flashed {
		flashed[i] = make([]bool, len(octopuses[i]))
	}

	sum := 0
	stepCount := 0
	for notAllFlashed(flashed) {
		stepCount++
		step(octopuses)

		for i := range flashed {
			flashed[i] = make([]bool, len(octopuses[i]))
		}

		for i := range octopuses {
			for j := range octopuses[i] {
				sum += flash(octopuses, flashed, i, j)
			}
		} 
		reset(octopuses, flashed)

		if stepCount == 100 {
			fmt.Printf("Part 1: %d\n", sum)
		}
	}
	fmt.Printf("Part 2: %d\n", stepCount)
}
