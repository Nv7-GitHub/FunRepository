package cbench

// #include "cbench.h"
import "C"

func Pow(a, b int) int {
	return int(C.intPow(C.int(a), C.int(b)))
}
