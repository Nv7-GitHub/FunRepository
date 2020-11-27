package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
	"time"
)

// The letters on the side
var lists = [][3]byte{[3]byte{[]byte("r")[0], []byte("i")[0], []byte("n")[0]}, [3]byte{[]byte("d")[0], []byte("s")[0], []byte("b")[0]}, [3]byte{[]byte("u")[0], []byte("t")[0], []byte("k")[0]}, [3]byte{[]byte("m")[0], []byte("c")[0], []byte("a")[0]}}
var startTime time.Time

// Handles errors
func handle(err error) {
	if err != nil {
		panic(err)
	}
}

// Checks if a letter is in one of the sides, given the side and the letter
func isIn(list [3]byte, character byte) bool {
	return (list[0] == character) || (list[1] == character) || (list[2] == character)
}

// Timing
func start() {
	startTime = time.Now()
}

// Timing
func end(text string) {
	fmt.Println("Calculated", text, "in", time.Now().Sub(startTime))
}

// Checks if a word is valid with the current sides
func isValid(word string) bool {
	// Check word length
	if len(word) < 2 {
		return false
	}
	// itemNum is which side of the letters it is on
	itemNum := -1
	// Loop through arrays to find which side
	for i, val := range lists {
		if isIn(val, word[0]) {
			itemNum = i
		}
	}
	// The letter isn't on any side, its not valid
	if itemNum == -1 {
		return false
	}
	// Now that first letter is found, repeat for every other letter
	for _, char := range []byte(word[1:]) {
		// Keep track of previous itemNum so that you know if it is on same side
		oldItemNum := itemNum
		itemNum = -1
		// Same loop to check which side
		for i, val := range lists {
			if isIn(val, char) {
				itemNum = i
			}
		}
		// Letter isn't on a side, invalid
		if itemNum == -1 {
			return false
		}
		// Letter is on same side as previous letter, invalid
		if itemNum == oldItemNum {
			return false
		}
	}
	// Passed all the tests, is valid
	return true
}

func main() {
	// Timing
	bigStart := time.Now()

	start()
	// Read words from words.txt
	wordFile, err := ioutil.ReadFile("words.txt")
	handle(err)
	// Split by enter to get array of words
	words := strings.Split(string(wordFile), "\n")
	// Make valid an array with the same length as words, cut of the empty stuff later
	// Did this instead of making with len 0 and appending for performance
	valid := make([]string, len(words))
	// Keep track of the amount found so that we can cut off end
	count := 0

	// Loop through words, check if valid
	for _, val := range words {
		if isValid(val) {
			// If valid make the next empty item in valid the word
			valid[count] = val
			// Increase count
			count++
		}
	}

	// Cut off end
	valid = valid[:count]
	end("valid")

	start()
	// Categories is a dictionary where a key of a letter returns all the words with that letter
	categories := make(map[byte][]string, 0)
	// Loop through valid and add to categories
	for _, val := range valid {
		// Check if array initialized
		_, exists := categories[val[0]]
		if exists {
			// If array initialized, append to that category
			categories[val[0]] = append(categories[val[0]], val)
		} else {
			// Otherwise make a new array
			categories[val[0]] = []string{val}
		}
	}
	end("categories")

	start()
	// Sort valid list by the length of the word
	sort.Slice(valid, func(i, j int) bool { return len(valid[i]) > len(valid[j]) })
	// Sort all category lists by the length of the word
	for key := range categories {
		sort.Slice(categories[key], func(i, j int) bool { return len(categories[key][i]) > len(categories[key][j]) })
	}
	end("sizes")

	start()
	// Combos is an array of possible paths you can take
	combos := make([][]string, 0)
	for _, val := range valid {
		// The combo starts with the first word
		combo := []string{val}
		end := val[len(val)-1]
		// Are there any words that start with the last letter of this word? If not, go to the next one
		_, exists := categories[end]
		if !exists {
			continue
		}
		// Fill out remaining 5 in combo
		for i := 0; i < 5; i++ {
			// index is the index of the word we are testing
			index := 0
			// Check if there are any words that start with the last letter of the word we are testing
			_, contains := categories[categories[end][index][len(categories[end][index])-1]]
			// If success is false, this whole combo is impossible, just skip it
			success := true
			// Loop until we find a word that has words that start with the last letter of it
			for !contains {
				index++
				// If we loop through all the words and don't find anything, just skip the combo
				if index > len(categories[end]) {
					success = false
					break
				}
				// Update contains so that this isn't a forever loop
				_, contains = categories[categories[end][index][len(categories[end][index])-1]]
			}
			// If this word is good, add it to the combos, move onto next word
			if success {
				combo = append(combo, categories[end][index])
				end = categories[end][index][len(categories[end][index])-1]
			}
		}
		// A lot of the time we would get things like "statistician statistician statistician", because it solved the puzzle in 3 words and had 3 more places to just repeat stuff, so remove duplicates
		// Cleaned is a map where all the keys are words
		cleaned := make(map[string]bool, 0)
		// For every word, make it a key in the map
		for _, val := range combo {
			cleaned[val] = true
		}
		// finalCombo is going to be an array with all the keys of the map
		finalCombo := make([]string, 0)
		for key := range cleaned {
			// For every key in the map, add it to finalCombo
			finalCombo = append(finalCombo, key)
		}
		// Add finalCombo to the combos
		combos = append(combos, finalCombo)
	}
	end("paths")

	start()
	// In this section, we calculate the amount of unique letters each word has, and sort by it. Then we find all of them that are solved
	// We do this instead of just looping through and finding solved in case that it wasn't solved and we want ideas
	// scores is an array of structs, each struct contains the combo and the amount of unique letters
	scores := make([]Scored, 0)
	// Add all the combos to scores
	for _, val := range combos {
		// set is a map where all the keys are a letter
		set := make(map[byte]bool, 0)
		// loop through all letters, add to map
		for _, word := range val {
			for _, char := range []byte(word) {
				set[char] = true
			}
		}
		// initialize struct
		score := Scored{
			Val:   val,
			Score: 0,
		}
		// Loop through keys and increase score by 1 for every key
		for range set {
			score.Score++
		}
		// Add to scores
		scores = append(scores, score)
	}
	// Sort
	sort.Slice(scores, func(i, j int) bool { return scores[i].Score > scores[j].Score })
	end("scoring")

	// Total time
	fmt.Println("Finished everything in", time.Now().Sub(bigStart))

	// Loop through output, print everything that is solved
	for _, val := range scores {
		if val.Score >= 12 {
			fmt.Println(val.Val)
		}
	}
}

// Scored has the data on a value and what it scored
type Scored struct {
	Val   []string
	Score int
}
