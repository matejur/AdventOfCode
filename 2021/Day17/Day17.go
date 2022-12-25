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

type probe struct {
	x, y, vx, vy, max int
}

func (p *probe) step() {
	p.x += p.vx
	p.y += p.vy

	if p.y > p.max {
		p.max = p.y
	}

	p.vy--
	if p.vx < 0 {
		p.vx++
	} else if p.vx > 0 {
		p.vx--
	}
}

func main() {
	file, err := os.Open("in17.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()

	line := scanner.Text()

	x1, x2, y1, y2 := 0, 0, 0, 0
	fmt.Sscanf(line, "target area: x=%d..%d, y=%d..%d", &x1, &x2, &y1, &y2)

	part1 := 0
	part2 := 0
	for vx := 0; vx < 500; vx++ {
		for vy := -500; vy < 500; vy++ {

			probe := probe{x: 0, y: 0, vx: vx, vy: vy}
			for probe.x <= x2 && probe.y >= y1 {
				probe.step()
				px := probe.x
				py := probe.y

				if px >= x1 && px <= x2 && py >= y1 && py <= y2 {
					part2++
					if probe.max > part1 {
						part1 = probe.max
					}
					break
				}
			}
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
