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

type Set []int

// very inefficient
func (s *Set) insert(x int) {
	for _, v := range *s {
		if v == x {
			return
		}
	}
	*s = append(*s, x)
}

func (s Set) index(x int) int {
	for i, v := range s {
		if v == x {
			return i
		}
	}
	return -1
}

func main() {
	file, err := os.Open("in22.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var lines []string

	var part1 [101][101][101]bool
	first := true

	xs := Set{}
	ys := Set{}
	zs := Set{}
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
		var action string
		var x1, x2, y1, y2, z1, z2 int
		fmt.Sscanf(line, "%s x=%d..%d,y=%d..%d,z=%d..%d", &action, &x1, &x2, &y1, &y2, &z1, &z2)

		if x1 < 50 && x1 > -50 {
			for x := x1; x <= x2; x++ {
				for y := y1; y <= y2; y++ {
					for z := z1; z <= z2; z++ {
						part1[x+50][y+50][z+50] = action == "on"
					}
				}
			}
		} else if first {
			first = false
			sum := 0

			for i := range part1 {
				for j := range part1[i] {
					for k := range part1[i][j] {
						if part1[i][j][k] {
							sum++
						}
					}
				}
			}

			fmt.Printf("Part 1: %d\n", sum)
		}

		x2++
		y2++
		z2++
		xs.insert(x1)
		xs.insert(x2)
		ys.insert(y1)
		ys.insert(y2)
		zs.insert(z1)
		zs.insert(z2)
	}

	sort.Ints(xs)
	sort.Ints(ys)
	sort.Ints(zs)

	reactor := make([][][]bool, len(xs))
	for i := range reactor {
		reactor[i] = make([][]bool, len(ys))
		for j := range reactor[i] {
			reactor[i][j] = make([]bool, len(zs))
		}
	}

	for _, line := range lines {
		var action string
		var x1, x2, y1, y2, z1, z2 int
		fmt.Sscanf(line, "%s x=%d..%d,y=%d..%d,z=%d..%d", &action, &x1, &x2, &y1, &y2, &z1, &z2)

		x2++
		y2++
		z2++

		x1i := xs.index(x1)
		x2i := xs.index(x2)
		y1i := ys.index(y1)
		y2i := ys.index(y2)
		z1i := zs.index(z1)
		z2i := zs.index(z2)

		for x := x1i; x < x2i; x++ {
			for y := y1i; y < y2i; y++ {
				for z := z1i; z < z2i; z++ {
					reactor[x][y][z] = action == "on"
				}
			}
		}
	}

	sum := 0
	for x := 0; x < len(xs); x++ {
		for y := 0; y < len(ys); y++ {
			for z := 0; z < len(zs); z++ {
				if reactor[x][y][z] {
					sum += (xs[x+1] - xs[x]) * (ys[y+1] - ys[y]) * (zs[z+1] - zs[z])
				}
			}
		}
	}

	fmt.Printf("Part 2: %d\n", sum)
}
