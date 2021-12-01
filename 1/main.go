package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	one()
	two()
}

func one() {
	if file, err := os.Open("./input.txt"); err == nil {
		defer file.Close()

		prev := 999999999999
		count := 0
		scanner := bufio.NewScanner(file)
		for scanner.Scan() {
			value, _ := strconv.Atoi(scanner.Text())
			if value > prev {
				count++
			}
			prev = value
		}
		fmt.Printf("%d\n", count)
	} else {
		fmt.Println(err)
	}
}

func two() {
	if file, err := os.Open("./input.txt"); err == nil {
		defer file.Close()

		windowSums := [3]int{0, 0, 0}
		lastWindowSum := 9999999999999
		increaseCount := 0
		loopCounter := 0
		scanner := bufio.NewScanner(file)
		for scanner.Scan() {
			value, _ := strconv.Atoi(scanner.Text())
			windowSums[loopCounter%3] = value
			currentWindowSum := windowSums[0] + windowSums[1] + windowSums[2]
			if currentWindowSum > lastWindowSum {
				increaseCount++
			}
			loopCounter++
			if loopCounter > 2 { // start at +2
				lastWindowSum = currentWindowSum
			}
		}
		fmt.Printf("%d\n", increaseCount)
	} else {
		fmt.Println(err)
	}
}
