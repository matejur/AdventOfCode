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

var corruptedScores = map[rune]int {
	')': 3,
	']': 57,
	'}': 1197,
	'>': 25137,
}

var incompleteScores = map[rune]int {
	'(': 1,
	'[': 2,
	'{': 3,
	'<': 4,
}

func findFirstIllegalOrFix(line string) (int, int) {
	var stack []rune

	for _, brace := range line {
		switch brace {
		case '(', '[', '{', '<':
			stack = append(stack, brace)
		case ')', ']', '}', '>':
			n := len(stack) - 1
			top := stack[n]

			if top == '(' && brace != ')' ||
			   top == '[' && brace != ']' ||
			   top == '{' && brace != '}' ||
			   top == '<' && brace != '>' {
				return corruptedScores[brace], 0
			}
			stack = stack[:n]
		}
	}

	value := 0
	for len(stack) > 0 {
		n := len(stack) - 1
		brace := stack[n]

		value *= 5
		value += incompleteScores[brace]

		stack = stack[:n]
	}

	return 0, value
}

func main() {
	file, err := os.Open("in10.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	sum1 := 0
	var scores []int
	for scanner.Scan() {
		line := scanner.Text()
		val1, val2 := findFirstIllegalOrFix(line)
		sum1 += val1
		if (val1 == 0) {
			scores = append(scores, val2)
		}
	}

	sort.Ints(scores)
	fmt.Printf("Part 1: %d\n", sum1)
	fmt.Printf("Part 2: %d\n", scores[len(scores) / 2])
}
