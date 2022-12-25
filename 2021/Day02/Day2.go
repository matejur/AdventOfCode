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

func main() {
	file, err := os.Open("in2.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanWords)

	x1 := 0
	y1 := 0
	aim := 0
	x2 := 0
	y2 := 0
	for scanner.Scan() {
		dir := scanner.Text()
		scanner.Scan()
		num, err := strconv.Atoi(scanner.Text())
		check(err)

		if dir == "forward" {
			x1 += num
			x2 += num
			y2 += aim * num
		} else if dir == "down" {
			y1 += num
			aim += num
		} else if dir == "up" {
			y1 -= num
			aim -= num
		}
	}

	fmt.Printf("Part1: %d\n", x1 * y1)
	fmt.Printf("Part2: %d\n", x2 * y2)

	file.Close()
}