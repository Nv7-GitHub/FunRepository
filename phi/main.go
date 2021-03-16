package main

// https://www.youtube.com/watch?v=KZ241NpMarI - Explanation that I followed

import (
	"fmt"
	"strings"

	"math/big"
)

// +200 = 60 more digits
// +10 = 3 digits
const digits = 240 * 60

func main() {
	prec := uint(digits / 3 * 10)
	phi := new(big.Float).SetPrec(prec).SetFloat64(2)
	one := new(big.Float).SetPrec(prec).SetFloat64(1)
	oldphi := new(big.Float).SetPrec(prec)
	for i := 0; i < 1000000; i++ {
		phi.Quo(one, phi)
		phi.Add(phi, one)

		if oldphi.Cmp(phi) == 0 {
			break
		}
		oldphi.Copy(phi)
	}
	digis := fmt.Sprintf("%v", phi)
	fmt.Printf("%s\nCalculated %d digits of pi!\n", digis, len(digis)-2)
	if strings.Compare(digis[2:], correct[:len(digis)-2]) == 0 {
		fmt.Println("Verified correct!")
	}
}
