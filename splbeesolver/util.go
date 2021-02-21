package main

import (
	"strconv"
	"strings"

	"github.com/Nv7-Github/chromego"
)

var textBox chromego.Element

func submitWord(text string) {
	// Make sure its empty
	for true {
		txt, err := textBox.Text()
		handle(err)
		if len(txt) == 0 || strings.Contains(txt, " ") {
			break
		}
		el := btnMap["ent"]
		err = el.Click()
		handle(err)
		isOver()
	}

	var el chromego.Element
	var err error
	for _, letter := range text {
		isOver()
		el = btnMap[string(letter)]
		err = el.Click()
		handle(err)
	}
	el = btnMap["ent"]
	err = el.Click()
	handle(err)
}

func amountFound() int {
	length, err := drv.FindElemsByCSSSelector(".sb-anagram")
	handle(err)
	return len(length) / 2
}

func permutations(testStr string) []string {
	join := func(ins []rune, c rune) (result []string) {
		for i := 0; i <= len(ins); i++ {
			result = append(result, string(ins[:i])+string(c)+string(ins[i:]))
		}
		return
	}
	var n func(testStr []rune, p []string) []string
	n = func(testStr []rune, p []string) []string {
		if len(testStr) == 0 {
			return p
		}
		result := []string{}
		for _, e := range p {
			result = append(result, join([]rune(e), testStr[0])...)
		}
		return n(testStr[1:], result)
	}

	output := []rune(testStr)
	return n(output[1:], []string{string(output[0])})
}

func isOver() {
	vals, err := drv.FindElemsByCSSSelector(".pz-modal__title")
	handle(err)
	if len(vals) > 0 {
		txt, err := vals[0].Text()
		handle(err)
		if txt != "You’re pretty good at this!" {
			return
		}

		countPoints()

		cleanup()
		startDriver()
		initHTML()

		found = 0
	}
}

func countPoints() {
	sc, err := drv.FindByCSSSelector(".sb-progress-value")
	handle(err)
	scr, err := sc.Text()
	handle(err)
	scre, err := strconv.Atoi(scr)
	handle(err)
	score += scre
}
