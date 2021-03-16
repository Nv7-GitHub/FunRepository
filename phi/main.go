package main

// https://www.youtube.com/watch?v=KZ241NpMarI - Explanation that I followed

import (
	"fmt"
	"strings"
	"time"

	"math/big"
)

// Calculates 14450 digits
const digits = 240 * 60

func main() {
	start := time.Now()
	prec := uint(digits / 3 * 10)
	phi := new(big.Float).SetPrec(prec).SetFloat64(2)
	one := new(big.Float).SetPrec(prec).SetFloat64(1)
	oldphi := new(big.Float).SetPrec(prec)
	fmt.Println("Initialized in", time.Since(start))

	start = time.Now()
	for {
		phi.Quo(one, phi)
		phi.Add(phi, one)

		if oldphi.Cmp(phi) == 0 {
			break
		}
		oldphi.Copy(phi)
	}
	fmt.Println("Calculated in", time.Since(start))

	start = time.Now()
	corr := false
	digis := fmt.Sprintf("%v", phi)
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

	fmt.Println()
	fmt.Println()
	fmt.Printf("%s\nCalculated %d digits of pi!\n", digis, len(digis)-2)
	if corr {
		fmt.Println("Verified correct!")
		return
	}
	fmt.Println("Incorrect :(")
}
