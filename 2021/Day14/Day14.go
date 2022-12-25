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

var insertions = map[rune]map[rune]rune{}

func step(polymer map[string]int) map[string]int {
	var newPolymer = map[string]int{}
	for pair, value := range polymer {
		temp := []rune(pair)
		first := temp[0]
		second := temp[1]
		new := insertions[first][second]

		one := string(first) + string(new)
		two := string(new) + string(second)

		newPolymer[one] += value
		newPolymer[two] += value
	}

	return newPolymer
}

func findDiff(pairs map[string]int, last rune) int {
	var count [25]int
	count[last - 'A']++

	for pair, value := range pairs {
		temp := []rune(pair)
		first := temp[0]

		count[first - 'A'] += value
	}

	min := 999999999999
	max := 0
	for _, c := range count {
		if c < min && c != 0 {
			min = c
		}
		if c > max {
			max = c
		}
	}

	return (max - min)
}

func main() {
	file, err := os.Open("in14.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()

	polymer := []rune(scanner.Text())

	scanner.Scan() // empty line
	for scanner.Scan() {
		line := ([]rune)(scanner.Text())

		a := line[0]
		b := line[1]
		c := line[6]

		if _, ok := insertions[a]; !ok {
			insertions[a] = make(map[rune]rune)
		}
		insertions[a][b] = c

	}

	var polymerPairs = map[string]int{}
	for i := 0; i < len(polymer)-1; i++ {
		polymerPairs[string(polymer[i:i+2])]++
	}

	last := rune(polymer[len(polymer)-1])
	for i := 0; i < 40; i++ {
		polymerPairs = step(polymerPairs)

		if i == 9 {
			fmt.Printf("Part 1: %d\n", findDiff(polymerPairs, last))
		}
	}

	fmt.Printf("Part 2: %d\n", findDiff(polymerPairs, last))
}
