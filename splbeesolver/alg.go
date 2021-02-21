package main

import (
	"io/ioutil"
	"sort"
	"strings"
)

var words []string
var valid []string
var center string
var letters map[string]empty = make(map[string]empty)

func loadWords() {
	// Load Words
	dat, err := ioutil.ReadFile("words.txt")
	handle(err)

	wrds := strings.Split(string(dat), "\n")
	words = make([]string, len(wrds))
	for i, val := range wrds {
		words[i] = strings.ToUpper(val)
	}
}

func findValid() {
	valid = make([]string, len(words))
	found := 0
	for _, word := range words {
		if isValid(word) {
			valid[found] = word
			found++
		}
	}
	valid = valid[:found]

	sort.Slice(valid, func(i, j int) bool {
		return len(valid[i]) > len(valid[j])
	})
}

func isValid(word string) bool {
	if len(word) < 4 {
		return false
	}
	hasFoundCenter := false
	for _, letter := range word {
		_, exists := letters[string(letter)]
		if !exists {
			return false
		}
		if string(letter) == center {
			hasFoundCenter = true
		}
	}
	return hasFoundCenter
}
