package main

import (
	"container/heap"
	"fmt"
	"hash/fnv"
)

type PriorityQueue []*burrow

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].cost < pq[j].cost
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*burrow)
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

type burrow struct {
	cost     int
	index    int
	hallway  []rune
	rooms    [4]*room
	previous *burrow
}

func (b burrow) hash() uint32 {
	h := fnv.New32a()
	h.Write([]byte(string(b.hallway)))

	for _, r := range b.rooms {
		h.Write([]byte(string(r.amphipods)))
	}

	return h.Sum32()
}

func (s burrow) copy(hallway []rune, cost int) burrow {
	new := burrow{
		hallway: hallway,
		cost:    s.cost + cost,
	}

	for i, r := range s.rooms {
		new.rooms[i] = r.copy()
	}

	return new
}

type room struct {
	num       int
	amphipods []rune
	amphType  rune
	index     int
	lock      int
	maxNum    int
}

func (r room) copy() *room {
	new := room{
		num:      r.num,
		amphType: r.amphType,
		index:    r.index,
		lock:     r.lock,
		maxNum:   r.maxNum,
	}

	new.amphipods = make([]rune, 4)
	copy(new.amphipods, r.amphipods)
	return &new
}

func (r *room) take() (rune, int) {
	if r.num < 0 {
		return '0', -1
	}

	if r.num == r.lock {
		return '0', -1
	}

	r.num--
	amphipod := r.amphipods[r.num]
	r.amphipods[r.num] = '.'
	return amphipod, costs[amphipod] * (r.maxNum - r.num)
}

func (r *room) insert(amphipod rune) int {
	if r.amphType != amphipod {
		return -1
	}

	if r.num == 4 {
		return -1
	}

	for i := 0; i < r.num; i++ {
		if r.amphipods[i] != r.amphType {
			return -1
		}
	}

	r.amphipods[r.num] = amphipod
	r.num++
	r.lock = r.num
	return costs[amphipod] * (r.maxNum - r.num + 1)
}

func (r room) print() {
	for i := 0; i < 4; i++ {
		if i < r.num {
			fmt.Print(string(r.amphipods[i]))
		} else {
			fmt.Print(".")
		}
	}
	fmt.Println()
	for i := 0; i < 4; i++ {
		if i < r.lock {
			fmt.Print("X")
		} else {
			fmt.Print(".")
		}
	}
	fmt.Println()
}

func (b burrow) finished() bool {
	for _, r := range b.rooms {
		if r.num != r.maxNum {
			return false
		}
		for _, a := range r.amphipods[:r.num] {
			if a != r.amphType {
				return false
			}
		}
	}
	return true
}

func (b burrow) printPath() {
	if b.previous != nil {
		b.previous.printPath()
	}
	b.print()
}

func (b burrow) print() {
	fmt.Println()
	fmt.Println(b.cost)
	fmt.Println(string(b.hallway))
	for i := 3; i > -1; i-- {
		fmt.Print("  ")
		for _, r := range b.rooms {
			fmt.Print(string(r.amphipods[i]))
			fmt.Print(" ")
		}
		fmt.Println()
	}
}

var costs = map[rune]int{
	'A': 1,
	'B': 10,
	'C': 100,
	'D': 1000,
}

var homes = map[rune]int{
	'A': 2,
	'B': 4,
	'C': 6,
	'D': 8,
}

var hashes = map[uint32]int{}

var canStay = []bool{true, true, false, true, false, true, false, true, false, true, true}

