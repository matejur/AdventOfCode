package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type node struct {
	depth       int
	value       int
	right, left *node
}

func parseLine(line string, index int) (*node, int) {
	if line[index] == '[' {
		left, nextIndex := parseLine(line, index+1)
		right, nextIndex := parseLine(line, nextIndex+1)
		return &node{right: right, left: left}, nextIndex + 1
	}

	return &node{value: int(line[index] - '0')}, index + 1
}

func leftToRight(node *node, list *[]*node, depth int) {
	if node.left != nil {
		leftToRight(node.left, list, depth+1)
	}

	node.depth = depth
	*list = append(*list, node)

	if node.right != nil {
		leftToRight(node.right, list, depth+1)
	}
}

func explode(root *node) bool {
	var arr []*node
	leftToRight(root, &arr, 0)
	for i := 0; i < len(arr); i++ {
		pair := arr[i]

		if pair.depth == 4 && pair.left != nil && pair.right != nil {
			for j := i - 2; j > -1; j-- {
				if arr[j].left == nil && arr[j].right == nil {
					arr[j].value += pair.left.value
					break
				}
			}

			for j := i + 2; j < len(arr); j++ {
				if arr[j].left == nil && arr[j].right == nil {
					arr[j].value += pair.right.value
					break
				}
			}

			pair.left = nil
			pair.right = nil
			pair.value = 0
			return true
		}
	}
	return false
}

func split(root *node) bool {
	var arr []*node
	leftToRight(root, &arr, 0)
	for i := 0; i < len(arr); i++ {
		pair := arr[i]
		if pair.value > 9 {
			left := node{value: pair.value / 2}

			var right node
			if pair.value%2 == 0 {
				right = node{value: pair.value / 2, depth: pair.depth + 1}
			} else {
				right = node{value: pair.value/2 + 1, depth: pair.depth + 1}
			}

			pair.left = &left
			pair.right = &right
			pair.value = 0
			return true
		}
	}
	return false
}

func reduce(root *node) {
	exploded := false
	splited := true

	for splited {
		exploded = explode(root)

		if exploded {
			continue
		}

		splited = split(root)
	}
}

func magnitude(node *node) int {
	if node.left == nil && node.right == nil {
		return node.value
	}

	sum := 0
	sum += 3 * magnitude(node.left)
	sum += 2 * magnitude(node.right)
	return sum
}

func main() {
	file, err := os.Open("in18.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var prevRoot *node
	var lines []string
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)

		root, _ := parseLine(line, 0)

		if prevRoot != nil {
			root = &node{left: prevRoot, right: root}
			reduce(root)
		}

		prevRoot = root
	}
	fmt.Printf("Part 1: %d\n", magnitude(prevRoot))

	maxMag := 0

	for i := 0; i < len(lines); i++ {
		for j := i + 1; j < len(lines); j++ {
			var sb1 strings.Builder
			sb1.WriteRune('[')
			sb1.WriteString(lines[i])
			sb1.WriteRune(',')
			sb1.WriteString(lines[j])
			sb1.WriteRune(']')

			var sb2 strings.Builder
			sb2.WriteRune('[')
			sb2.WriteString(lines[j])
			sb2.WriteRune(',')
			sb2.WriteString(lines[i])
			sb2.WriteRune(']')

			root1, _ := parseLine(sb1.String(), 0)
			root2, _ := parseLine(sb2.String(), 0)

			reduce(root1)
			reduce(root2)

			mag1 := magnitude(root1)
			mag2 := magnitude(root2)

			if mag1 > maxMag {
				maxMag = mag1
			}

			if mag2 > maxMag {
				maxMag = mag2
			}
		}
	}

	fmt.Printf("Part 2: %d\n", maxMag)
}
