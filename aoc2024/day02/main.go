package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func abs(n int) int {
	if n < 0 {
		return n * -1
	}

	return n
}

func sameSign(a int, b int) bool {
	return (a >= 0 && b >= 0) || (a <= 0 && b <= 0)
}

func safeDelta(delta int) bool {
	absDelta := abs(delta)
	return (absDelta >= 1 && absDelta <= 3)
}

func getDeltas(numbers []string) ([]int, []int) {
	deltas := make([]int, len(numbers)-1)
	prevNumber := 0
	negDeltas := make([]int, 0, len(deltas))
	posDeltas := make([]int, 0, len(deltas))
	unsafeDeltas := make([]int, 0, len(deltas))

	for i, num := range numbers {
		parsedNumber, _ := strconv.Atoi(num)
		if i == 0 {
			prevNumber = parsedNumber
			continue
		}

		deltas[i-1] = parsedNumber - prevNumber
		prevNumber = parsedNumber

		if !safeDelta(deltas[i-1]) {
			unsafeDeltas = append(unsafeDeltas, i-1)
		} else if deltas[i-1] < 0 {
			negDeltas = append(negDeltas, i-1)
		} else {
			posDeltas = append(posDeltas, i-1)
		}
	}

	if len(posDeltas) > len(negDeltas) {
		return deltas, append(unsafeDeltas, negDeltas...)
	} else {
		return deltas, append(unsafeDeltas, posDeltas...)
	}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	safe_reports := 0
	safe_reports_2 := 0

	for scanner.Scan() {
		numbers := strings.Fields(scanner.Text())
		deltas, wrongDeltas := getDeltas(numbers)

		// Part 1
		if len(wrongDeltas) == 0 {
			safe_reports += 1
			safe_reports_2 += 1
		}

		// Part 2
		if len(wrongDeltas) == 2 {
			// Are the two faulty deltas next to each other?
			neighbors := abs(wrongDeltas[0]-wrongDeltas[1]) == 1

			// I need a "sane" delta to compare the sign with
			// If they are next to each other then this will be a different one
			// Don't need to care about when they aren't as that will make it unsafe either way
			saneDelta := deltas[(wrongDeltas[0]+2)%len(deltas)]

			// We add the two deltas together to make a new one, which is the delta that would exist if we remove a level
			newDelta := deltas[wrongDeltas[0]] + deltas[wrongDeltas[1]]

			// So, if they are next to each other, the new delta is safe and has the same sign as the others
			// Then it can be reported safe
			if neighbors && safeDelta(newDelta) && sameSign(newDelta, saneDelta) {
				safe_reports_2 += 1
			}
		} else if len(wrongDeltas) == 1 {
			// If there's only one wrong delta and it's either the first or last
			// We can simply remove the first or last level and report this as safe
			if wrongDeltas[0] == 0 || wrongDeltas[0] == len(deltas)-1 {
				safe_reports_2 += 1
				continue
			}

			// Add the wrong delta to previous and to the next and check if either produce a new safe delta
			wrongDelta := deltas[wrongDeltas[0]]
			prevDelta := deltas[wrongDeltas[0]-1]
			nextDelta := deltas[wrongDeltas[0]+1]

			delta1 := prevDelta + wrongDelta
			delta2 := wrongDelta + nextDelta
			if (sameSign(delta1, deltas[0]) && safeDelta(delta1)) || (sameSign(delta2, deltas[0]) && safeDelta(delta2)) {
				safe_reports_2 += 1
			}
		}

	}

	fmt.Println("Part 1:", safe_reports)
	fmt.Println("Part 2:", safe_reports_2)
}
