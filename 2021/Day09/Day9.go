package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func getBasinSize(heatmap [][]rune, i int, j int) int {
	if heatmap[i][j] == 9 {
		return 0
	}

	heatmap[i][j] = 9

	sum := getBasinSize(heatmap, i-1, j)
	sum += getBasinSize(heatmap, i+1, j)
	sum += getBasinSize(heatmap, i, j-1)
	sum += getBasinSize(heatmap, i, j+1)
	return sum + 1
}

func localMinima(heatmap [][]rune) (int, int) {
	sum := 0
	var basinSizes []int
	for i := 1; i < len(heatmap)-1; i++ {
		for j := 1; j < len(heatmap[i])-1; j++ {
			up := heatmap[i-1][j]
			down := heatmap[i+1][j]
			left := heatmap[i][j-1]
			right := heatmap[i][j+1]
			center := heatmap[i][j]

			if center < up && center < down && center < left && center < right {
				sum += int(center) + 1

				basinSize := getBasinSize(heatmap, i, j)
				basinSizes = append(basinSizes, basinSize)
			}
		}
	}
	
	sort.Slice(basinSizes, func(i,j int) bool {
		return basinSizes[i] > basinSizes[j]
	})
	basin := basinSizes[0] * basinSizes[1] * basinSizes[2] 
	return sum, basin
}

func main() {
	file, err := os.Open("in9.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var heatmap [][]rune
	var padding []rune
	for scanner.Scan() {
		line := scanner.Text()

		if len(padding) == 0 {
			for i := 0; i < len(line)+2; i++ {
				padding = append(padding, 9)
			}
			heatmap = append(heatmap, padding)
		}

		var numbers []rune
		numbers = append(numbers, 9)
		for _, char := range line {
			numbers = append(numbers, char-'0')
		}
		numbers = append(numbers, 9)
		heatmap = append(heatmap, numbers)
	}
	heatmap = append(heatmap, padding)

	part1, part2 := localMinima(heatmap)
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
