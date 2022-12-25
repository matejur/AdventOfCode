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

type instruction struct {
	operator  string
	firstReg  int
	secondReg int
	num       int
}

var instructins []instruction

func compute(input []int) (int, bool) {
	var register [4]int
	index := 0

	for _, ins := range instructins {
		switch ins.operator {
		case "inp":
			register[ins.firstReg] = input[index]
			index++
		case "add":
			if ins.secondReg == -1 {
				register[ins.firstReg] = register[ins.firstReg] + ins.num
			} else {
				register[ins.firstReg] = register[ins.firstReg] + register[ins.secondReg]
			}
		case "mul":
			if ins.secondReg == -1 {
				register[ins.firstReg] = register[ins.firstReg] * ins.num
			} else {
				register[ins.firstReg] = register[ins.firstReg] * register[ins.secondReg]
			}
		case "div":
			var a, b int
			a = register[ins.firstReg]
			if ins.secondReg == -1 {
				b = ins.num
			} else {
				b = register[ins.secondReg]
			}

			if b == 0 {
				return 0, false
			}
			register[ins.firstReg] = a / b
		case "mod":
			var a, b int
			a = register[ins.firstReg]
			if ins.secondReg == -1 {
				b = ins.num
			} else {
				b = register[ins.secondReg]
			}

			if a < 0 || b <= 0 {
				return 0, false
			}
			register[ins.firstReg] = a % b
		case "eql":
			equal := false
			if ins.secondReg == -1 {
				equal = register[ins.firstReg] == ins.num
			} else {
				equal = register[ins.firstReg] == register[ins.secondReg]
			}
			if equal {
				register[ins.firstReg] = 1
			} else {
				register[ins.firstReg] = 0
			}
		}
	}
	return register[3], true
}

var numToReg = map[string]int{
	"w": 0,
	"x": 1,
	"y": 2,
	"z": 3,
}

func main() {
	file, err := os.Open("in24.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " ")
		if line[0] == "" {
			break
		}
		if line[0] == "inp" {
			ins := instruction{"inp", numToReg[line[1]], -1, -1}
			instructins = append(instructins, ins)
		} else {
			var ins instruction
			if i, err := strconv.Atoi(line[2]); err == nil {
				ins = instruction{line[0], numToReg[line[1]], -1, i}
			} else {
				ins = instruction{line[0], numToReg[line[1]], numToReg[line[2]], -1}
			}
			instructins = append(instructins, ins)
		}
	}

	input := []int{9, 9, 4, 2, 9, 7, 9, 5, 9, 9, 3, 9, 2, 9}
	compute(input)

	input = []int{1, 8, 1, 1, 3, 1, 8, 1, 5, 7, 1, 6, 1, 1}
	compute(input)

	fmt.Println("Part 1: 99429795993929")
	fmt.Println("Part 2: 18113181571611")
}
