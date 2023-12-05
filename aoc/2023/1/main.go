package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var /* const */ re = regexp.MustCompile(`\D+`)

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
		numStr := re.ReplaceAllString(line, "")
		num, err := strconv.Atoi(string(numStr[0]) + string(numStr[len(numStr)-1]))
		if err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}
		ans += num
	}

	fmt.Println(ans)
}
