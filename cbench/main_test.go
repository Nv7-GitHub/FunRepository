package main

import (
	"math/rand"
	"testing"
	"time"

	"github.com/Nv7-Github/FunRepository/cbench/cbench"
	"github.com/Nv7-Github/FunRepository/cbench/gobench"
)

const maxSize = 1000

func makeTestData(length int) ([]int, []int) {
	as := make([]int, length)
	bs := make([]int, length)
	rand.Seed(time.Now().UnixNano())
	for i := range as {
		as[i] = rand.Intn(maxSize)
		bs[i] = rand.Intn(maxSize)
	}
	return as, bs
}

func BenchmarkGobenchPow(t *testing.B) {
	as, bs := makeTestData(t.N)
	t.ResetTimer()

	for i, val := range as {
		gobench.Pow(val, bs[i])
	}
}

func BenchmarkCbenchPow(t *testing.B) {
	as, bs := makeTestData(t.N)
	t.ResetTimer()

	for i, val := range as {
		cbench.Pow(val, bs[i])
	}
}
