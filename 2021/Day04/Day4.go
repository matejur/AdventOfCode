package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func toIntArr(arr []string) []int {
	out := make([]int, len(arr))
	for i, numStr := range arr {
		num, err := strconv.Atoi(numStr)
		check(err)
		out[i] = num
	}
	return out
}

func markBoard(board *board, num int) {
	numbers := board.numbers

	for i := range numbers {
		for j := range numbers[i] {
			if numbers[i][j] == num {
				board.marked[i][j] = true
			}
		}
	}
}

func unmarkedSum(board *board) int {
	sum := 0
	for i, line := range board.numbers {
		for j, num := range line {
			if !board.marked[i][j] {
				sum += num
			}
		}
	}
	return sum
}

func checkBoard(board *board) int {
	for i := range board.numbers {
		row := true
		for j := range board.numbers[i] {
			if !board.marked[i][j] {
				row = false
				break
			}
		}
		if row {
			return unmarkedSum(board)
		}
	}

	for i := range board.numbers {
		column := true
		for j := range board.numbers[i] {
			if !board.marked[j][i] {
				column = false
				break
			}
		}
		if column {
			return unmarkedSum(board)
		}
	}

	return -1
}

type board struct {
	numbers [5][]int
	marked  [5][5]bool
	won bool
}

func main() {
	file, err := os.Open("in4.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	scanner.Scan()

	line := strings.Split(scanner.Text(), ",")
	numbers := toIntArr(line)

	var boards []board
	for scanner.Scan() {
		board := board{}

		for i := 0; i < 5; i++ {
			scanner.Scan()
			line := strings.Fields(scanner.Text())
			board.numbers[i] = toIntArr(line)
		}
		boards = append(boards, board)
	}

	part1 := true
	lastScore := 0
	for _, num := range numbers {
		for i := range boards {
			board := &boards[i]

			if (!board.won) {
				markBoard(board, num)
				sum := checkBoard(board)
	
				if sum > 0 {
					if part1 {
						fmt.Printf("Part1: %d\n", sum*num)
						part1 = false
					}
					board.won = true
					lastScore = sum*num
				}
			}
		}
	}
	fmt.Printf("Part2: %d\n", lastScore)
}
