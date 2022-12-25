package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type vector struct {
	x, y, z int
}

func (v vector) rotate(i int) vector {
	switch i {
	case 0:
		return vector{v.x, v.y, v.z}
	case 1:
		return vector{v.x, -v.z, v.y}
	case 2:
		return vector{v.x, -v.y, -v.z}
	case 3:
		return vector{v.x, v.z, -v.y}
	case 4:
		return vector{-v.x, -v.y, v.z}
	case 5:
		return vector{-v.x, -v.z, -v.y}
	case 6:
		return vector{-v.x, v.y, -v.z}
	case 7:
		return vector{-v.x, v.z, v.y}
	case 8:
		return vector{v.y, v.x, -v.z}
	case 9:
		return vector{v.y, -v.x, v.z}
	case 10:
		return vector{v.y, v.z, v.x}
	case 11:
		return vector{v.y, -v.z, -v.x}
	case 12:
		return vector{-v.y, v.x, v.z}
	case 13:
		return vector{-v.y, -v.x, -v.z}
	case 14:
		return vector{-v.y, -v.z, v.x}
	case 15:
		return vector{-v.y, v.z, -v.x}
	case 16:
		return vector{v.z, v.x, v.y}
	case 17:
		return vector{v.z, -v.x, -v.y}
	case 18:
		return vector{v.z, -v.y, v.x}
	case 19:
		return vector{v.z, v.y, -v.x}
	case 20:
		return vector{-v.z, v.x, -v.y}
	case 21:
		return vector{-v.z, -v.x, v.y}
	case 22:
		return vector{-v.z, v.y, v.x}
	case 23:
		return vector{-v.z, -v.y, -v.x}
	}
	panic("Invalid rotation!")
}

func (v vector) dist(w vector) int {
	return int(math.Abs(float64(v.x-w.x)) + math.Abs(float64(v.y-w.y)) + math.Abs(float64(v.z-w.z)))
}

func (v vector) add(w vector) vector {
	return vector{
		x: v.x + w.x,
		y: v.y + w.y,
		z: v.z + w.z,
	}
}

func (v vector) sub(w vector) vector {
	return vector{
		x: v.x - w.x,
		y: v.y - w.y,
		z: v.z - w.z,
	}
}

func main() {
	file, err := os.Open("in19.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var scanners [][]vector
	var beacons []vector

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" || line[:3] == "---" {
			if len(beacons) > 0 {
				scanners = append(scanners, beacons)
				beacons = []vector{}
			}
			continue
		}

		var x, y, z int
		fmt.Sscanf(line, "%d,%d,%d", &x, &y, &z)
		beacons = append(beacons, vector{x: x, y: y, z: z})
	}

	if len(beacons) > 0 {
		scanners = append(scanners, beacons)
	}

	world := map[vector]bool{}
	locations := []vector{{0,0,0}}

	for _, b := range scanners[0] {
		world[b] = true
	}

	scanners = scanners[1:]

	start:

	for len(scanners) > 0 {
		for i := range scanners {
			for rot := 0; rot < 24; rot++ {

				offsets := map[vector]int{}
				for known := range world {
					for _, b := range scanners[i] {
						offset := known.sub(b.rotate(rot))
						offsets[offset]++

						if offsets[offset] >= 12 {
							locations = append(locations, offset)
			
							for _, b := range scanners[i] {
								worldPos := b.rotate(rot).add(offset)
								world[worldPos] = true
							}
			
							scanners = append(scanners[:i], scanners[i+1:]...)
							continue start
						}
					}
				}
			}
		}
	}

	fmt.Printf("Part 1: %d\n", len(world))

	maxDist := 0
	for i := range locations {
		for j := i + 1; j < len(locations); j++ {
			dist := locations[i].dist(locations[j])
			if dist > maxDist {
				maxDist = dist
			}
		}
	}

	fmt.Printf("Part 2: %d\n", maxDist)
}
