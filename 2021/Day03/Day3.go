package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.Open("in3.txt")
	check(err);

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	file.Close()

	n := len(lines[0])
	ones := make([]int, n)
	zeros := make([]int, n)

	for _, line := range lines {
		for i, bit := range line {
			if bit == '0' {
				zeros[i] += 1
			} else {
				ones[i] += 1
			}
		}
	}

	gama := 0
	epsilon := 0

	for i := 0; i < n; i++ {
		if ones[i] > zeros[i] {
			gama += int(math.Pow(2, float64(n-i-1)))
		} else {
			epsilon += int(math.Pow(2, float64(n-i-1)))
		}
	}

	fmt.Printf("Part 1: %d\n", gama*epsilon)

	oxygen := eliminate(lines, ones, zeros, '1', '0')
	scrubber := eliminate(lines, ones, zeros, '0', '1')

	fmt.Printf("Part 2: %d\n", oxygen*scrubber)
	file.Close()
}

func remove(s []string, i int) []string {
    s[i] = s[len(s)-1]
    return s[:len(s)-1]
}

func eliminate(arrIn []string, onesIn []int, zerosIn []int, a byte, b byte) int64 {
	i := 0
	arr := make([]string, len(arrIn))
	ones := make([]int, len(onesIn))
	zeros := make([]int, len(zerosIn))

	copy(arr, arrIn)
	copy(ones, onesIn)
	copy(zeros, zerosIn)
	for len(arr) > 1 {
		var bit byte
		if ones[i] >= zeros[i] {
			bit = a
		} else {
			bit = b
		}

		for j := len(arr) - 1; j > -1; j-- {
			line := arr[j]
			if (line[i] != bit) {
				arr = remove(arr, j)

				for k, bin := range line {
					if (bin == '1') {
						ones[k]--;
					} else {
						zeros[k]--;
					}
				}
			}
		}
		i++
	}

	num, err := strconv.ParseInt(arr[0], 2, 0)
	check(err)
	return num
}
