package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type spot struct {
	x, y, risk, index int
}

type PriorityQueue []*spot

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].risk < pq[j].risk
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*spot)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func findBestPath(cavern [][]byte, distances [][]int) int {
	endY := len(cavern) - 1
	endX := len(cavern[0]) - 1

	queue := make(PriorityQueue, 1)
	queue[0] = &spot{x: 0, y: 0, risk: 0, index: 0}

	for {
		curr := heap.Pop(&queue).(*spot)

		x := curr.x
		y := curr.y

		if x == endX && y == endY {
			return curr.risk
		}

		directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

		for _, dir := range directions {
			newX := x + dir[0]
			newY := y + dir[1]

			if newY > -1 && newY < len(cavern) && newX > -1 && newX < len(cavern[newY]) {
				risk := curr.risk + int(cavern[newY][newX])

				if distances[newY][newX] > risk {
					distances[newY][newX] = risk
					heap.Push(&queue, &spot{x: newX, y: newY, risk: risk})
				}
			}
		}
	}
}

// this function is a slight mess
func expandCavern(in [][]byte) ([][]byte, [][]int) {
	var out [][]byte
	var distances [][]int

	// expand to the right
	for i := range in {
		var row []byte
		var dist []int
		for j := 0; j < 5; j++ {
			for k := range in[i] {
				val := in[i][k] + byte(j)

				if val > 9 {
					val -= 9
				}
				
				row = append(row, val)
				dist = append(dist, 9999999999)
			}
		}
		out = append(out, row)
		distances = append(distances, dist)
	}

	// expand down
	for i := 1; i < 5; i++ {
		for j := range in {
			var row []byte
			var dist []int
			for k := range out[j] {
				val := out[j][k] + byte(i)

				if val > 9 {
					val -= 9
				}

				row = append(row, val)
				dist = append(dist, 9999999999)
			}
			out = append(out, row)
			distances = append(distances, dist)
		}
	}
	return out, distances
}

func main() {
	file, err := os.Open("in15.txt")
	check(err)

	scanner := bufio.NewScanner(file)

	var cavern [][]byte
	var distances [][]int

	for scanner.Scan() {
		line := scanner.Text()

		var row []byte
		var dist []int
		for _, risk := range line {
			row = append(row, byte(risk-'0'))
			dist = append(dist, 9999999999)
		}
		cavern = append(cavern, row)
		distances = append(distances, dist)
	}

	cavern2, distances2 := expandCavern(cavern)
	fmt.Printf("Part 1: %d\n", findBestPath(cavern, distances))
	fmt.Printf("Part 2: %d\n", findBestPath(cavern2, distances2))
}
