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

func getFishAtDay(ages [9]int, days int) int {
	for day := 0; day < days; day++ {
		newFishToday := ages[0]

		for i := 0; i < 8; i++ {
			ages[i] = ages[i+1]
		}
		ages[8] = newFishToday
		ages[6] += newFishToday
	}

	sum := 0
	for i := range(ages) {
		sum += ages[i]
	}
	return sum
}

func main() {
	file, err := os.Open("in6.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	var ages [9]int

	for _, num := range strings.Split(line, ",") {
		i, err := strconv.Atoi(num)
		check(err)
		ages[i]++
	}
	
	fmt.Printf("Part 1: %d\n", getFishAtDay(ages, 80))
	fmt.Printf("Part 2: %d\n", getFishAtDay(ages, 256))
}