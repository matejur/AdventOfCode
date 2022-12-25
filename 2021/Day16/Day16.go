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

var hex2bin = map[rune]string{
	'0': "0000",
	'1': "0001",
	'2': "0010",
	'3': "0011",
	'4': "0100",
	'5': "0101",
	'6': "0110",
	'7': "0111",
	'8': "1000",
	'9': "1001",
	'A': "1010",
	'B': "1011",
	'C': "1100",
	'D': "1101",
	'E': "1110",
	'F': "1111",
}

func toDec(in string) int {
	val, err := strconv.ParseInt(in, 2, 64)
	check(err)
	return int(val)
}

func literalValue(packet string) int {
	var packetValue strings.Builder

	for i := index + 6; ; i += 5 {
		packetValue.WriteString(packet[i+1 : i+5])

		index = i
		if packet[i] == '0' {
			break
		}
	}
	index += 5
	return toDec(packetValue.String())
}

func sum(arr []int) int {
	sum := 0
	for _, i := range arr {
		sum += i
	}
	return sum
}

func multiply(arr []int) int {
	product := 1
	for _, i := range arr {
		product *= i
	}
	return product
}

func minimum(arr []int) int {
	min := 9999999999
	for _, i := range arr {
		if i < min {
			min = i
		}
	}
	return min
}

func maximum(arr []int) int {
	max := 0
	for _, i := range arr {
		if i > max {
			max = i
		}
	}
	return max
}

var versionSum = 0
var index = 0
var packet string

func processPackets() int {
	version := toDec(packet[index : index+3])
	versionSum += version
	typeID := toDec(packet[index+3 : index+6])

	if typeID == 4 {
		return literalValue(packet)
	} else {
		mode := packet[index+6]

		var values []int
		if mode == '0' {
			len := toDec(packet[index+7 : index+22])
			index += 22
			curr := index
			for index < curr+len-1 {
				values = append(values, processPackets())
			}
		} else {
			num := toDec(packet[index+7 : index+18])
			index += 18

			for i := 0; i < num; i++ {
				values = append(values, processPackets())
			}
		}

		temp := false
		switch typeID {
		case 0:
			return sum(values)
		case 1:
			return multiply(values)
		case 2:
			return minimum(values)
		case 3:
			return maximum(values)
		case 5:
			temp = values[0] > values[1]
		case 6:
			temp = values[0] < values[1]
		case 7:
			temp = values[0] == values[1]
		}

		if temp {
			return 1
		}
	}

	return 0
}

func main() {
	file, err := os.Open("in16.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()

	var builder strings.Builder
	for _, c := range line {
		builder.WriteString(hex2bin[c])
	}
	packet = builder.String()

	value := processPackets()
	fmt.Printf("Part 1: %d\n", versionSum)
	fmt.Printf("Part 2: %d\n", value)
}
