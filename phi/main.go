package main

// https://www.youtube.com/watch?v=KZ241NpMarI - Explanation that I followed

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"time"

	"math/big"
)

// Calculates 7224 digits
const digits = 1000000000 + (ignoreLast + 1)
const ignoreLast = 2

func main() {
	start := time.Now()
	prec := uint(digits / 3 * 10)
	phi := new(big.Float).SetPrec(prec)
	five := new(big.Float).SetPrec(prec).SetFloat64(5)
	two := new(big.Float).SetPrec(prec).SetFloat64(2)
	half := new(big.Float).SetPrec(prec).SetFloat64(0.5)
	fmt.Println("Initialized in", time.Since(start))

	start = time.Now()
	phi.Sqrt(five)
	phi.Quo(phi, two)
	phi.Add(phi, half)
	fmt.Println("Calculated in", time.Since(start))

	start = time.Now()
	digis := phi.Text('g', digits)
	digis = digis[:len(digis)-ignoreLast]
	fmt.Println("Converted digits in", time.Since(start))

	start = time.Now()
	err := ioutil.WriteFile("digits.txt", []byte(digis), os.ModePerm)
	if err != nil {
		panic(err)
	}
	fmt.Println("Saved digits in", time.Since(start))

	start = time.Now()
	corr := false
	if len(digis) > len(correct) {
		if strings.Compare(digis[:len(correct)], correct) == 0 {
			corr = true
		}
	} else {
		if strings.Compare(digis, correct[:len(digis)]) == 0 {
			corr = true
		}
	}
	fmt.Println("Verified in", time.Since(start))

	fmt.Printf("Calculated %d digits of pi!\n", len(digis)-2)
	if corr {
		fmt.Println("Verified correct!")
		return
	}
	fmt.Println("Incorrect :(")
}
