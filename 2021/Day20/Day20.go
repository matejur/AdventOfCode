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

func print(board [][]rune) {
	for i := range board {
		fmt.Println(string(board[i]))
	}
}

func getIndex(board [][]rune, i, j int) int {
	sum := 0
	power := 8

	for dy := -1; dy < 2; dy++ {
		for dx := -1; dx < 2; dx++ {
			x := j + dx
			y := i + dy
			if board[y][x] == '#' {
				sum += 1 << power
			}
			power--
		}
	}

	return sum
}

func makeBoard(n int) [][]rune {
	newBoard := make([][]rune, n)
	for i := range newBoard {
		newBoard[i] = make([]rune, n)
		for j := range newBoard[i] {
			newBoard[i][j] = '.'
		}
	}
	return newBoard
}

func enhance(board [][]rune, alg string) [][]rune {
	n := len(board)
	newBoard := makeBoard(n + 2)

	for i := 1; i < len(board)-1; i++ {
		for j := 1; j < len(board[i])-1; j++ {
			index := getIndex(board, i, j)
			newBoard[i+1][j+1] = rune(alg[index])
		}
	}
	return newBoard
}

func count(grid [][]rune, offset int) int {
	sum := 0
	for i := offset; i < len(grid)-offset; i++ {
		for j := offset; j < len(grid[i])-offset; j++ {
			if grid[i][j] == '#' {
				sum++
			}
		}
	}
	return sum
}

func main() {
	file, err := os.Open("in20.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	scanner.Scan()
	algorithm := scanner.Text()

	var grid [][]rune

	offset := 200
	y := offset / 2

	scanner.Scan()
	for scanner.Scan() {
		line := []rune(scanner.Text())

		n := len(line) + offset

		if len(grid) == 0 {
			grid = makeBoard(n)
		}

		for i, r := range line {
			grid[y][offset/2+i] = r
		}
		y++
	}
	for i := 0; i < 50; i++ {
		grid = enhance(grid, algorithm)

		if i == 1 {
			fmt.Printf("Part 1: %d\n", count(grid, offset/2))
		}
	}

	fmt.Printf("Part 2: %d\n", count(grid, offset/2))
}
