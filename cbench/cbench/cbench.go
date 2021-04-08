package cbench

// #include "cbench.h"
import "C"

func Add(a, b int) int {
	return int(C.add(C.int(a), C.int(b)))
}

func Multiply(a, b int) int {
	return int(C.multiply(C.int(a), C.int(b)))
}
