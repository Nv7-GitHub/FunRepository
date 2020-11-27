package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
	"time"
)

var lists = [][3]byte{[3]byte{[]byte("r")[0], []byte("i")[0], []byte("n")[0]}, [3]byte{[]byte("d")[0], []byte("s")[0], []byte("b")[0]}, [3]byte{[]byte("u")[0], []byte("t")[0], []byte("k")[0]}, [3]byte{[]byte("m")[0], []byte("c")[0], []byte("a")[0]}}
var startTime time.Time

func handle(err error) {
	if err != nil {
		panic(err)
	}
}

func isIn(list [3]byte, character byte) bool {
	return (list[0] == character) || (list[1] == character) || (list[2] == character)
}

func start() {
	startTime = time.Now()
}

func end(text string) {
	fmt.Println("Calculated", text, "in", time.Now().Sub(startTime))
}

func isValid(word string) bool {
	if len(word) < 2 {
		return false
	}
	itemNum := -1
	for i, val := range lists {
		if isIn(val, word[0]) {
			itemNum = i
		}
	}
	if itemNum == -1 {
		return false
	}
	for _, char := range []byte(word[1:]) {
		oldItemNum := itemNum
		itemNum = -1
		for i, val := range lists {
			if isIn(val, char) {
				itemNum = i
			}
		}
		if itemNum == -1 {
			return false
		}
		if itemNum == oldItemNum {
			return false
		}
	}
	return true
}

func main() {
	bigStart := time.Now()

	start()
	wordFile, err := ioutil.ReadFile("words.txt")
	handle(err)
	words := strings.Split(string(wordFile), "\n")
	valid := make([]string, len(words))
	count := 0

	for _, val := range words {
		if isValid(val) {
			valid[count] = val
			count++
		}
	}

	valid = valid[:count]
	end("valid")

	start()
	categories := make(map[byte][]string, 0)
	for _, val := range valid {
		_, exists := categories[val[0]]
		if exists {
			categories[val[0]] = append(categories[val[0]], val)
		} else {
			categories[val[0]] = []string{val}
		}
	}
	end("categories")

	start()
	sort.Slice(valid, func(i, j int) bool { return len(valid[i]) > len(valid[j]) })
	for key := range categories {
		sort.Slice(categories[key], func(i, j int) bool { return len(categories[key][i]) > len(categories[key][j]) })
	}
	end("sizes")

	start()
	combos := make([][]string, 0)
	for _, val := range valid {
		combo := []string{val}
		end := val[len(val)-1]
		_, exists := categories[end]
		if !exists {
			continue
		}
		for i := 0; i < 5; i++ {
			index := 0
			_, contains := categories[categories[end][index][len(categories[end][index])-1]]
			success := true
			for !contains {
				index++
				if index > len(categories[end]) {
					success = false
					break
				}
				_, contains = categories[categories[end][index][len(categories[end][index])-1]]
			}
			if success {
				combo = append(combo, categories[end][index])
				end = categories[end][index][len(categories[end][index])-1]
			}
		}
		cleaned := make(map[string]bool, 0)
		for _, val := range combo {
			cleaned[val] = true
		}
		finalCombo := make([]string, 0)
		for key := range cleaned {
			finalCombo = append(finalCombo, key)
		}
		combos = append(combos, finalCombo)
	}
	end("paths")

	start()
	scores := make([]Scored, 0)
	for _, val := range combos {
		set := make(map[byte]bool, 0)
		for _, word := range val {
			for _, char := range []byte(word) {
				set[char] = true
			}
		}
		score := Scored{
			Val:   val,
			Score: 0,
		}
		for range set {
			score.Score++
		}
		scores = append(scores, score)
	}
	sort.Slice(scores, func(i, j int) bool { return scores[i].Score > scores[j].Score })
	end("scoring")

	fmt.Println("Finished everything in", time.Now().Sub(bigStart))

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
