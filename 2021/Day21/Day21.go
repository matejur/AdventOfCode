package main

import (
	"fmt"
)

func roll(die int) (int, int) {
	sum := 0
	for i := 0; i < 3; i++ {
		sum += die
		die += 1
		if die == 101 {
			die = 1
		}
	}
	return sum, die
}

var memo [2][11][11][22][22][2]int
var repeats = [7]int{1, 3, 6, 7, 6, 3, 1}

func play(p1, p2, score1, score2, player int) (int, int) {
	if score1 >= 21 {
		return 1, 0
	}

	if score2 >= 21 {
		return 0, 1
	}

	if memo[player][p1][p2][score1][score2][0] > 0 || memo[player][p1][p2][score1][score2][1] > 0 {
		return memo[player][p1][p2][score1][score2][0], memo[player][p1][p2][score1][score2][1]
	}

	player1 := 0
	player2 := 0
	var one, two int
	for i, rep := range repeats {
		die := i + 3
		if player == 0 {
			newPos := (p1+die-1)%10 + 1
			one, two = play(newPos, p2, score1+newPos, score2, 1)
		} else {
			newPos := (p2+die-1)%10 + 1
			one, two = play(p1, newPos, score1, score2+newPos, 0)
		}
		player1 += one * rep
		player2 += two * rep
	}

	memo[player][p1][p2][score1][score2] = [2]int{player1, player2}
	return player1, player2
}

func main() {
	start1 := 8
	start2 := 6

	pos1 := start1
	pos2 := start2

	score1 := 0
	score2 := 0

	numRolls := 0
	die := 1
	p1 := 0
	p2 := 0

	for {
		p1, die = roll(die)
		numRolls += 3
		pos1 = (pos1+p1-1)%10 + 1
		score1 += pos1

		if score1 >= 1000 {
			break
		}

		p2, die = roll(die)
		numRolls += 3
		pos2 = (pos2+p2-1)%10 + 1
		score2 += pos2

		if score2 >= 1000 {
			break
		}
	}

	var part1 int
	if score1 < score2 {
		part1 = score1 * numRolls
	} else {
		part1 = score2 * numRolls
	}

	fmt.Printf("Part 1: %d\n", part1)

	player1, player2 := play(start1, start2, 0, 0, 0)
	if player1 > player2 {
		fmt.Printf("Part 2: %d\n", player1)
	} else {
		fmt.Printf("Part 2: %d\n", player2)
	}
}
