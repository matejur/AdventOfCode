package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func countByWindow(numbers []int, window int) int {
	counter := 0
	for i := window; i < len(numbers); i++ {
		if numbers[i] > numbers[i-window] {
			counter++
		}
	}
	return counter
}

func main() {
	file, err := os.Open("in1.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var numbers []int

	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		check(err)
		numbers = append(numbers, num)
	}

	file.Close()

	fmt.Printf("Part1: %d\n", countByWindow(numbers, 1))
	fmt.Printf("Part2: %d\n", countByWindow(numbers, 3))
}
