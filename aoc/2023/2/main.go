package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	RED   = 12
	GREEN = 13
	BLUE  = 14
)

// Game 6: 1 red, 12 blue; 20 blue, 3 green, 2 red; 4 red, 15 blue
func isValidGame(text string) bool {
	games := strings.Split(text, ";")

	for _, game := range games {
		colorCounts := strings.Split(strings.TrimSpace(game), ",")

		for _, colorCount := range colorCounts {
			parts := strings.Split(strings.TrimSpace(colorCount), " ")

			color := parts[1]
			count := parts[0]

			num, err := strconv.Atoi(count)
			if err != nil {
				fmt.Println(err.Error())
				os.Exit(1)
			}

			if (color == "red" && num > RED) ||
				(color == "green" && num > GREEN) ||
				(color == "blue" && num > BLUE) {
				return false
			}
		}
	}

  return true
}

func main() {
	f, err := os.Open("input")
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	var ans int
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		line := sc.Text()
		parts := strings.Split(line, ":")
		if isValidGame(parts[1]) {
			gameId, err := strconv.Atoi(strings.Split(parts[0], " ")[1])
			if err != nil {
				fmt.Println(err.Error())
				os.Exit(1)
			}
			ans += gameId
		}
	}

	fmt.Println(ans)
}
