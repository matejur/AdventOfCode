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

func move(cucumbers [][]rune) ([][]rune, bool) {
	new := make([][]rune, len(cucumbers))
	for i := range new {
		new[i] = make([]rune, len(cucumbers[0]))
		for j := range new[i] {
			new[i][j] = '.'
		}
	}

	var xs []int
	var ys []int
	moved := false
	for y := range cucumbers {
		for x := len(cucumbers[0]) - 1; x > -1; x-- {
			cucumber := cucumbers[y][x]
			if cucumber == '>' {
				newX := x + 1
				if newX == len(cucumbers[0]) {
					newX = 0
				}
				if cucumbers[y][newX] == '.' {
					moved = true
					new[y][newX] = cucumber
					xs = append(xs, x)
					ys = append(ys, y)
				} else {
					new[y][x] = cucumber
				}
			}
		}
	}

	for i := range xs {
		cucumbers[ys[i]][xs[i]] = '.'
	}

	for y := len(cucumbers) - 1; y > -1; y-- {
		for x := range cucumbers[y] {
			cucumber := cucumbers[y][x]
			if cucumber == 'v' {
				newY := y + 1
				if newY == len(cucumbers) {
					newY = 0
				}
				if cucumbers[newY][x] == '.' && new[newY][x] == '.' {
					moved = true
					new[newY][x] = cucumber
				} else {
					new[y][x] = cucumber
				}
			}
		}
	}

	return new, moved
}

func print(arr [][]rune) {
	for i := range arr {
		fmt.Println(string(arr[i]))
	}
}

func main() {
	file, err := os.Open("in25.txt")
	check(err)

	var cucumbers [][]rune

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		cucumbers = append(cucumbers, []rune(scanner.Text()))
	}

	moved := true
	i := 0
	for ; moved; i++ {
		cucumbers, moved = move(cucumbers)
	}

	fmt.Printf("Part 1: %d\n", i)
	fmt.Println("MERRY CHRISTMAS!")
}