func main() {

	fmt.Println("Part 1: 18282 (solved by hand)")
	room1 := room{num: 4, maxNum: 4, amphipods: []rune{'D', 'D', 'D', 'C'}, amphType: 'A', index: 2}
	room2 := room{num: 4, maxNum: 4, amphipods: []rune{'D', 'B', 'C', 'C'}, amphType: 'B', index: 4}
	room3 := room{num: 4, maxNum: 4, amphipods: []rune{'B', 'A', 'B', 'A'}, amphType: 'C', index: 6}
	room4 := room{num: 4, maxNum: 4, amphipods: []rune{'A', 'C', 'A', 'B'}, amphType: 'D', index: 8}

	start := burrow{
		cost:    0,
		index:   0,
		rooms:   [4]*room{&room1, &room2, &room3, &room4},
		hallway: []rune{'.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
	}

	queue := make(PriorityQueue, 1)
	queue[0] = &start

	for len(queue) > 0 {
		curr := heap.Pop(&queue).(*burrow)

		// curr.print()
		// fmt.Scanln()

		if curr.finished() {
			fmt.Printf("Part 2: %d\n", curr.cost)
			break
		}

		// za vsako sobo probamo spravit enga vn
		//fmt.Scanln()
		for rId, r := range curr.rooms {
			amphipod, cost := r.copy().take()

			if cost > -1 {
				// probimo jt desno
				moved := 0
				for i := r.index + 1; i < len(curr.hallway); i++ {
					moved++
					if curr.hallway[i] == '.' && canStay[i] {
						newHallway := make([]rune, 11)
						copy(newHallway, curr.hallway)
						newHallway[i] = amphipod

						new := curr.copy(newHallway, cost+moved*costs[amphipod])
						new.rooms[rId].take()
						new.previous = curr

						cost, ok := hashes[new.hash()]
						if !ok || cost > new.cost {
							hashes[new.hash()] = new.cost
							heap.Push(&queue, &new)
						}

					} else if curr.hallway[i] != '.' {
						break
					}
				}

				// probimo jt levo
				moved = 0
				for i := r.index - 1; i > -1; i-- {
					moved++
					if curr.hallway[i] == '.' && canStay[i] {
						newHallway := make([]rune, 11)
						copy(newHallway, curr.hallway)
						newHallway[i] = amphipod

						new := curr.copy(newHallway, cost+moved*costs[amphipod])
						new.rooms[rId].take()
						new.previous = curr

						cost, ok := hashes[new.hash()]
						if !ok || cost > new.cost {
							hashes[new.hash()] = new.cost
							heap.Push(&queue, &new)
						}
					} else if curr.hallway[i] != '.' {
						break
					}
				}
			}

			// iz hallwaya notr
			for i := 0; i < len(curr.hallway); i++ {
				if curr.hallway[i] != '.' {
					amph := curr.hallway[i]
					home := homes[amph]

					step := 1
					if home < i {
						step = -1
					}

					moved := 1
					j := i + step
					for ; home != j; j += step {
						if curr.hallway[j] != '.' {
							j -= step
							break
						}
						moved++
					}

					if j == home {
						newHallway := make([]rune, 11)
						copy(newHallway, curr.hallway)
						newHallway[i] = '.'

						new := curr.copy(newHallway, moved*costs[amph])
						insertCost := new.rooms[home/2-1].insert(amph)
						if insertCost > 0 {
							new.cost += insertCost
							new.previous = curr
							cost, ok := hashes[new.hash()]
							if !ok || cost > new.cost {
								hashes[new.hash()] = new.cost
								heap.Push(&queue, &new)
							}
						}
					}
				}
			}

			// pa se v kontra smeri (lahk bi naredu posebi funkcijo, but we dont have time for that)
			for i := len(curr.hallway) - 1; i > -1; i-- {
				if curr.hallway[i] != '.' {
					amph := curr.hallway[i]
					home := homes[amph]

					step := 1
					if home < i {
						step = -1
					}

					moved := 1
					j := i + step
					for ; home != j; j += step {
						if curr.hallway[j] != '.' {
							j -= step
							break
						}
						moved++
					}

					if j == home {
						newHallway := make([]rune, 11)
						copy(newHallway, curr.hallway)
						newHallway[i] = '.'

						new := curr.copy(newHallway, moved*costs[amph])
						insertCost := new.rooms[home/2-1].insert(amph)
						if insertCost > 0 {
							new.cost += insertCost
							new.previous = curr
							cost, ok := hashes[new.hash()]
							if !ok || cost > new.cost {
								hashes[new.hash()] = new.cost
								heap.Push(&queue, &new)
							}
						}
					}
				}
			}
		}
	}
}
