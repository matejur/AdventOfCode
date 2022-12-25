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

func alignToPos(positions []int, target int, minFuel int, f func(int) int) int {
	fuel := 0
	for _, pos := range positions {
		diff := target - pos
		if diff < 0 {
			diff *= -1
		}
		fuel += f(diff)
		if fuel > minFuel {
			return minFuel
		}
	}
	return fuel
}

func cost1(c int) int {
	return c
}

func cost2(c int) int {
	return c * (c + 1) / 2
}

func main() {
	file, err := os.Open("in7.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	positions := toIntArr(strings.Split(line, ","))

	maxPos := 0
	for i, pos := range positions {
		if i == 0 || pos > maxPos {
			maxPos = pos
		}
	}

	minFuel1 := 9999999999
	minFuel2 := 9999999999
	for i := 0; i < maxPos; i++ {
		minFuel1 = alignToPos(positions, i, minFuel1, cost1)
		minFuel2 = alignToPos(positions, i, minFuel2, cost2)
	}

	fmt.Printf("Part 1: %d\n", minFuel1)
	fmt.Printf("Part 2: %d\n", minFuel2)
}
