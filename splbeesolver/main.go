package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/Nv7-Github/chromego"
)

var btnMap = make(map[string]chromego.Element)
var drv chromego.Driver
var score int
var found int

func handle(err error) {
	if err != nil {
		panic(err)
	}
}

func startDriver() {
	var err error
	drv, err = chromego.CreateDriver("/usr/local/bin/chromedriver", 3000)
	handle(err)
	drv.Navigate("https://www.nytimes.com/puzzles/spelling-bee")
}

func main() {
	fmt.Println("Starting...")
	startDriver()

	fmt.Println("Initializing...")
	setup()

	fmt.Println("Calculating...")
	findValid()

	fmt.Println("Testing...")
	found = amountFound()
	answers := make([]string, 0)
	for _, val := range valid {
		submitWord(val)
		isOver()
		newFound := amountFound()
		if newFound > found {
			word := strings.ToLower(val)
			fmt.Println(word)
			answers = append(answers, word)
			found = newFound
		}
	}
	countPoints()
	score++ // Misses a point
	fmt.Printf("Final Score: %d\n", score)

	// Done!
	cleanup()

	// Save results to file
	final := strings.Join(answers, "\n")
	final = fmt.Sprintf("Final Score: %d\n", score) + final
	file, err := os.OpenFile("final.txt", os.O_CREATE|os.O_WRONLY, os.ModePerm)
	handle(err)
	err = file.Truncate(0)
	handle(err)
	_, err = file.Write([]byte(final))
	handle(err)
	err = file.Close()
	handle(err)
}
