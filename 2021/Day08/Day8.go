package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sort"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func getA(one string, seven string) rune {
	for _, s := range seven {
		isA := true
		for _, o := range one {
			if s == o {
				isA = false
				break
			}
		}

		if isA {
			return s
		}
	}
	return '0'
}

func getG(four string, eight string, preslikava map[rune]rune) rune {
	for _, e := range eight {
		isG := true
		for _, f := range four {
			if e == f {
				isG = false
				break
			}
		}

		if isG {
			if _, ok := preslikava[e]; !ok {
				return e
			}
		}
	}
	return '0'
}

type ByRune []rune

func (r ByRune) Len() int {
	return len(r)
}

func (r ByRune) Swap(i int, j int) {
	r[i], r[j] = r[j], r[i]
}

func (r ByRune) Less(i int, j int) bool {
	return r[i] < r[j]
}

var segmentsToDigit = map[string]int {
	"abcefg": 0,
	"cf": 1,
	"acdeg": 2,
	"acdfg": 3,
	"bcdf": 4,
	"abdfg": 5,
	"abdefg": 6,
	"acf": 7,
	"abcdefg": 8,
	"abcdfg": 9,
}

func sortRunes(in string) string {
	var r ByRune = []rune(in);
	sort.Sort(r)
	return string(r)
}

func main() {
	file, err := os.Open("in8.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var patterns [][]string
	var output [][]string

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " | ")
		patterns = append(patterns, strings.Split(line[0], " "))
		output = append(output, strings.Split(line[1], " "))
	}

	uniqueNumbers := 0
	for i := range output {
		for _, num := range output[i] {
			seg := len(num)
			if seg == 2 || seg == 4 || seg == 3 || seg == 7 {
				uniqueNumbers++
			}
		}
	}

	sum := 0
	for i := range patterns {
		var segments [7]int
		var one string
		var four string
		var seven string
		var eight string
		for _, signal := range patterns[i] {
			for _, segment := range signal {
				segments[int(segment-'a')]++
			}

			if len(signal) == 2 {
				one = signal
			} else if len(signal) == 4 {
				four = signal
			} else if len(signal) == 3 {
				seven = signal
			} else if len(signal) == 7 {
				eight = signal
			}
		}

		preslikava := make(map[rune]rune)

		// dobim B, E, F trivilano iz števila ponovitev
		for i := range segments {
			rep := segments[i]

			if rep == 4 {
				preslikava[rune(i + 'a')] = 'e'
			} else if rep == 6 {
				preslikava[rune(i + 'a')] = 'b'
			} else if rep == 9 {
				preslikava[rune(i + 'a')] = 'f'
			}
		}
		
		// na podlagi 1 in 7 lahko določimo a
		preslikava[getA(one, seven)] = 'a'

		// na pdolagi 4, 8 in dosedanje preslikave lahko določimo g
		preslikava[getG(four, eight, preslikava)] = 'g'

		for i := range segments {
			rep := segments[i]

			if rep == 7 {
				// sedemkrat se pojavljata le segmenta a in d
				if _, ok := preslikava[rune(i + 'a')]; !ok {
					preslikava[rune(i + 'a')] = 'd'
				} 
			} else if rep == 8 {
				// osemkrat se pojavljata le segmenta a in c
				if _, ok := preslikava[rune(i + 'a')]; !ok {
					preslikava[rune(i + 'a')] = 'c'
				}
			}
		}

		mapDigit := func(r rune) rune {
			return preslikava[r]
		}

		number := 0
		multi := 1000
		for _, segments := range output[i] {
			remapped := strings.Map(mapDigit, segments)
			sorted := sortRunes(remapped)
			digit := segmentsToDigit[sorted]
			number += digit * multi
			multi /= 10
		}
		sum += number
	}

	fmt.Printf("Part 1: %d\n", uniqueNumbers)
	fmt.Printf("Part 2: %d\n", sum)
}
